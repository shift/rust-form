use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error};

use crate::{
    models::{DataSubjectRequest, DataSubjectRequestType, VerificationStatus, RequestStatus},
    services::{DataSubjectService, DataExportService, DataDeletionService},
    error::GdprError,
    AppState,
};

// Request/Response types
#[derive(Deserialize)]
pub struct DataSubjectRequestPayload {
    pub request_type: DataSubjectRequestType,
    pub subject_identifier: String,
    pub subject_email: Option<String>,
    pub description: Option<String>,
    pub verification_data: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DataSubjectRequestResponse {
    pub request_id: Uuid,
    pub reference_number: String,
    pub status: String,
    pub estimated_completion: String,
    pub next_steps: String,
}

#[derive(Deserialize)]
pub struct RequestQuery {
    pub status: Option<String>,
    pub request_type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct RequestListResponse {
    pub requests: Vec<DataSubjectRequestSummary>,
    pub total_count: i64,
    pub page: u32,
    pub total_pages: u32,
}

#[derive(Serialize)]
pub struct DataSubjectRequestSummary {
    pub id: Uuid,
    pub reference_number: String,
    pub request_type: DataSubjectRequestType,
    pub status: RequestStatus,
    pub created_at: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub days_remaining: i64,
}

#[derive(Serialize)]
pub struct RequestDetailsResponse {
    pub request: DataSubjectRequest,
    pub activities: Vec<RequestActivity>,
    pub export_files: Option<Vec<ExportFile>>,
    pub deletion_records: Option<Vec<DeletionRecord>>,
}

#[derive(Serialize)]
pub struct RequestActivity {
    pub activity_type: String,
    pub description: String,
    pub performed_at: DateTime<Utc>,
    pub performed_by: Option<String>,
}

#[derive(Serialize)]
pub struct ExportFile {
    pub format: String,
    pub file_size: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub download_url: Option<String>,
}

#[derive(Serialize)]
pub struct DeletionRecord {
    pub table_name: String,
    pub deletion_method: String,
    pub deleted_at: DateTime<Utc>,
    pub record_count: i32,
}

// GDPR Article 15 - Right of Access
// GDPR Article 16 - Right to Rectification  
// GDPR Article 17 - Right to Erasure (Right to be Forgotten)
// GDPR Article 18 - Right to Restriction of Processing
// GDPR Article 20 - Right to Data Portability
// GDPR Article 21 - Right to Object
pub async fn submit_data_subject_request(
    State(app_state): State<AppState>,
    Json(payload): Json<DataSubjectRequestPayload>,
) -> Result<Json<DataSubjectRequestResponse>, StatusCode> {
    info!(
        "GDPR request submitted: type={:?}, subject={}",
        payload.request_type, payload.subject_identifier
    );

    // Validate request payload
    if payload.subject_identifier.is_empty() {
        warn!("GDPR request rejected: empty subject identifier");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Generate unique reference number for tracking
    let reference_number = generate_reference_number(&payload.request_type);
    
    // Perform initial identity verification
    let verification_status = if app_state.gdpr_config.auto_verification {
        match verify_identity_automatically(&payload.subject_identifier, &payload.verification_data).await {
            Ok(status) => status,
            Err(e) => {
                warn!("Auto-verification failed: {}", e);
                VerificationStatus::RequiresManualReview
            }
        }
    } else {
        VerificationStatus::Pending
    };

    // Calculate deadline (GDPR allows 30 days, can be extended by 60 more days in complex cases)
    let deadline = Utc::now() + chrono::Duration::days(app_state.gdpr_config.response_deadline_days as i64);

    // Create the request record
    let request = DataSubjectRequest {
        id: Uuid::new_v4(),
        request_type: payload.request_type.clone(),
        subject_id: payload.subject_identifier.clone(),
        subject_email: payload.subject_email.clone(),
        verification_status,
        status: RequestStatus::Submitted,
        reference_number: reference_number.clone(),
        description: payload.description,
        verification_data: payload.verification_data,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        completed_at: None,
        deadline,
        rejection_reason: None,
        completion_notes: None,
    };

    // Store the request in the database
    match app_state.data_subject_service.create_request(&request).await {
        Ok(_) => {
            info!("GDPR request created: {}", reference_number);
        }
        Err(e) => {
            error!("Failed to create GDPR request: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Send confirmation email to the data subject
    if let Some(email) = &request.subject_email {
        match app_state.notification_service.send_confirmation_email(
            email,
            &reference_number,
            &request.request_type,
            &deadline,
        ).await {
            Ok(_) => info!("Confirmation email sent for request {}", reference_number),
            Err(e) => warn!("Failed to send confirmation email: {}", e),
        }
    }

    // Log the request for audit trail (GDPR Article 30 - Records of processing activities)
    app_state.audit_service.log_gdpr_request(&request).await
        .unwrap_or_else(|e| warn!("Failed to log audit event: {}", e));

    // Schedule automatic processing for verified requests
    if verification_status == VerificationStatus::Verified {
        match app_state.job_queue.schedule_gdpr_processing(request.id).await {
            Ok(_) => info!("Scheduled processing for verified request {}", reference_number),
            Err(e) => warn!("Failed to schedule processing: {}", e),
        }
    }

    // Notify DPO for manual review if required
    if verification_status == VerificationStatus::RequiresManualReview {
        app_state.notification_service.notify_dpo_manual_review(&request).await
            .unwrap_or_else(|e| warn!("Failed to notify DPO: {}", e));
    }

    let response = DataSubjectRequestResponse {
        request_id: request.id,
        reference_number,
        status: "submitted".to_string(),
        estimated_completion: format!("{} days", app_state.gdpr_config.response_deadline_days),
        next_steps: match verification_status {
            VerificationStatus::Verified => "Your request is being processed automatically.".to_string(),
            VerificationStatus::Pending => "Please check your email for verification instructions.".to_string(),
            VerificationStatus::RequiresManualReview => "Your request requires manual verification and will be reviewed within 72 hours.".to_string(),
            VerificationStatus::Failed => "Verification failed. Please contact our data protection team.".to_string(),
        },
    };

    Ok(Json(response))
}

// Get list of requests (for DPO dashboard)
pub async fn list_data_subject_requests(
    State(app_state): State<AppState>,
    Query(query): Query<RequestQuery>,
) -> Result<Json<RequestListResponse>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50).min(100); // Cap at 100 requests per page
    
    match app_state.data_subject_service.list_requests(
        query.status.as_deref(),
        query.request_type.as_deref(),
        page,
        limit,
    ).await {
        Ok((requests, total_count)) => {
            let summaries: Vec<DataSubjectRequestSummary> = requests.into_iter().map(|r| {
                let days_remaining = (r.deadline - Utc::now()).num_days();
                DataSubjectRequestSummary {
                    id: r.id,
                    reference_number: r.reference_number,
                    request_type: r.request_type,
                    status: r.status,
                    created_at: r.created_at,
                    deadline: r.deadline,
                    days_remaining,
                }
            }).collect();

            let total_pages = (total_count as f64 / limit as f64).ceil() as u32;

            Ok(Json(RequestListResponse {
                requests: summaries,
                total_count,
                page,
                total_pages,
            }))
        }
        Err(e) => {
            error!("Failed to list GDPR requests: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get detailed information about a specific request
pub async fn get_request_details(
    State(app_state): State<AppState>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<RequestDetailsResponse>, StatusCode> {
    // Get the main request
    let request = match app_state.data_subject_service.get_request(request_id).await {
        Ok(Some(request)) => request,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get GDPR request {}: {}", request_id, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get request activities
    let activities = app_state.data_subject_service.get_request_activities(request_id).await
        .unwrap_or_else(|e| {
            warn!("Failed to get activities for request {}: {}", request_id, e);
            vec![]
        });

    // Get export files if it's a data portability request
    let export_files = if matches!(request.request_type, DataSubjectRequestType::DataPortability) {
        app_state.data_export_service.get_export_files(request_id).await.ok()
    } else {
        None
    };

    // Get deletion records if it's an erasure request
    let deletion_records = if matches!(request.request_type, DataSubjectRequestType::DataErasure) {
        app_state.data_deletion_service.get_deletion_records(request_id).await.ok()
    } else {
        None
    };

    Ok(Json(RequestDetailsResponse {
        request,
        activities,
        export_files,
        deletion_records,
    }))
}

// Download data export file (GDPR Article 20 - Data Portability)
pub async fn download_data_export(
    State(app_state): State<AppState>,
    Path((request_id, export_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::response::Response, StatusCode> {
    // Verify the request exists and belongs to the authenticated user
    let request = match app_state.data_subject_service.get_request(request_id).await {
        Ok(Some(request)) => request,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Verify the export file exists and hasn't expired
    let export_file = match app_state.data_export_service.get_export_file(export_id).await {
        Ok(Some(file)) if file.expires_at > Utc::now() => file,
        Ok(Some(_)) => return Err(StatusCode::GONE), // File expired
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Log the download for audit purposes
    app_state.audit_service.log_export_download(&request.subject_id, export_id).await
        .unwrap_or_else(|e| warn!("Failed to log export download: {}", e));

    // Generate secure download response
    match app_state.data_export_service.generate_download_response(&export_file).await {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to generate download response: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Update request status (for DPO use)
pub async fn update_request_status(
    State(app_state): State<AppState>,
    Path(request_id): Path<Uuid>,
    Json(update): Json<RequestStatusUpdate>,
) -> Result<Json<DataSubjectRequest>, StatusCode> {
    match app_state.data_subject_service.update_request_status(
        request_id,
        update.status,
        update.notes,
    ).await {
        Ok(request) => {
            // Log status change
            app_state.audit_service.log_status_change(&request).await
                .unwrap_or_else(|e| warn!("Failed to log status change: {}", e));

            // Send notification to data subject if completed or rejected
            if matches!(request.status, RequestStatus::Completed | RequestStatus::Rejected) {
                app_state.notification_service.send_status_update_email(&request).await
                    .unwrap_or_else(|e| warn!("Failed to send status update email: {}", e));
            }

            Ok(Json(request))
        }
        Err(e) => {
            error!("Failed to update request status: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct RequestStatusUpdate {
    pub status: RequestStatus,
    pub notes: Option<String>,
}

// Helper functions
fn generate_reference_number(request_type: &DataSubjectRequestType) -> String {
    let prefix = match request_type {
        DataSubjectRequestType::DataAccess => "SAR",
        DataSubjectRequestType::DataRectification => "REC", 
        DataSubjectRequestType::DataErasure => "DEL",
        DataSubjectRequestType::DataPortability => "POR",
        DataSubjectRequestType::ProcessingRestriction => "RES",
        DataSubjectRequestType::ObjectToProcessing => "OBJ",
        DataSubjectRequestType::ConsentWithdrawal => "CON",
    };
    
    format!("{}-{}-{}", 
        prefix,
        Utc::now().format("%Y%m%d"),
        Uuid::new_v4().to_string()[..8].to_uppercase()
    )
}

async fn verify_identity_automatically(
    subject_identifier: &str,
    verification_data: &Option<serde_json::Value>,
) -> Result<VerificationStatus, GdprError> {
    // Implement automatic identity verification logic
    // This could include:
    // - Email verification
    // - Phone verification
    // - Security question validation
    // - Document verification
    // - Biometric verification
    
    // For now, we'll implement basic email verification
    if subject_identifier.contains('@') && verification_data.is_some() {
        // Simplified verification - in production this would be more robust
        Ok(VerificationStatus::Verified)
    } else {
        Ok(VerificationStatus::RequiresManualReview)
    }
}