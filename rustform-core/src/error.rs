use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Configuration error")]
    Config(#[from] ConfigError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Component error: {0}")]
    ComponentError(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parsing error")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum ConfigError {
    #[error("YAML parsing error")]
    #[diagnostic(
        code(config::yaml_syntax),
        help("Check your YAML syntax. Common issues include incorrect indentation, missing colons, or invalid characters.")
    )]
    Yaml(#[from] serde_yaml::Error),

    #[error("Configuration validation failed")]
    #[diagnostic(code(config::validation))]
    Validation(#[from] ValidationError),

    #[error("IO error")]
    #[diagnostic(code(config::io))]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug, Diagnostic)]
pub enum ValidationError {
    #[error("Required field '{field}' is missing")]
    #[diagnostic(
        code(validation::missing_field),
        help("Add the required field '{field}' to your configuration")
    )]
    MissingField { field: String },

    #[error("Invalid field type for '{field}': expected {expected}, got {actual}")]
    #[diagnostic(
        code(validation::invalid_type),
        help("Change the type of field '{field}' to {expected}")
    )]
    InvalidType {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Model '{model}' referenced in endpoint but not defined")]
    #[diagnostic(
        code(validation::model_not_found),
        help("Define the model '{model}' in the 'models' section or fix the reference")
    )]
    ModelNotFound { model: String },

    #[error("Duplicate table name '{table_name}' found in models '{model1}' and '{model2}'")]
    #[diagnostic(
        code(validation::duplicate_table),
        help("Each model must have a unique table name. Rename one of the tables.")
    )]
    DuplicateTableName {
        table_name: String,
        model1: String,
        model2: String,
    },

    #[error("Invalid endpoint path '{path}': {reason}")]
    #[diagnostic(
        code(validation::invalid_path),
        help("Ensure the path starts with '/' and uses valid URL characters")
    )]
    InvalidPath { path: String, reason: String },

    #[error("Model '{model}' has no primary key defined")]
    #[diagnostic(
        code(validation::no_primary_key),
        help(
            "Add a primary key field to model '{model}' by setting 'primary_key: true' on a field"
        )
    )]
    NoPrimaryKey { model: String },

    #[error("Model '{model}' has multiple primary keys: {fields:?}")]
    #[diagnostic(
        code(validation::multiple_primary_keys),
        help("A model can only have one primary key. Choose one field to be the primary key.")
    )]
    MultiplePrimaryKeys { model: String, fields: Vec<String> },

    #[error("Invalid field constraint for '{field}' in model '{model}': {reason}")]
    #[diagnostic(
        code(validation::invalid_constraint),
        help("Check the field constraints and ensure they are compatible with the field type")
    )]
    InvalidFieldConstraint {
        field: String,
        model: String,
        reason: String,
    },

    #[error("Invalid relationship in model '{model}', field '{field}': {reason}")]
    #[diagnostic(
        code(validation::invalid_relationship),
        help("Check the relationship configuration and ensure the referenced model exists")
    )]
    InvalidRelationship {
        model: String,
        field: String,
        reason: String,
    },

    #[error("Invalid index '{index}' in model '{model}': {reason}")]
    #[diagnostic(
        code(validation::invalid_index),
        help("Check the index configuration and ensure all referenced fields exist")
    )]
    InvalidIndex {
        model: String,
        index: String,
        reason: String,
    },

    #[error("Invalid middleware configuration: {reason}")]
    #[diagnostic(
        code(validation::invalid_middleware),
        help("Check the middleware configuration format and values")
    )]
    InvalidMiddleware { reason: String },

    #[error("Invalid project name '{name}': {reason}")]
    #[diagnostic(
        code(validation::invalid_project_name),
        help("Project name should be a valid identifier (letters, numbers, underscores, hyphens)")
    )]
    InvalidProjectName { name: String, reason: String },

    #[error("Invalid version '{version}': must be a valid semantic version")]
    #[diagnostic(
        code(validation::invalid_version),
        help("Use semantic versioning format like '1.0.0' or '0.1.0-alpha'")
    )]
    InvalidVersion { version: String },

    #[error("Empty configuration section: {section}")]
    #[diagnostic(
        code(validation::empty_section),
        help("Add at least one item to the '{section}' section or remove it")
    )]
    EmptySection { section: String },

    #[error("Circular dependency detected in relationships: {cycle:?}")]
    #[diagnostic(
        code(validation::circular_dependency),
        help("Remove circular references between models to break the dependency cycle")
    )]
    CircularDependency { cycle: Vec<String> },

    #[error("Incompatible API version '{requested}': current rust-form version is '{current}' ({reason})")]
    #[diagnostic(
        code(validation::incompatible_api_version),
        help("Update your configuration's api_version to match a compatible rust-form version or upgrade rust-form")
    )]
    IncompatibleApiVersion {
        requested: String,
        current: String,
        reason: String,
    },
}

pub type ValidationResult<T> = std::result::Result<T, ValidationError>;
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[cfg(test)]
mod tests;
