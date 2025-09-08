// Config Extensions - Custom Logic Implementation
// This file demonstrates the YAML-driven custom logic integration

use serde_yaml;
use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::models::{Config, CreateConfig, UpdateConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub project_name: String,
    pub version: String,
    pub models_count: usize,
    pub endpoints_count: usize,
    pub has_frontend: bool,
    pub database_type: String,
}

#[derive(Debug)]
pub struct ConfigValidationError {
    pub field: String,
    pub message: String,
    pub line: Option<usize>,
}

impl std::fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Validation error in {}: {}", self.field, self.message)
    }
}

impl Error for ConfigValidationError {}

impl Config {
    /// Validates YAML syntax and structure
    pub fn validate_yaml_syntax(&self) -> Result<(), Box<dyn Error>> {
        // Parse the YAML content to ensure it's valid
        let parsed: serde_yaml::Value = serde_yaml::from_str(&self.yaml_content)
            .map_err(|e| ConfigValidationError {
                field: "yaml_content".to_string(),
                message: format!("Invalid YAML syntax: {}", e),
                line: None,
            })?;

        // Validate required top-level fields
        let mapping = parsed.as_mapping().ok_or_else(|| ConfigValidationError {
            field: "yaml_content".to_string(),
            message: "YAML content must be a mapping/object".to_string(),
            line: None,
        })?;

        // Check for required fields
        let required_fields = ["project_name", "version", "database", "api"];
        for field in &required_fields {
            if !mapping.contains_key(&serde_yaml::Value::String(field.to_string())) {
                return Err(Box::new(ConfigValidationError {
                    field: "yaml_content".to_string(),
                    message: format!("Missing required field: {}", field),
                    line: None,
                }));
            }
        }

        // Validate project name format
        if let Some(project_name) = mapping.get(&serde_yaml::Value::String("project_name".to_string())) {
            if let Some(name_str) = project_name.as_str() {
                let name_regex = Regex::new(r"^[a-z][a-z0-9_]*$").unwrap();
                if !name_regex.is_match(name_str) {
                    return Err(Box::new(ConfigValidationError {
                        field: "project_name".to_string(),
                        message: "Project name must start with a letter and contain only lowercase letters, numbers, and underscores".to_string(),
                        line: None,
                    }));
                }
            }
        }

        Ok(())
    }

    /// Extracts project information from YAML configuration
    pub fn extract_project_info(&self) -> Result<ProjectInfo, Box<dyn Error>> {
        let parsed: serde_yaml::Value = serde_yaml::from_str(&self.yaml_content)?;
        let mapping = parsed.as_mapping().ok_or("Invalid YAML structure")?;

        let project_name = mapping
            .get(&serde_yaml::Value::String("project_name".to_string()))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let version = mapping
            .get(&serde_yaml::Value::String("version".to_string()))
            .and_then(|v| v.as_str())
            .unwrap_or("0.1.0")
            .to_string();

        // Count models
        let models_count = mapping
            .get(&serde_yaml::Value::String("api".to_string()))
            .and_then(|api| api.as_mapping())
            .and_then(|api_map| api_map.get(&serde_yaml::Value::String("models".to_string())))
            .and_then(|models| models.as_mapping())
            .map(|models_map| models_map.len())
            .unwrap_or(0);

        // Count endpoints
        let endpoints_count = mapping
            .get(&serde_yaml::Value::String("api".to_string()))
            .and_then(|api| api.as_mapping())
            .and_then(|api_map| api_map.get(&serde_yaml::Value::String("endpoints".to_string())))
            .and_then(|endpoints| endpoints.as_sequence())
            .map(|endpoints_seq| endpoints_seq.len())
            .unwrap_or(0);

        // Check for frontend configuration
        let has_frontend = mapping.contains_key(&serde_yaml::Value::String("frontend".to_string()));

        // Get database type
        let database_type = mapping
            .get(&serde_yaml::Value::String("database".to_string()))
            .and_then(|db| db.as_mapping())
            .and_then(|db_map| db_map.get(&serde_yaml::Value::String("type".to_string())))
            .and_then(|db_type| db_type.as_str())
            .unwrap_or("sqlite")
            .to_string();

        Ok(ProjectInfo {
            project_name,
            version,
            models_count,
            endpoints_count,
            has_frontend,
            database_type,
        })
    }

    /// Sanitizes configuration content by removing potentially harmful elements
    pub fn sanitize_config(&self) -> Result<String, Box<dyn Error>> {
        let mut parsed: serde_yaml::Value = serde_yaml::from_str(&self.yaml_content)?;

        // Remove any potential script injections or dangerous configurations
        if let Some(mapping) = parsed.as_mapping_mut() {
            // Remove any fields that might contain scripts
            let dangerous_fields = ["script", "exec", "command", "shell"];
            for field in &dangerous_fields {
                mapping.remove(&serde_yaml::Value::String(field.to_string()));
            }

            // Sanitize database URL to prevent injection
            if let Some(db_config) = mapping.get_mut(&serde_yaml::Value::String("database".to_string())) {
                if let Some(db_mapping) = db_config.as_mapping_mut() {
                    if let Some(url_env) = db_mapping.get_mut(&serde_yaml::Value::String("url_env".to_string())) {
                        if let Some(url_str) = url_env.as_str() {
                            // Ensure it's just an environment variable name
                            let env_var_regex = Regex::new(r"^[A-Z_][A-Z0-9_]*$").unwrap();
                            if !env_var_regex.is_match(url_str) {
                                *url_env = serde_yaml::Value::String("DATABASE_URL".to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(serde_yaml::to_string(&parsed)?)
    }

    /// Hook: Validates and sanitizes configuration before creation
    pub async fn validate_and_sanitize(&self, data: &mut CreateConfig) -> Result<(), Box<dyn Error>> {
        // Create a temporary config for validation
        let temp_config = Config {
            id: None,
            name: data.name.clone(),
            description: data.description.as_ref().and_then(|d| d.clone()),
            yaml_content: data.yaml_content.clone(),
            is_template: data.is_template.as_ref().and_then(|t| t.clone()),
            created_at: None,
            updated_at: None,
        };

        // Validate syntax
        temp_config.validate_yaml_syntax()?;

        // Sanitize content
        data.yaml_content = temp_config.sanitize_config()?;

        Ok(())
    }

    /// Hook: Logs configuration creation for audit purposes
    pub async fn log_config_creation(&self, entity: &Config) -> Result<(), Box<dyn Error>> {
        let project_info = entity.extract_project_info()?;
        
        tracing::info!(
            "Configuration created: {} (version: {}, models: {}, endpoints: {}, frontend: {})",
            project_info.project_name,
            project_info.version,
            project_info.models_count,
            project_info.endpoints_count,
            project_info.has_frontend
        );

        // In a real implementation, this could:
        // - Write to an audit log
        // - Send notifications
        // - Update analytics
        // - Trigger webhooks

        Ok(())
    }

    /// Hook: Validates YAML changes before updates
    pub async fn validate_yaml_changes(&self, _id: &str, data: &mut UpdateConfig) -> Result<(), Box<dyn Error>> {
        if let Some(ref yaml_content) = data.yaml_content {
            // Create temporary config for validation
            let temp_config = Config {
                id: self.id,
                name: data.name.clone().unwrap_or_else(|| self.name.clone()),
                description: data.description.as_ref().and_then(|d| d.clone()).or_else(|| self.description.clone()),
                yaml_content: yaml_content.clone(),
                is_template: data.is_template.as_ref().and_then(|t| t.clone()).or(self.is_template),
                created_at: self.created_at,
                updated_at: self.updated_at,
            };

            temp_config.validate_yaml_syntax()?;
            
            // Update with sanitized content
            data.yaml_content = Some(temp_config.sanitize_config()?);
        }

        Ok(())
    }
}

// Advanced configuration analysis functions
pub fn analyze_configuration_complexity(yaml_content: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let parsed: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
    let mut metrics = HashMap::new();

    if let Some(mapping) = parsed.as_mapping() {
        // Count various complexity metrics
        metrics.insert("total_fields".to_string(), count_fields_recursive(&parsed));
        metrics.insert("nesting_depth".to_string(), calculate_nesting_depth(&parsed));
        
        if let Some(api) = mapping.get(&serde_yaml::Value::String("api".to_string())) {
            if let Some(api_mapping) = api.as_mapping() {
                if let Some(models) = api_mapping.get(&serde_yaml::Value::String("models".to_string())) {
                    metrics.insert("models_count".to_string(), models.as_mapping().map(|m| m.len()).unwrap_or(0));
                }
                
                if let Some(endpoints) = api_mapping.get(&serde_yaml::Value::String("endpoints".to_string())) {
                    metrics.insert("endpoints_count".to_string(), endpoints.as_sequence().map(|e| e.len()).unwrap_or(0));
                }
            }
        }
    }

    Ok(metrics)
}

fn count_fields_recursive(value: &serde_yaml::Value) -> usize {
    match value {
        serde_yaml::Value::Mapping(map) => {
            map.len() + map.values().map(count_fields_recursive).sum::<usize>()
        }
        serde_yaml::Value::Sequence(seq) => {
            seq.iter().map(count_fields_recursive).sum()
        }
        _ => 0,
    }
}

fn calculate_nesting_depth(value: &serde_yaml::Value) -> usize {
    match value {
        serde_yaml::Value::Mapping(map) => {
            1 + map.values().map(calculate_nesting_depth).max().unwrap_or(0)
        }
        serde_yaml::Value::Sequence(seq) => {
            1 + seq.iter().map(calculate_nesting_depth).max().unwrap_or(0)
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_yaml_syntax_valid() {
        let config = Config {
            id: Some(1),
            name: "test".to_string(),
            description: None,
            yaml_content: r#"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models: {}
  endpoints: []
"#.to_string(),
            is_template: Some(false),
            created_at: None,
            updated_at: None,
        };

        assert!(config.validate_yaml_syntax().is_ok());
    }

    #[test]
    fn test_validate_yaml_syntax_invalid() {
        let config = Config {
            id: Some(1),
            name: "test".to_string(),
            description: None,
            yaml_content: "invalid: yaml: content:".to_string(),
            is_template: Some(false),
            created_at: None,
            updated_at: None,
        };

        assert!(config.validate_yaml_syntax().is_err());
    }

    #[test]
    fn test_extract_project_info() {
        let config = Config {
            id: Some(1),
            name: "test".to_string(),
            description: None,
            yaml_content: r#"
project_name: my_awesome_project
version: "2.0.0"
database:
  type: postgres
api:
  models:
    User: {}
    Post: {}
  endpoints:
    - path: /users
    - path: /posts
frontend:
  target: react
"#.to_string(),
            is_template: Some(false),
            created_at: None,
            updated_at: None,
        };

        let info = config.extract_project_info().unwrap();
        assert_eq!(info.project_name, "my_awesome_project");
        assert_eq!(info.version, "2.0.0");
        assert_eq!(info.models_count, 2);
        assert_eq!(info.endpoints_count, 2);
        assert_eq!(info.has_frontend, true);
        assert_eq!(info.database_type, "postgres");
    }
}