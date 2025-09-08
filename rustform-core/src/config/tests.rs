use rstest::*;
use tempfile::TempDir;
use similar_asserts::assert_eq;
use crate::config::{Config, Day2Operations, DeprecationWarning, CompatibilityReport, Severity};
use crate::error::ValidationError;

#[rstest]
fn test_config_with_versions() {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
        name:
          type: string
          required: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    
    assert_eq!(config.schema_version, "1.0.0");
    assert_eq!(config.api_version, "0.1.0");
    assert_eq!(config.project_name, "test_project");
    assert_eq!(config.version, "1.0.0");
}

#[rstest]
fn test_config_default_versions() {
    let yaml = r#"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
        name:
          type: string
          required: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    
    // Should use default values
    assert_eq!(config.schema_version, "1.0.0");
    assert_eq!(config.api_version, "0.1.0");
}

#[cfg(feature = "registry")]
#[rstest]
fn test_registry_config() {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
registry:
  url: "https://registry.rustform.dev"
  auth:
    type: token
    credentials: "test_token"
  cache:
    enabled: true
    ttl: 3600
    directory: ".cache"
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    
    let registry = config.registry.expect("Registry config should be present");
    assert_eq!(registry.url, "https://registry.rustform.dev");
    assert_eq!(registry.auth.unwrap().auth_type, "token");
    assert_eq!(registry.cache.ttl, 3600);
}

#[rstest]
fn test_deprecation_warnings() {
    let yaml = r#"
schema_version: "0.8.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
middleware:
  - security:
      helmet: true
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    let warnings = Day2Operations::check_deprecations(&config);
    
    // Should have deprecation warnings for old schema version and helmet
    assert!(warnings.len() >= 1);
    assert!(warnings.iter().any(|w| w.feature.contains("Schema version")));
}

#[rstest]
fn test_compatibility_report() {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    let report = Day2Operations::check_compatibility_matrix(&config);
    
    assert_eq!(report.config_api_version, "0.1.0");
    assert_eq!(report.config_schema_version, "1.0.0");
    assert!(report.overall_compatible);
}

#[rstest]
fn test_incompatible_api_version() {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "1.0.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    let report = Day2Operations::check_compatibility_matrix(&config);
    
    // Should have compatibility issues with api_version 1.0.0 vs current 0.1.0
    assert!(!report.overall_compatible);
    assert!(report.issues.iter().any(|issue| 
        issue.component == "rust-form core" && 
        matches!(issue.severity, Severity::Error)
    ));
}

#[rstest]
fn test_migration_guide_generation() {
    let guide = Day2Operations::generate_migration_guide("0.9.0", "1.0.0")
        .expect("Should generate migration guide");
    
    assert_eq!(guide.from_version, "0.9.0");
    assert_eq!(guide.to_version, "1.0.0");
    assert!(guide.breaking_changes);
    assert!(!guide.steps.is_empty());
    
    // Should have steps for adding version fields
    assert!(guide.steps.iter().any(|step| 
        step.description.contains("schema_version") || 
        step.description.contains("api_version")
    ));
}

#[rstest]
fn test_semver_validation() {
    use crate::config::validation::validate_config;
    
    // Valid semver versions
    let valid_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0-alpha.1"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    let config: Config = serde_yaml::from_str(valid_yaml).expect("Failed to parse valid YAML");
    assert!(validate_config(&config).is_ok());
    
    // Invalid semver versions
    let invalid_yaml = r#"
schema_version: "invalid"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    let config: Config = serde_yaml::from_str(invalid_yaml).expect("Failed to parse invalid YAML");
    assert!(validate_config(&config).is_err());
}

#[rstest]
fn test_config_serialization_round_trip() {
    let original_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: test_project
version: "1.0.0"
database:
  type: postgres
  url_env: DATABASE_URL
  pool_size: 20
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: uuid
          primary_key: true
        email:
          type: string
          required: true
          unique: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#;

    let config: Config = serde_yaml::from_str(original_yaml).expect("Failed to parse YAML");
    let serialized = serde_yaml::to_string(&config).expect("Failed to serialize config");
    let deserialized: Config = serde_yaml::from_str(&serialized).expect("Failed to deserialize config");
    
    assert_eq!(config.schema_version, deserialized.schema_version);
    assert_eq!(config.api_version, deserialized.api_version);
    assert_eq!(config.project_name, deserialized.project_name);
    assert_eq!(config.version, deserialized.version);
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_semver_parsing_property(
            major in 0u32..100,
            minor in 0u32..100, 
            patch in 0u32..100
        ) {
            let version = format!("{}.{}.{}", major, minor, patch);
            let yaml = format!(r#"
schema_version: "{}"
api_version: "{}"
project_name: test_project
version: "{}"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#, version, version, version);

            let config: Result<Config, _> = serde_yaml::from_str(&yaml);
            prop_assert!(config.is_ok());
            
            let config = config.unwrap();
            prop_assert_eq!(config.schema_version, version);
            prop_assert_eq!(config.api_version, version);
            prop_assert_eq!(config.version, version);
        }
        
        #[test]
        fn test_project_name_validation_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}"
        ) {
            let yaml = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: {}
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#, name);

            let config: Result<Config, _> = serde_yaml::from_str(&yaml);
            prop_assert!(config.is_ok());
            
            if let Ok(config) = config {
                use crate::config::validation::validate_config;
                prop_assert!(validate_config(&config).is_ok());
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_config_file_loading() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.yml");
        
        let config_content = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: file_test
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: integer
          primary_key: true
        name:
          type: string
          required: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
"#;
        
        fs::write(&config_path, config_content).expect("Failed to write config file");
        
        let file_content = fs::read_to_string(&config_path).expect("Failed to read config file");
        let config: Config = serde_yaml::from_str(&file_content).expect("Failed to parse config");
        
        assert_eq!(config.project_name, "file_test");
        assert_eq!(config.schema_version, "1.0.0");
        assert_eq!(config.api_version, "0.1.0");
    }
}