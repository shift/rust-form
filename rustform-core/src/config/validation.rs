use crate::config::{Config, ModelConfig, EndpointConfig, FieldConfig, FieldType, RelationshipType};
use crate::error::{ValidationError, ValidationResult};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use indexmap::IndexMap;

pub fn validate_config(config: &Config) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Validate project name
    if let Err(e) = validate_project_name(&config.project_name) {
        errors.push(e);
    }

    // Validate version
    if let Err(e) = validate_version(&config.version) {
        errors.push(e);
    }

    // Validate database configuration
    if let Err(e) = validate_database_config(&config.database) {
        errors.push(e);
    }

    // Validate API configuration
    if let Err(e) = validate_api_config(&config.api) {
        errors.push(e);
    }

    // Validate middleware configuration
    if let Err(e) = validate_middleware_config(&config.middleware) {
        errors.push(e);
    }

    // Return first error if any (in a real implementation, you might collect all errors)
    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    Ok(())
}

fn validate_project_name(name: &str) -> ValidationResult<()> {
    if name.is_empty() {
        return Err(ValidationError::MissingField {
            field: "project_name".to_string(),
        });
    }

    let valid_pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    if !valid_pattern.is_match(name) {
        return Err(ValidationError::InvalidProjectName {
            name: name.to_string(),
            reason: "must start with a letter and contain only letters, numbers, underscores, and hyphens".to_string(),
        });
    }

    if name.len() > 50 {
        return Err(ValidationError::InvalidProjectName {
            name: name.to_string(),
            reason: "must be 50 characters or less".to_string(),
        });
    }

    Ok(())
}

fn validate_version(version: &str) -> ValidationResult<()> {
    let semver_pattern = Regex::new(r"^\d+\.\d+\.\d+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$").unwrap();
    if !semver_pattern.is_match(version) {
        return Err(ValidationError::InvalidVersion {
            version: version.to_string(),
        });
    }
    Ok(())
}

fn validate_database_config(db_config: &crate::config::DatabaseConfig) -> ValidationResult<()> {
    if db_config.url_env.is_empty() {
        return Err(ValidationError::MissingField {
            field: "database.url_env".to_string(),
        });
    }

    // Validate environment variable name format
    let env_pattern = Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap();
    if !env_pattern.is_match(&db_config.url_env) {
        return Err(ValidationError::InvalidType {
            field: "database.url_env".to_string(),
            expected: "uppercase environment variable name".to_string(),
            actual: db_config.url_env.clone(),
        });
    }

    Ok(())
}

fn validate_api_config(api_config: &crate::config::ApiConfig) -> ValidationResult<()> {
    if api_config.models.is_empty() {
        return Err(ValidationError::EmptySection {
            section: "api.models".to_string(),
        });
    }

    if api_config.endpoints.is_empty() {
        return Err(ValidationError::EmptySection {
            section: "api.endpoints".to_string(),
        });
    }

    // Validate models
    validate_models(&api_config.models)?;

    // Validate endpoints
    validate_endpoints(&api_config.endpoints, &api_config.models)?;

    // Check for circular dependencies in relationships
    validate_no_circular_dependencies(&api_config.models)?;

    Ok(())
}

fn validate_models(models: &IndexMap<String, ModelConfig>) -> ValidationResult<()> {
    let mut table_names: HashMap<String, String> = HashMap::new();

    for (model_name, model_config) in models {
        // Validate model name
        if model_name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "model name".to_string(),
            });
        }

        // Check for duplicate table names
        if let Some(existing_model) = table_names.get(&model_config.table_name) {
            return Err(ValidationError::DuplicateTableName {
                table_name: model_config.table_name.clone(),
                model1: existing_model.clone(),
                model2: model_name.clone(),
            });
        }
        table_names.insert(model_config.table_name.clone(), model_name.clone());

        // Validate model fields
        validate_model_fields(model_name, &model_config.fields)?;

        // Validate relationships
        validate_model_relationships(model_name, &model_config.relationships, models)?;

        // Validate indexes
        validate_model_indexes(model_name, &model_config.indexes, &model_config.fields)?;
    }

    Ok(())
}

fn validate_model_fields(model_name: &str, fields: &IndexMap<String, FieldConfig>) -> ValidationResult<()> {
    if fields.is_empty() {
        return Err(ValidationError::EmptySection {
            section: format!("model '{}' fields", model_name),
        });
    }

    let mut primary_keys = Vec::new();
    
    for (field_name, field_config) in fields {
        if field_config.primary_key {
            primary_keys.push(field_name.clone());
        }

        // Validate field constraints
        validate_field_constraints(model_name, field_name, field_config)?;
    }

    // Check primary key constraints
    match primary_keys.len() {
        0 => Err(ValidationError::NoPrimaryKey {
            model: model_name.to_string(),
        }),
        1 => Ok(()),
        _ => Err(ValidationError::MultiplePrimaryKeys {
            model: model_name.to_string(),
            fields: primary_keys,
        }),
    }
}

fn validate_field_constraints(model_name: &str, field_name: &str, field_config: &FieldConfig) -> ValidationResult<()> {
    // Auto increment can only be used with integer primary keys
    if field_config.auto_increment && !field_config.primary_key {
        return Err(ValidationError::InvalidFieldConstraint {
            field: field_name.to_string(),
            model: model_name.to_string(),
            reason: "auto_increment can only be used on primary key fields".to_string(),
        });
    }

    if field_config.auto_increment && field_config.field_type != FieldType::Integer {
        return Err(ValidationError::InvalidFieldConstraint {
            field: field_name.to_string(),
            model: model_name.to_string(),
            reason: "auto_increment can only be used with integer fields".to_string(),
        });
    }

    // Auto now fields should be datetime
    if (field_config.auto_now || field_config.auto_now_add) && field_config.field_type != FieldType::DateTime {
        return Err(ValidationError::InvalidFieldConstraint {
            field: field_name.to_string(),
            model: model_name.to_string(),
            reason: "auto_now and auto_now_add can only be used with datetime fields".to_string(),
        });
    }

    // Length constraints only apply to string/text fields
    if field_config.max_length.is_some() || field_config.min_length.is_some() {
        match field_config.field_type {
            FieldType::String | FieldType::Text => {},
            _ => return Err(ValidationError::InvalidFieldConstraint {
                field: field_name.to_string(),
                model: model_name.to_string(),
                reason: "length constraints can only be used with string or text fields".to_string(),
            }),
        }
    }

    // Value constraints only apply to numeric fields
    if field_config.min_value.is_some() || field_config.max_value.is_some() {
        match field_config.field_type {
            FieldType::Integer | FieldType::Float | FieldType::Double | FieldType::Decimal => {},
            _ => return Err(ValidationError::InvalidFieldConstraint {
                field: field_name.to_string(),
                model: model_name.to_string(),
                reason: "value constraints can only be used with numeric fields".to_string(),
            }),
        }
    }

    Ok(())
}

fn validate_model_relationships(
    model_name: &str,
    relationships: &IndexMap<String, crate::config::RelationshipConfig>,
    all_models: &IndexMap<String, ModelConfig>,
) -> ValidationResult<()> {
    for (rel_name, rel_config) in relationships {
        // Check if referenced model exists
        if !all_models.contains_key(&rel_config.model) {
            return Err(ValidationError::InvalidRelationship {
                model: model_name.to_string(),
                field: rel_name.to_string(),
                reason: format!("referenced model '{}' does not exist", rel_config.model),
            });
        }

        // Validate foreign key if specified
        if let Some(foreign_key) = &rel_config.foreign_key {
            let current_model = all_models.get(model_name).unwrap();
            if !current_model.fields.contains_key(foreign_key) {
                return Err(ValidationError::InvalidRelationship {
                    model: model_name.to_string(),
                    field: rel_name.to_string(),
                    reason: format!("foreign key field '{}' does not exist in model", foreign_key),
                });
            }
        }
    }

    Ok(())
}

fn validate_model_indexes(
    model_name: &str,
    indexes: &[crate::config::IndexConfig],
    fields: &IndexMap<String, FieldConfig>,
) -> ValidationResult<()> {
    for index in indexes {
        // Check if all index fields exist
        for field_name in &index.fields {
            if !fields.contains_key(field_name) {
                return Err(ValidationError::InvalidIndex {
                    model: model_name.to_string(),
                    index: index.name.clone(),
                    reason: format!("field '{}' does not exist in model", field_name),
                });
            }
        }

        // Index should have at least one field
        if index.fields.is_empty() {
            return Err(ValidationError::InvalidIndex {
                model: model_name.to_string(),
                index: index.name.clone(),
                reason: "index must specify at least one field".to_string(),
            });
        }
    }

    Ok(())
}

fn validate_endpoints(
    endpoints: &[EndpointConfig],
    models: &IndexMap<String, ModelConfig>,
) -> ValidationResult<()> {
    let mut paths = HashSet::new();

    for endpoint in endpoints {
        // Validate path format
        validate_endpoint_path(&endpoint.path)?;

        // Check for duplicate paths
        if paths.contains(&endpoint.path) {
            return Err(ValidationError::InvalidPath {
                path: endpoint.path.clone(),
                reason: "duplicate endpoint path".to_string(),
            });
        }
        paths.insert(&endpoint.path);

        // Check if referenced model exists
        if !models.contains_key(&endpoint.model) {
            return Err(ValidationError::ModelNotFound {
                model: endpoint.model.clone(),
            });
        }

        // Validate that at least one CRUD operation is enabled
        let crud = &endpoint.crud;
        if !crud.create && !crud.read_all && !crud.read_one && !crud.update && !crud.delete && !crud.patch {
            return Err(ValidationError::InvalidPath {
                path: endpoint.path.clone(),
                reason: "at least one CRUD operation must be enabled".to_string(),
            });
        }
    }

    Ok(())
}

fn validate_endpoint_path(path: &str) -> ValidationResult<()> {
    if path.is_empty() {
        return Err(ValidationError::InvalidPath {
            path: path.to_string(),
            reason: "path cannot be empty".to_string(),
        });
    }

    if !path.starts_with('/') {
        return Err(ValidationError::InvalidPath {
            path: path.to_string(),
            reason: "path must start with '/'".to_string(),
        });
    }

    // Basic path validation - could be more sophisticated
    let path_pattern = Regex::new(r"^/[a-zA-Z0-9/_-]*$").unwrap();
    if !path_pattern.is_match(path) {
        return Err(ValidationError::InvalidPath {
            path: path.to_string(),
            reason: "path contains invalid characters".to_string(),
        });
    }

    Ok(())
}

fn validate_no_circular_dependencies(models: &IndexMap<String, ModelConfig>) -> ValidationResult<()> {
    for (model_name, _model_config) in models {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if let Err(cycle) = check_circular_dependency(model_name, models, &mut visited, &mut path) {
            return Err(ValidationError::CircularDependency { cycle });
        }
    }
    Ok(())
}

fn check_circular_dependency(
    model_name: &str,
    models: &IndexMap<String, ModelConfig>,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Result<(), Vec<String>> {
    if path.contains(&model_name.to_string()) {
        // Found a cycle
        let cycle_start = path.iter().position(|x| x == model_name).unwrap();
        return Err(path[cycle_start..].to_vec());
    }

    if visited.contains(model_name) {
        return Ok(());
    }

    visited.insert(model_name.to_string());
    path.push(model_name.to_string());

    if let Some(model) = models.get(model_name) {
        for relationship in model.relationships.values() {
            // Skip self-references as they are valid
            if relationship.model == model_name {
                continue;
            }
            
            // Only check relationships that could create dependencies
            match relationship.relationship_type {
                RelationshipType::OneToOne | RelationshipType::ManyToOne => {
                    check_circular_dependency(&relationship.model, models, visited, path)?;
                }
                _ => {} // OneToMany and ManyToMany don't create hard dependencies
            }
        }
    }

    path.pop();
    Ok(())
}

fn validate_middleware_config(middleware: &[crate::config::MiddlewareConfig]) -> ValidationResult<()> {
    // Basic middleware validation - could be extended
    for middleware_item in middleware {
        match middleware_item {
            crate::config::MiddlewareConfig::Cors { cors } => {
                if cors.allow_origin.is_empty() {
                    return Err(ValidationError::InvalidMiddleware {
                        reason: "CORS allow_origin cannot be empty".to_string(),
                    });
                }
            }
            crate::config::MiddlewareConfig::RateLimit { rate_limit } => {
                if rate_limit.max_requests == 0 {
                    return Err(ValidationError::InvalidMiddleware {
                        reason: "Rate limit max_requests must be greater than 0".to_string(),
                    });
                }
                if rate_limit.window_seconds == 0 {
                    return Err(ValidationError::InvalidMiddleware {
                        reason: "Rate limit window_seconds must be greater than 0".to_string(),
                    });
                }
            }
            _ => {} // Other middleware types pass basic validation
        }
    }

    Ok(())
}