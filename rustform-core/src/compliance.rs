use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Compliance framework core types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComplianceConfig {
    pub enabled_frameworks: Vec<ComplianceFramework>,
    pub data_classification: DataClassificationConfig,
    pub retention_policies: Vec<RetentionPolicy>,
    pub consent_management: ConsentManagementConfig,
    pub audit_configuration: AuditConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ComplianceFramework {
    Gdpr(GdprConfig),
    Ccpa(CcpaConfig),
    Hipaa(HipaaConfig),
    Sox(SoxConfig),
    Iso27001(Iso27001Config),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GdprConfig {
    pub dpo_contact: String,
    pub data_controller: String,
    pub lawful_basis_default: LawfulBasis,
    pub cross_border_transfers: bool,
    pub automated_decision_making: bool,
    pub response_deadline_days: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcpaConfig {
    pub business_contact: String,
    pub do_not_sell_enabled: bool,
    pub verification_method: String,
    pub consumer_request_deadline_days: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HipaaConfig {
    pub covered_entity: String,
    pub business_associate_agreements: Vec<String>,
    pub minimum_necessary_enabled: bool,
    pub breach_notification_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoxConfig {
    pub internal_controls_enabled: bool,
    pub change_management_required: bool,
    pub segregation_of_duties: bool,
    pub financial_reporting_controls: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Iso27001Config {
    pub information_security_policy: String,
    pub risk_assessment_frequency: String,
    pub incident_response_enabled: bool,
    pub business_continuity_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LawfulBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterests,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataClassificationConfig {
    pub personal_data_fields: Vec<String>,
    pub sensitive_data_fields: Vec<String>,
    pub financial_data_fields: Vec<String>,
    pub health_data_fields: Vec<String>,
    pub auto_classification: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetentionPolicy {
    pub data_category: String,
    pub retention_period: String,
    pub deletion_method: DeletionMethod,
    pub legal_hold_exemptions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeletionMethod {
    SoftDelete,
    HardDelete,
    Anonymization,
    Pseudonymization,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsentManagementConfig {
    pub granular_consent: bool,
    pub consent_withdrawal_methods: Vec<String>,
    pub age_verification_required: bool,
    pub parental_consent_age: u8,
    pub consent_categories: Vec<ConsentCategory>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsentCategory {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub lawful_basis: LawfulBasis,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditConfig {
    pub log_all_data_access: bool,
    pub log_data_modifications: bool,
    pub log_consent_changes: bool,
    pub retention_period: String,
    pub encryption_required: bool,
}

// Data Subject Rights Request Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataSubjectRequest {
    pub id: Uuid,
    pub request_type: DataSubjectRequestType,
    pub subject_id: String,
    pub verification_status: VerificationStatus,
    pub status: RequestStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deadline: DateTime<Utc>,
    pub reference_number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DataSubjectRequestType {
    DataAccess,            // GDPR Article 15
    DataRectification,     // GDPR Article 16
    DataErasure,           // GDPR Article 17 (Right to be forgotten)
    DataPortability,       // GDPR Article 20
    ProcessingRestriction, // GDPR Article 18
    ObjectToProcessing,    // GDPR Article 21
    ConsentWithdrawal,     // GDPR Article 7(3)
    CcpaDataAccess,        // CCPA Consumer Rights
    CcpaDataDeletion,      // CCPA Consumer Rights
    CcpaOptOutOfSale,      // CCPA Do Not Sell
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    RequiresManualReview,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestStatus {
    Submitted,
    UnderReview,
    InProgress,
    Completed,
    Rejected,
    PartiallyCompleted,
}

// Consent Tracking Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub subject_id: String,
    pub category: String,
    pub granted: bool,
    pub timestamp: DateTime<Utc>,
    pub method: ConsentMethod,
    pub purpose: String,
    pub lawful_basis: LawfulBasis,
    pub withdrawal_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConsentMethod {
    WebForm,
    Email,
    Phone,
    InPerson,
    Api,
    ImpliedConsent,
}

// Audit Trail Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub event_type: AuditEventType,
    pub subject_id: Option<String>,
    pub user_id: Option<String>,
    pub table_name: String,
    pub record_id: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuditEventType {
    DataAccess,
    DataModification,
    DataDeletion,
    ConsentGranted,
    ConsentWithdrawn,
    RightsRequestSubmitted,
    RightsRequestCompleted,
    DataExport,
    BreachDetected,
}

// Data Classification Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataClassification {
    pub field_name: String,
    pub table_name: String,
    pub classification: DataClassificationType,
    pub sensitivity_level: SensitivityLevel,
    pub retention_period: Option<String>,
    pub encryption_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DataClassificationType {
    PersonalData,
    SensitivePersonalData,
    FinancialData,
    HealthData,
    TechnicalData,
    BehavioralData,
    ContactInformation,
    IdentificationData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SensitivityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
}

// Compliance Error Types
#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Invalid request type: {0}")]
    InvalidRequestType(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Data deletion failed: {0}")]
    DataDeletionFailed(String),

    #[error("Consent required for operation: {0}")]
    ConsentRequired(String),

    #[error("Retention period violation: {0}")]
    RetentionViolation(String),

    #[error("Audit log error: {0}")]
    AuditError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
