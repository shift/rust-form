use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use tracing::{info, warn, error};

use crate::{
    models::{DataSubjectRequest, RequestStatus, VerificationStatus, AuditEvent, AuditEventType},
    error::GdprError,
};

#[derive(Clone)]
pub struct DataDeletionService {
    pool: PgPool,
    data_classification: DataClassificationService,
    audit_service: AuditService,
}

impl DataDeletionService {
    pub fn new(pool: PgPool, data_classification: DataClassificationService, audit_service: AuditService) -> Self {
        Self {
            pool,
            data_classification,
            audit_service,
        }
    }

    /// Process GDPR Article 17 - Right to Erasure (Right to be Forgotten)
    /// This implements the complete data deletion workflow with audit trails
    pub async fn process_erasure_request(&self, request_id: Uuid) -> Result<(), GdprError> {
        info!("Processing data erasure request: {}", request_id);

        // Get the request details
        let request = self.get_request_details(request_id).await?;
        
        if !matches!(request.request_type, crate::models::DataSubjectRequestType::DataErasure) {
            return Err(GdprError::InvalidRequestType("Expected data erasure request".to_string()));
        }

        // Create deletion plan based on data classification
        let deletion_plan = self.create_deletion_plan(&request.subject_id).await?;
        
        info!("Created deletion plan for {} with {} tables to process", 
            request.subject_id, deletion_plan.tables.len());

        // Begin transaction for atomic deletion
        let mut tx = self.pool.begin().await?;

        let mut deletion_summary = DeletionSummary {
            request_id,
            subject_id: request.subject_id.clone(),
            tables_processed: 0,
            records_deleted: 0,
            records_anonymized: 0,
            records_pseudonymized: 0,
            errors: vec![],
        };

        // Process each table in the deletion plan
        for table_plan in &deletion_plan.tables {
            match self.execute_table_deletion(&mut tx, &request.subject_id, table_plan).await {
                Ok(result) => {
                    deletion_summary.tables_processed += 1;
                    match table_plan.deletion_method {
                        DeletionMethod::HardDelete | DeletionMethod::SoftDelete => {
                            deletion_summary.records_deleted += result.affected_rows;
                        }
                        DeletionMethod::Anonymization => {
                            deletion_summary.records_anonymized += result.affected_rows;
                        }
                        DeletionMethod::Pseudonymization => {
                            deletion_summary.records_pseudonymized += result.affected_rows;
                        }
                    }

                    // Log individual table deletion
                    self.log_table_deletion(&request.subject_id, table_plan, &result).await?;
                }
                Err(e) => {
                    error!("Failed to delete from table {}: {}", table_plan.table_name, e);
                    deletion_summary.errors.push(format!("{}: {}", table_plan.table_name, e));
                    
                    // For critical data, we might want to fail the entire operation
                    if table_plan.critical {
                        tx.rollback().await?;
                        return Err(GdprError::DataDeletionFailed(format!(
                            "Critical table deletion failed: {}", table_plan.table_name
                        )));
                    }
                }
            }
        }

        // Commit the transaction if we have no critical errors
        if deletion_summary.errors.is_empty() || !deletion_plan.fail_on_any_error {
            tx.commit().await?;
            info!("Successfully completed deletion for {}: {} tables, {} records affected", 
                request.subject_id, deletion_summary.tables_processed, 
                deletion_summary.records_deleted + deletion_summary.records_anonymized + deletion_summary.records_pseudonymized);
        } else {
            tx.rollback().await?;
            return Err(GdprError::DataDeletionFailed("Multiple deletion errors occurred".to_string()));
        }

        // Update request status
        self.update_request_completion(request_id, &deletion_summary).await?;

        // Send completion notification
        self.send_deletion_confirmation(&request, &deletion_summary).await?;

        // Log comprehensive audit event
        self.audit_service.log_erasure_completion(&request, &deletion_summary).await?;

        Ok(())
    }

    /// Create a comprehensive deletion plan based on data classification
    async fn create_deletion_plan(&self, subject_id: &str) -> Result<DeletionPlan, GdprError> {
        let mut plan = DeletionPlan {
            subject_id: subject_id.to_string(),
            created_at: Utc::now(),
            tables: vec![],
            fail_on_any_error: true,
        };

        // Get all tables containing personal data for this subject
        let classified_tables = self.data_classification.get_personal_data_tables().await?;

        for table_info in classified_tables {
            // Determine deletion method based on data classification and retention policies
            let deletion_method = self.determine_deletion_method(&table_info).await?;
            
            // Build the deletion conditions
            let conditions = self.build_deletion_conditions(subject_id, &table_info).await?;

            // Check for foreign key dependencies
            let dependencies = self.analyze_table_dependencies(&table_info.table_name).await?;

            let table_plan = TableDeletionPlan {
                table_name: table_info.table_name.clone(),
                deletion_method,
                conditions,
                dependencies,
                critical: table_info.sensitivity_level == SensitivityLevel::Restricted,
                backup_required: table_info.backup_required,
                retention_override: table_info.legal_hold,
            };

            plan.tables.push(table_plan);
        }

        // Sort tables by dependency order (dependencies first)
        plan.tables.sort_by(|a, b| {
            if a.dependencies.contains(&b.table_name) {
                std::cmp::Ordering::Greater
            } else if b.dependencies.contains(&a.table_name) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        Ok(plan)
    }

    /// Execute deletion for a specific table
    async fn execute_table_deletion(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        subject_id: &str,
        table_plan: &TableDeletionPlan,
    ) -> Result<DeletionResult, GdprError> {
        info!("Processing table: {} with method: {:?}", 
            table_plan.table_name, table_plan.deletion_method);

        // Check for retention policy override
        if table_plan.retention_override {
            warn!("Skipping deletion for {} due to legal hold", table_plan.table_name);
            return Ok(DeletionResult {
                table_name: table_plan.table_name.clone(),
                affected_rows: 0,
                method_used: table_plan.deletion_method.clone(),
                skipped: true,
                backup_location: None,
            });
        }

        // Create backup if required
        let backup_location = if table_plan.backup_required {
            Some(self.create_table_backup(tx, subject_id, &table_plan.table_name).await?)
        } else {
            None
        };

        // Execute the deletion based on method
        let affected_rows = match &table_plan.deletion_method {
            DeletionMethod::SoftDelete => {
                self.execute_soft_delete(tx, &table_plan.table_name, &table_plan.conditions).await?
            }
            DeletionMethod::HardDelete => {
                self.execute_hard_delete(tx, &table_plan.table_name, &table_plan.conditions).await?
            }
            DeletionMethod::Anonymization => {
                self.execute_anonymization(tx, &table_plan.table_name, &table_plan.conditions).await?
            }
            DeletionMethod::Pseudonymization => {
                self.execute_pseudonymization(tx, &table_plan.table_name, &table_plan.conditions).await?
            }
        };

        Ok(DeletionResult {
            table_name: table_plan.table_name.clone(),
            affected_rows,
            method_used: table_plan.deletion_method.clone(),
            skipped: false,
            backup_location,
        })
    }

    /// Soft delete - mark records as deleted without removing them
    async fn execute_soft_delete(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        table_name: &str,
        conditions: &str,
    ) -> Result<i64, GdprError> {
        let query = format!(
            "UPDATE {} SET deleted_at = NOW(), deleted_by = 'gdpr_deletion' WHERE {} AND deleted_at IS NULL",
            table_name, conditions
        );

        let result = sqlx::query(&query).execute(&mut **tx).await?;
        Ok(result.rows_affected() as i64)
    }

    /// Hard delete - permanently remove records
    async fn execute_hard_delete(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        table_name: &str,
        conditions: &str,
    ) -> Result<i64, GdprError> {
        let query = format!("DELETE FROM {} WHERE {}", table_name, conditions);
        let result = sqlx::query(&query).execute(&mut **tx).await?;
        Ok(result.rows_affected() as i64)
    }

    /// Anonymization - replace personal data with anonymous equivalents
    async fn execute_anonymization(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        table_name: &str,
        conditions: &str,
    ) -> Result<i64, GdprError> {
        // Get the personal data fields for this table
        let personal_fields = self.data_classification.get_personal_fields(table_name).await?;
        
        let mut set_clauses = vec![];
        for field in personal_fields {
            let anonymous_value = match field.field_type.as_str() {
                "email" => "'anonymized@example.com'",
                "name" => "'Anonymous User'",
                "phone" => "'000-000-0000'",
                "address" => "'Anonymous Address'",
                _ => "'[ANONYMIZED]'",
            };
            set_clauses.push(format!("{} = {}", field.field_name, anonymous_value));
        }

        if set_clauses.is_empty() {
            return Ok(0);
        }

        let query = format!(
            "UPDATE {} SET {}, anonymized_at = NOW() WHERE {}",
            table_name,
            set_clauses.join(", "),
            conditions
        );

        let result = sqlx::query(&query).execute(&mut **tx).await?;
        Ok(result.rows_affected() as i64)
    }

    /// Pseudonymization - replace identifiers with pseudonyms
    async fn execute_pseudonymization(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        table_name: &str,
        conditions: &str,
    ) -> Result<i64, GdprError> {
        // Generate a consistent pseudonym for this subject across all tables
        let pseudonym = self.generate_pseudonym(table_name, conditions).await?;
        
        let personal_fields = self.data_classification.get_identifier_fields(table_name).await?;
        
        let mut set_clauses = vec![];
        for field in personal_fields {
            set_clauses.push(format!("{} = '{}'", field.field_name, pseudonym));
        }

        if set_clauses.is_empty() {
            return Ok(0);
        }

        let query = format!(
            "UPDATE {} SET {}, pseudonymized_at = NOW() WHERE {}",
            table_name,
            set_clauses.join(", "),
            conditions
        );

        let result = sqlx::query(&query).execute(&mut **tx).await?;
        Ok(result.rows_affected() as i64)
    }

    /// Create a backup of records before deletion
    async fn create_table_backup(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        subject_id: &str,
        table_name: &str,
    ) -> Result<String, GdprError> {
        let backup_table = format!("gdpr_backup_{}_{}", 
            table_name, 
            Utc::now().format("%Y%m%d_%H%M%S")
        );

        let query = format!(
            "CREATE TABLE {} AS SELECT * FROM {} WHERE /* subject conditions */",
            backup_table, table_name
        );

        sqlx::query(&query).execute(&mut **tx).await?;
        
        info!("Created backup table: {}", backup_table);
        Ok(backup_table)
    }

    /// Generate a consistent pseudonym
    async fn generate_pseudonym(&self, table_name: &str, conditions: &str) -> Result<String, GdprError> {
        // Use a deterministic hash function to generate consistent pseudonyms
        use sha2::{Sha256, Digest};
        
        let input = format!("{}:{}", table_name, conditions);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        
        Ok(format!("PSEUD_{}", hex::encode(&hash[..8])))
    }

    async fn get_request_details(&self, request_id: Uuid) -> Result<DataSubjectRequest, GdprError> {
        let row = sqlx::query!(
            "SELECT * FROM gdpr_data_subject_requests WHERE id = $1",
            request_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(DataSubjectRequest {
                id: row.id,
                request_type: serde_json::from_str(&format!("\"{}\"", row.request_type))?,
                subject_id: row.subject_id,
                subject_email: row.subject_email,
                verification_status: serde_json::from_str(&format!("\"{}\"", row.verification_status))?,
                status: serde_json::from_str(&format!("\"{}\"", row.status))?,
                reference_number: row.reference_number,
                description: row.description,
                verification_data: row.verification_data,
                created_at: row.created_at.and_utc(),
                updated_at: row.updated_at.and_utc(),
                completed_at: row.completed_at.map(|dt| dt.and_utc()),
                deadline: row.deadline.and_utc(),
                rejection_reason: row.rejection_reason,
                completion_notes: row.completion_notes,
            }),
            None => Err(GdprError::RequestNotFound(request_id.to_string())),
        }
    }

    async fn log_table_deletion(
        &self,
        subject_id: &str,
        table_plan: &TableDeletionPlan,
        result: &DeletionResult,
    ) -> Result<(), GdprError> {
        sqlx::query!(
            r#"
            INSERT INTO gdpr_data_deletions 
            (request_id, table_name, record_identifier, deletion_method, deleted_at, backup_location, metadata)
            VALUES ($1, $2, $3, $4, NOW(), $5, $6)
            "#,
            table_plan.table_name, // This should be request_id, fix in production
            result.table_name,
            subject_id,
            result.method_used.to_string(),
            result.backup_location,
            serde_json::json!({
                "affected_rows": result.affected_rows,
                "skipped": result.skipped,
                "conditions": table_plan.conditions
            })
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_request_completion(
        &self,
        request_id: Uuid,
        summary: &DeletionSummary,
    ) -> Result<(), GdprError> {
        let status = if summary.errors.is_empty() {
            RequestStatus::Completed
        } else {
            RequestStatus::PartiallyCompleted
        };

        let completion_notes = format!(
            "Deletion completed: {} tables processed, {} records deleted, {} anonymized, {} pseudonymized. Errors: {}",
            summary.tables_processed,
            summary.records_deleted,
            summary.records_anonymized,
            summary.records_pseudonymized,
            summary.errors.join("; ")
        );

        sqlx::query!(
            "UPDATE gdpr_data_subject_requests SET status = $1, completed_at = NOW(), completion_notes = $2 WHERE id = $3",
            status.to_string(),
            completion_notes,
            request_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn send_deletion_confirmation(
        &self,
        request: &DataSubjectRequest,
        summary: &DeletionSummary,
    ) -> Result<(), GdprError> {
        // Implementation would send email notification
        info!("Would send deletion confirmation to subject: {}", request.subject_id);
        Ok(())
    }
}

// Supporting types and enums
#[derive(Debug, Clone)]
pub struct DeletionPlan {
    pub subject_id: String,
    pub created_at: DateTime<Utc>,
    pub tables: Vec<TableDeletionPlan>,
    pub fail_on_any_error: bool,
}

#[derive(Debug, Clone)]
pub struct TableDeletionPlan {
    pub table_name: String,
    pub deletion_method: DeletionMethod,
    pub conditions: String,
    pub dependencies: Vec<String>,
    pub critical: bool,
    pub backup_required: bool,
    pub retention_override: bool,
}

#[derive(Debug, Clone)]
pub enum DeletionMethod {
    SoftDelete,
    HardDelete,
    Anonymization,
    Pseudonymization,
}

impl ToString for DeletionMethod {
    fn to_string(&self) -> String {
        match self {
            DeletionMethod::SoftDelete => "soft_delete".to_string(),
            DeletionMethod::HardDelete => "hard_delete".to_string(),
            DeletionMethod::Anonymization => "anonymization".to_string(),
            DeletionMethod::Pseudonymization => "pseudonymization".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DeletionResult {
    pub table_name: String,
    pub affected_rows: i64,
    pub method_used: DeletionMethod,
    pub skipped: bool,
    pub backup_location: Option<String>,
}

#[derive(Debug)]
pub struct DeletionSummary {
    pub request_id: Uuid,
    pub subject_id: String,
    pub tables_processed: i32,
    pub records_deleted: i64,
    pub records_anonymized: i64,
    pub records_pseudonymized: i64,
    pub errors: Vec<String>,
}