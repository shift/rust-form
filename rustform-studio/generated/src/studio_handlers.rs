use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{AppState, AppError};
use crate::component_registry::ComponentRegistry;
use rustform_core::config::Config;
use rustform_codegen::CodeGenerator;
use tracing::{error, info, warn};

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
    State(state): State<AppState>,
    Query(query): Query<ComponentSearchQuery>,
) -> Result<Json<ComponentCatalogResponse>, AppError> {
    info!("Searching components with query: {:?}", query);

    // Get or initialize component registry
    let mut registry = state.component_registry.lock().await;
    
    // If registry is empty, try to load from cache or discover
    if registry.components.is_empty() {
        if let Err(e) = registry.load_cache() {
            warn!("Failed to load component cache: {}", e);
        }
        
        // If still empty after cache load, perform discovery
        if registry.components.is_empty() {
            info!("Component registry empty, performing discovery...");
            if let Err(e) = registry.discover_components().await {
                error!("Failed to discover components: {}", e);
                return Err(AppError::Internal("Component discovery failed".to_string()));
            }
        }
    }

    // Search components using real registry
    let components = registry.search_components(
        query.query.as_deref(),
        query.category.as_deref(),
        query.author.as_deref(),
    );

    let categories = registry.get_categories().to_vec();
    let authors = registry.get_authors().to_vec();

    Ok(Json(ComponentCatalogResponse {
        total: components.len() as u32,
        components,
        categories,
        authors,
    }))
}

/// Validate a YAML configuration
pub async fn validate_config(
    State(state): State<AppState>,
    Json(request): Json<ValidateConfigRequest>,
) -> Result<Json<ValidateConfigResponse>, AppError> {
    info!("Validating YAML configuration...");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse and validate using rustform-core
    match Config::from_yaml(&request.yaml_content) {
        Ok(config) => {
            info!("YAML configuration is valid");

            // Check for potential issues and generate warnings
            
            // Database warnings
            if let Some(ref db_config) = config.database {
                if db_config.db_type == "sqlite" {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: "SQLite is not recommended for production environments. Consider PostgreSQL or MySQL.".to_string(),
                    });
                }
                
                if db_config.url_env.is_none() && db_config.url.is_none() {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: "Database URL not specified. Using default SQLite.".to_string(),
                    });
                }
            }

            // Model validation warnings
            for (model_name, model) in &config.models {
                let mut has_primary_key = false;
                let mut has_timestamps = false;

                for (field_name, field) in &model.fields {
                    if field.primary_key.unwrap_or(false) {
                        has_primary_key = true;
                    }
                    
                    if field_name == "created_at" || field_name == "updated_at" {
                        has_timestamps = true;
                    }
                }

                if !has_primary_key {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: format!("Model '{}' does not have a primary key field. Consider adding an 'id' field.", model_name),
                    });
                }

                if !has_timestamps {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: format!("Model '{}' does not have timestamp fields. Consider adding 'created_at' and 'updated_at'.", model_name),
                    });
                }
            }

            // Frontend configuration warnings
            if let Some(ref frontend_config) = config.frontend {
                if frontend_config.target == "vue" || frontend_config.target == "svelte" {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: format!("{} frontend support is experimental. React is recommended for production.", frontend_config.target),
                    });
                }
            }

            // Server configuration warnings
            if let Some(ref server_config) = config.server {
                if server_config.port == 3000 || server_config.port == 8000 {
                    warnings.push(ValidationWarning {
                        line: None,
                        column: None,
                        message: "Using common development port. Consider using a different port for production.".to_string(),
                    });
                }
            }
        }
        Err(e) => {
            error!("YAML configuration validation failed: {}", e);
            
            // Try to parse as basic YAML to get line numbers
            match serde_yaml::from_str::<serde_yaml::Value>(&request.yaml_content) {
                Ok(_) => {
                    errors.push(ValidationError {
                        line: None,
                        column: None,
                        message: format!("Configuration schema validation failed: {}", e),
                        severity: "error".to_string(),
                    });
                }
                Err(yaml_err) => {
                    // Extract line and column from YAML error if possible
                    let error_msg = yaml_err.to_string();
                    let (line, column) = extract_yaml_error_position(&error_msg);
                    
                    errors.push(ValidationError {
                        line,
                        column,
                        message: format!("YAML syntax error: {}", yaml_err),
                        severity: "error".to_string(),
                    });
                }
            }
        }
    }

    Ok(Json(ValidateConfigResponse {
        valid: errors.is_empty(),
        errors,
        warnings,
    }))
}

fn extract_yaml_error_position(error_msg: &str) -> (Option<u32>, Option<u32>) {
    // Try to extract line and column from YAML error message
    // Example: "at line 5 column 10"
    let line_regex = regex::Regex::new(r"line (\d+)").unwrap();
    let column_regex = regex::Regex::new(r"column (\d+)").unwrap();
    
    let line = line_regex
        .captures(error_msg)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());
    
    let column = column_regex
        .captures(error_msg)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());
    
    (line, column)
}

/// Generate a project from configuration
pub async fn generate_project(
    State(state): State<AppState>,
    Json(request): Json<GenerateProjectRequest>,
) -> Result<Json<GenerateProjectResponse>, AppError> {
    info!("Generating project: {} (format: {})", request.name, request.output_format);

    // Get config from database by ID
    let config_data = {
        let pool = &state.pool;
        let row = sqlx::query!("SELECT yaml_content FROM configs WHERE id = ?", request.config_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        match row {
            Some(row) => row.yaml_content,
            None => return Err(AppError::NotFound),
        }
    };

    // Parse configuration
    let config = Config::from_yaml(&config_data)
        .map_err(|e| AppError::Validation(format!("Invalid configuration: {}", e)))?;

    // Create temporary directory for generation
    let temp_dir = std::env::temp_dir().join(format!("rustform-gen-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| AppError::Internal(format!("Failed to create temp directory: {}", e)))?;

    // Generate project using rustform-codegen
    let generator = CodeGenerator::new();
    
    let generation_result = generator.generate_project(&config, &temp_dir).await
        .map_err(|e| AppError::Internal(format!("Code generation failed: {}", e)))?;

    // Store project record in database
    let project_id = {
        let pool = &state.pool;
        let result = sqlx::query!(
            "INSERT INTO projects (name, config_id, status, output_path, created_at) VALUES (?, ?, ?, ?, datetime('now'))",
            request.name,
            request.config_id,
            "completed",
            temp_dir.to_string_lossy().as_ref()
        )
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        result.last_insert_rowid()
    };

    match request.output_format.as_str() {
        "zip" => {
            // Create ZIP file
            let zip_path = temp_dir.with_extension("zip");
            create_zip_archive(&temp_dir, &zip_path)
                .map_err(|e| AppError::Internal(format!("Failed to create ZIP: {}", e)))?;

            // Generate download URL
            let download_url = format!("/api/projects/{}/download", project_id);

            // Store ZIP path in database
            let pool = &state.pool;
            sqlx::query!(
                "UPDATE projects SET zip_path = ? WHERE id = ?",
                zip_path.to_string_lossy().as_ref(),
                project_id
            )
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok(Json(GenerateProjectResponse {
                project_id: project_id as i32,
                download_url: Some(download_url),
                files: None,
            }))
        }
        "files" => {
            // Return file list with content
            let files = collect_generated_files(&temp_dir)
                .map_err(|e| AppError::Internal(format!("Failed to collect files: {}", e)))?;

            Ok(Json(GenerateProjectResponse {
                project_id: project_id as i32,
                download_url: None,
                files: Some(files),
            }))
        }
        _ => Err(AppError::Validation("Invalid output format. Use 'zip' or 'files'".to_string())),
    }
}

fn create_zip_archive(source_dir: &std::path::Path, zip_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use zip::write::FileOptions;

    let file = std::fs::File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(source_dir)?;

        if path.is_file() {
            zip.start_file(name.to_string_lossy().into_owned(), options)?;
            let content = std::fs::read(path)?;
            zip.write_all(&content)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy().into_owned(), options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn collect_generated_files(source_dir: &std::path::Path) -> Result<Vec<GeneratedFile>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(source_dir)?;
            let content = std::fs::read_to_string(path)?;

            files.push(GeneratedFile {
                path: relative_path.to_string_lossy().into_owned(),
                content,
            });
        }
    }

    Ok(files)
}

/// Get component details by name
pub async fn get_component_details(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ComponentInfo>, AppError> {
    info!("Getting component details for: {}", name);

    let registry = state.component_registry.lock().await;
    
    match registry.get_component(&name) {
        Some(component) => {
            info!("Found component: {}", name);
            Ok(Json(component.clone()))
        }
        None => {
            warn!("Component not found: {}", name);
            Err(AppError::NotFound)
        }
    }
}

/// Refresh component cache
pub async fn refresh_components(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    info!("Refreshing component cache...");

    let mut registry = state.component_registry.lock().await;
    
    match registry.refresh().await {
        Ok(()) => {
            let component_count = registry.components.len();
            info!("Component cache refreshed successfully with {} components", component_count);
            
            let mut status = HashMap::new();
            status.insert("status".to_string(), "success".to_string());
            status.insert("refreshed".to_string(), component_count.to_string());
            status.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            
            Ok(Json(status))
        }
        Err(e) => {
            error!("Failed to refresh component cache: {}", e);
            Err(AppError::Internal(format!("Component refresh failed: {}", e)))
        }
    }
}