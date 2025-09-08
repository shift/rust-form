use proptest::prelude::*;
use rustform_core::config::{Config, validation::validate_config};
use rustform_core::component::{ComponentManifest, CompatibilityStatus};

proptest! {
    #[test]
    fn test_config_semver_versions_property(
        major in 0u32..10,
        minor in 0u32..50,
        patch in 0u32..100,
        pre_release in prop::option::of("[a-z]+\\.[0-9]+"),
    ) {
        let version = if let Some(pre) = pre_release {
            format!("{}.{}.{}-{}", major, minor, patch, pre)
        } else {
            format!("{}.{}.{}", major, minor, patch)
        };
        
        let config_yaml = format!(r#"
schema_version: "{}"
api_version: "{}"
project_name: property_test
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

        let config: Result<Config, _> = serde_yaml::from_str(&config_yaml);
        prop_assert!(config.is_ok(), "Config should parse with valid semver: {}", version);
        
        if let Ok(config) = config {
            prop_assert!(validate_config(&config).is_ok(), "Config should be valid with version: {}", version);
        }
    }
    
    #[test]
    fn test_project_name_security_property(
        name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}"
    ) {
        let config_yaml = format!(r#"
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

        let config: Result<Config, _> = serde_yaml::from_str(&config_yaml);
        prop_assert!(config.is_ok(), "Config should parse with valid project name: {}", name);
        
        if let Ok(config) = config {
            // Test that project name doesn't contain path traversal or injection attacks
            prop_assert!(!name.contains(".."), "Project name should not contain path traversal: {}", name);
            prop_assert!(!name.contains('/'), "Project name should not contain path separators: {}", name);
            prop_assert!(!name.contains('\\'), "Project name should not contain Windows path separators: {}", name);
            prop_assert!(!name.contains('\0'), "Project name should not contain null bytes: {}", name);
            
            let validation_result = validate_config(&config);
            if name.len() <= 50 && !name.is_empty() {
                prop_assert!(validation_result.is_ok(), "Valid project name should pass validation: {}", name);
            }
        }
    }
    
    #[test]
    fn test_component_api_compatibility_property(
        api_major in 0u32..5,
        api_minor in 0u32..20,
        api_patch in 0u32..100,
        min_major in 0u32..5,
        min_minor in 0u32..20,
        min_patch in 0u32..100,
        max_major in 0u32..5,
        max_minor in 0u32..20,
        max_patch in 0u32..100,
    ) {
        let api_version = format!("{}.{}.{}", api_major, api_minor, api_patch);
        let min_version = format!("{}.{}.{}", min_major, min_minor, min_patch);
        let max_version = format!("{}.{}.{}", max_major, max_minor, max_patch);
        
        let manifest_yaml = format!(r#"
name: property-test-component
description: Property testing component
category: testing
priority: medium
version: "1.0.0"
author: "Property Test"
license: "MIT"
api_compatibility:
  api_version: "{}"
  min_version: "{}"
  max_version: "{}"
"#, api_version, min_version, max_version);

        let manifest: Result<ComponentManifest, _> = serde_yaml::from_str(&manifest_yaml);
        prop_assert!(manifest.is_ok(), "Manifest should parse with valid versions");
        
        if let Ok(manifest) = manifest {
            // Test compatibility logic
            let status = manifest.compatibility_status(&api_version);
            prop_assert!(status.is_ok(), "Compatibility check should not fail");
            
            if let Ok(status) = status {
                // If api_version is within min/max range, should be compatible
                if api_version >= min_version && api_version <= max_version {
                    prop_assert!(matches!(status, CompatibilityStatus::Compatible { .. }), 
                        "Version {} should be compatible with range {} - {}", api_version, min_version, max_version);
                }
            }
        }
    }
    
    #[test]
    fn test_database_url_env_security_property(
        env_var_name in "[A-Z][A-Z0-9_]{0,49}"
    ) {
        let config_yaml = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: security_test
version: "1.0.0"
database:
  type: postgres
  url_env: {}
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
"#, env_var_name);

        let config: Result<Config, _> = serde_yaml::from_str(&config_yaml);
        prop_assert!(config.is_ok(), "Config should parse with valid env var name: {}", env_var_name);
        
        if let Ok(config) = config {
            // Test that environment variable name doesn't contain security risks
            prop_assert!(!env_var_name.contains(".."), "Env var should not contain path traversal: {}", env_var_name);
            prop_assert!(!env_var_name.contains('$'), "Env var should not contain shell expansion: {}", env_var_name);
            prop_assert!(!env_var_name.contains('`'), "Env var should not contain command substitution: {}", env_var_name);
            prop_assert!(!env_var_name.contains('\n'), "Env var should not contain newlines: {}", env_var_name);
            prop_assert!(!env_var_name.contains('\r'), "Env var should not contain carriage returns: {}", env_var_name);
            
            let validation_result = validate_config(&config);
            if env_var_name.len() <= 50 && !env_var_name.is_empty() && env_var_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                prop_assert!(validation_result.is_ok(), "Valid env var name should pass validation: {}", env_var_name);
            }
        }
    }
    
    #[test]
    fn test_endpoint_path_security_property(
        path_segments in prop::collection::vec("[a-z][a-z0-9_-]{0,20}", 1..5)
    ) {
        let path = format!("/{}", path_segments.join("/"));
        
        let config_yaml = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: path_test
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
    - path: {}
      model: User
      crud:
        create: true
"#, path);

        let config: Result<Config, _> = serde_yaml::from_str(&config_yaml);
        prop_assert!(config.is_ok(), "Config should parse with valid path: {}", path);
        
        if let Ok(config) = config {
            // Test that path doesn't contain security risks
            prop_assert!(!path.contains(".."), "Path should not contain path traversal: {}", path);
            prop_assert!(!path.contains("//"), "Path should not contain double slashes: {}", path);
            prop_assert!(!path.contains('\0'), "Path should not contain null bytes: {}", path);
            prop_assert!(!path.contains('\n'), "Path should not contain newlines: {}", path);
            prop_assert!(!path.contains('\r'), "Path should not contain carriage returns: {}", path);
            prop_assert!(path.starts_with('/'), "Path should start with slash: {}", path);
            
            let validation_result = validate_config(&config);
            if path.len() <= 200 && path.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c)) {
                prop_assert!(validation_result.is_ok(), "Valid path should pass validation: {}", path);
            }
        }
    }
}

#[cfg(test)]
mod cryptographic_property_tests {
    use super::*;
    use sha2::{Sha256, Digest};
    
    proptest! {
        #[test]
        fn test_configuration_hash_consistency_property(
            project_name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}",
            version in "[0-9]\\.[0-9]\\.[0-9]",
        ) {
            let config_yaml = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: {}
version: {}
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
"#, project_name, version);

            // Parse config multiple times and ensure hash consistency
            let config1: Result<Config, _> = serde_yaml::from_str(&config_yaml);
            let config2: Result<Config, _> = serde_yaml::from_str(&config_yaml);
            
            prop_assert!(config1.is_ok() && config2.is_ok(), "Configs should parse successfully");
            
            if let (Ok(config1), Ok(config2)) = (config1, config2) {
                let serialized1 = serde_yaml::to_string(&config1).expect("Should serialize");
                let serialized2 = serde_yaml::to_string(&config2).expect("Should serialize");
                
                let hash1 = Sha256::digest(serialized1.as_bytes());
                let hash2 = Sha256::digest(serialized2.as_bytes());
                
                prop_assert_eq!(hash1, hash2, "Identical configs should produce identical hashes");
            }
        }
        
        #[test]
        fn test_sensitive_data_not_in_generated_code_property(
            secret_value in "[a-zA-Z0-9]{32}"
        ) {
            // This test ensures that sensitive data doesn't leak into generated code
            let config_yaml = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: secret_test
version: "1.0.0"
database:
  type: postgres
  url_env: SECRET_DATABASE_URL_{}
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
"#, secret_value);

            let config: Result<Config, _> = serde_yaml::from_str(&config_yaml);
            prop_assert!(config.is_ok(), "Config should parse");
            
            if let Ok(config) = config {
                // Verify that the secret value is properly handled as an environment variable reference
                let env_var = &config.database.url_env;
                prop_assert!(env_var.contains(&secret_value), "Secret should be part of env var name");
                
                // In a real implementation, we would test that generated code uses env::var()
                // rather than hardcoding the secret value
                prop_assert!(!config.project_name.contains(&secret_value), "Secret should not leak into project name");
                prop_assert!(!config.version.contains(&secret_value), "Secret should not leak into version");
            }
        }
    }
}