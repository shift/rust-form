use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{AppState, AppError};

#[derive(Debug, Deserialize)]
pub struct ComponentSearchQuery {
    pub query: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ComponentCatalogResponse {
    pub components: Vec<ComponentInfo>,
    pub total: u32,
    pub categories: Vec<String>,
    pub authors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ComponentInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub keywords: Vec<String>,
    pub templates: Vec<TemplateInfo>,
    pub uri: String,
    pub cached_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: Option<String>,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidateConfigRequest {
    pub yaml_content: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateConfigResponse {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationWarning {
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateProjectRequest {
    pub config_id: i32,
    pub name: String,
    pub output_format: String, // "zip" | "files"
}

#[derive(Debug, Serialize)]
pub struct GenerateProjectResponse {
    pub project_id: i32,
    pub download_url: Option<String>,
    pub files: Option<Vec<GeneratedFile>>,
}

#[derive(Debug, Serialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

/// Search and discover available components
pub async fn search_components(
    State(_state): State<AppState>,
    Query(query): Query<ComponentSearchQuery>,
) -> Result<Json<ComponentCatalogResponse>, AppError> {
    // TODO: Implement actual component discovery
    // This would integrate with the component system to:
    // 1. Scan local component cache
    // 2. Query component registry
    // 3. Search GitHub/GitLab for components
    // 4. Filter by search criteria
    
    let mock_components = vec![
        ComponentInfo {
            name: "ui-kit".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Basic UI components for Rust-form projects".to_string()),
            author: Some("rust-form".to_string()),
            keywords: vec!["ui".to_string(), "components".to_string()],
            templates: vec![
                TemplateInfo {
                    name: "button.tera".to_string(),
                    description: Some("Reusable button component".to_string()),
                    target: "Frontend".to_string(),
                },
                TemplateInfo {
                    name: "form.tera".to_string(),
                    description: Some("Form wrapper component".to_string()),
                    target: "Frontend".to_string(),
                },
            ],
            uri: "path:../examples/components/ui-kit".to_string(),
            cached_at: Some("2025-09-08T15:57:02Z".to_string()),
        }
    ];
    
    let mut filtered_components = mock_components;
    
    // Apply search filters
    if let Some(search_query) = &query.query {
        filtered_components.retain(|c| {
            c.name.contains(search_query) || 
            c.description.as_ref().map_or(false, |d| d.contains(search_query))
        });
    }
    
    if let Some(author) = &query.author {
        filtered_components.retain(|c| c.author.as_ref() == Some(author));
    }
    
    let categories = vec!["UI".to_string(), "Forms".to_string(), "Layout".to_string()];
    let authors = vec!["rust-form".to_string(), "community".to_string()];
    
    Ok(Json(ComponentCatalogResponse {
        total: filtered_components.len() as u32,
        components: filtered_components,
        categories,
        authors,
    }))
}

/// Validate a YAML configuration
pub async fn validate_config(
    State(_state): State<AppState>,
    Json(request): Json<ValidateConfigRequest>,
) -> Result<Json<ValidateConfigResponse>, AppError> {
    // TODO: Implement actual config validation
    // This would use the Rust-form config parser to:
    // 1. Parse YAML syntax
    // 2. Validate against schema
    // 3. Check field requirements
    // 4. Validate relationships
    // 5. Check component availability
    
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Basic YAML syntax check
    match serde_yaml::from_str::<serde_yaml::Value>(&request.yaml_content) {
        Ok(_) => {
            // YAML is valid, could do more validation here
        }
        Err(e) => {
            errors.push(ValidationError {
                line: None,
                column: None,
                message: format!("YAML syntax error: {}", e),
                severity: "error".to_string(),
            });
        }
    }
    
    // Mock validation warnings
    if request.yaml_content.contains("sqlite") {
        warnings.push(ValidationWarning {
            line: None,
            column: None,
            message: "Consider using PostgreSQL for production environments".to_string(),
        });
    }
    
    Ok(Json(ValidateConfigResponse {
        valid: errors.is_empty(),
        errors,
        warnings,
    }))
}

/// Generate a project from configuration
pub async fn generate_project(
    State(state): State<AppState>,
    Json(request): Json<GenerateProjectRequest>,
) -> Result<Json<GenerateProjectResponse>, AppError> {
    // TODO: Implement actual project generation
    // This would:
    // 1. Load config from database
    // 2. Run rustform generation
    // 3. Package files according to output_format
    // 4. Store project record
    // 5. Return download URL or file list
    
    // For now, return a mock response
    Ok(Json(GenerateProjectResponse {
        project_id: 123,
        download_url: Some("/api/projects/123/download".to_string()),
        files: None,
    }))
}

/// Get component details by name
pub async fn get_component_details(
    State(_state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ComponentInfo>, AppError> {
    // TODO: Implement actual component lookup
    
    // Mock response for ui-kit component
    if name == "ui-kit" {
        Ok(Json(ComponentInfo {
            name: "ui-kit".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Basic UI components for Rust-form projects".to_string()),
            author: Some("rust-form".to_string()),
            keywords: vec!["ui".to_string(), "components".to_string()],
            templates: vec![
                TemplateInfo {
                    name: "button.tera".to_string(),
                    description: Some("Reusable button component".to_string()),
                    target: "Frontend".to_string(),
                },
            ],
            uri: "path:../examples/components/ui-kit".to_string(),
            cached_at: Some("2025-09-08T15:57:02Z".to_string()),
        }))
    } else {
        Err(AppError::NotFound)
    }
}

/// Refresh component cache
pub async fn refresh_components(
    State(_state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // TODO: Implement component cache refresh
    // This would:
    // 1. Scan for new components
    // 2. Update existing component metadata
    // 3. Remove stale entries
    // 4. Return refresh status
    
    let mut status = HashMap::new();
    status.insert("status".to_string(), "success".to_string());
    status.insert("refreshed".to_string(), "1".to_string());
    
    Ok(Json(status))
}