use rstest::*;
use tempfile::TempDir;
use similar_asserts::assert_eq;
use crate::component::{ComponentManifest, ComponentUri, ComponentSystem, CompatibilityStatus};

#[rstest]
fn test_component_manifest_parsing() {
    let yaml = r#"
name: jwt-auth
description: JWT authentication component
category: auth
priority: high
version: "1.0.0"
author: "Test Author"
license: "MIT"
repository: "https://github.com/test/jwt-auth"
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
dependencies:
  runtime:
    - "jsonwebtoken"
    - "serde"
  dev:
    - "tokio-test"
templates:
  - name: auth_handler
    path: "auth/jwt_handler.rs.tera"
  - name: middleware
    path: "auth/jwt_middleware.rs.tera"
tests:
  generate_unit_tests: true
  generate_integration_tests: true
  test_framework: "rstest"
"#;

    let manifest: ComponentManifest = serde_yaml::from_str(yaml).expect("Failed to parse component manifest");
    
    assert_eq!(manifest.name, "jwt-auth");
    assert_eq!(manifest.description, "JWT authentication component");
    assert_eq!(manifest.api_compatibility.api_version, "0.1.0");
    assert_eq!(manifest.api_compatibility.min_version, "0.1.0");
    assert_eq!(manifest.api_compatibility.max_version, Some("0.2.0".to_string()));
}

#[rstest]
fn test_component_compatibility_checking() {
    let manifest = create_test_manifest();
    
    // Test compatible version
    let status = manifest.compatibility_status("0.1.5").expect("Should check compatibility");
    assert!(matches!(status, CompatibilityStatus::Compatible { .. }));
    
    // Test incompatible versions
    let status = manifest.compatibility_status("0.0.9").expect("Should check compatibility");
    assert!(matches!(status, CompatibilityStatus::TooOld { .. }));
    
    let status = manifest.compatibility_status("0.3.0").expect("Should check compatibility");
    assert!(matches!(status, CompatibilityStatus::TooNew { .. }));
    
    // Test boundary conditions
    let status = manifest.compatibility_status(&manifest.api_compatibility.min_version).expect("Should check compatibility");
    assert!(matches!(status, CompatibilityStatus::Compatible { .. }));
    
    if let Some(ref max_version) = manifest.api_compatibility.max_version {
        let status = manifest.compatibility_status(max_version).expect("Should check compatibility");
        assert!(matches!(status, CompatibilityStatus::Compatible { .. }));
    }
}

#[rstest]
fn test_component_uri_parsing() {
    // Test GitHub URI
    let uri_str = "github://owner/repo@1.0.0";
    let uri: ComponentUri = uri_str.parse().expect("Should parse GitHub URI");
    assert_eq!(uri.provider(), "github");
    
    // Test local file URI
    let uri_str = "file://./components/auth";
    let uri: ComponentUri = uri_str.parse().expect("Should parse file URI");
    assert_eq!(uri.provider(), "file");
    
    // Test registry URI
    let uri_str = "registry://auth/jwt-manager@1.0.0";
    let uri: ComponentUri = uri_str.parse().expect("Should parse registry URI");
    assert_eq!(uri.provider(), "registry");
}

#[rstest]
fn test_component_validation() {
    let mut manifest = create_test_manifest();
    
    // Valid manifest should pass
    assert!(manifest.validate().is_ok());
    
    // Invalid name should fail
    manifest.name = "invalid name!".to_string();
    assert!(manifest.validate().is_err());
    
    // Invalid API version should fail
    manifest.name = "valid-name".to_string();
    manifest.api_compatibility.api_version = "invalid".to_string();
    assert!(manifest.validate().is_err());
    
    // Min version > max version should fail
    manifest.api_compatibility.api_version = "0.1.0".to_string();
    manifest.api_compatibility.min_version = "0.2.0".to_string();
    manifest.api_compatibility.max_version = Some("0.1.0".to_string());
    assert!(manifest.validate().is_err());
}

#[rstest]
#[tokio::test]
async fn test_component_system_creation() {
    let system = ComponentSystem::new().expect("Should create component system");
    assert_eq!(system.rust_form_version(), env!("CARGO_PKG_VERSION"));
    
    let custom_version = "0.2.0";
    let system = ComponentSystem::with_version(custom_version.to_string()).expect("Should create component system with custom version");
    assert_eq!(system.rust_form_version(), custom_version);
}

#[rstest]
#[tokio::test] 
async fn test_component_system_compatibility_check() {
    let system = ComponentSystem::new().expect("Should create component system");
    let manifest = create_test_manifest();
    
    // Compatible manifest should pass
    assert!(system.check_compatibility(&manifest).is_ok());
    
    // Incompatible manifest should fail
    let mut incompatible_manifest = create_test_manifest();
    incompatible_manifest.api_compatibility.max_version = Some("0.0.1".to_string());
    incompatible_manifest.api_compatibility.min_version = "0.0.1".to_string();
    
    assert!(system.check_compatibility(&incompatible_manifest).is_err());
}

fn create_test_manifest() -> ComponentManifest {
    ComponentManifest {
        name: "jwt-manager".to_string(),
        description: "JWT token management".to_string(),
        category: crate::component::ComponentCategory::Auth,
        priority: crate::component::Priority::High,
        version: "1.0.0".to_string(),
        author: "Test Author".to_string(),
        license: "MIT".to_string(),
        repository: Some("https://github.com/test/jwt-manager".to_string()),
        api_compatibility: crate::component::ApiCompatibility {
            api_version: "0.1.0".to_string(),
            min_version: "0.1.0".to_string(),
            max_version: Some("0.2.0".to_string()),
            experimental: None,
        },
        dependencies: None,
        templates: None,
        tests: None,
        docs: None,
        config_schema: None,
        features: None,
        build_config: None,
    }
}

mod component_property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_component_name_validation_property(
            name in "[a-z][a-z0-9-]{0,49}"
        ) {
            let mut manifest = create_test_manifest();
            manifest.name = name.clone();
            
            let validation_result = manifest.validate();
            if name.len() <= 50 && !name.contains("__") && !name.starts_with('-') && !name.ends_with('-') {
                prop_assert!(validation_result.is_ok());
            }
        }
        
        #[test]
        fn test_api_version_compatibility_property(
            major in 0u32..5,
            minor in 0u32..20,
            patch in 0u32..100
        ) {
            let version = format!("{}.{}.{}", major, minor, patch);
            let mut manifest = create_test_manifest();
            manifest.api_compatibility.api_version = version.clone();
            manifest.api_compatibility.min_version = version.clone();
            manifest.api_compatibility.max_version = Some(format!("{}.{}.{}", major, minor + 1, 0));
            
            prop_assert!(manifest.validate().is_ok());
            
            let status = manifest.compatibility_status(&version);
            prop_assert!(status.is_ok());
            if let Ok(status) = status {
                prop_assert!(matches!(status, CompatibilityStatus::Compatible { .. }));
            }
        }
    }
}

#[cfg(test)]
mod component_integration_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[tokio::test]
    async fn test_component_manifest_loading() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let manifest_path = temp_dir.path().join("rustform-component.yml");
        
        let manifest_content = r#"
name: test-component
description: Test component for integration testing
category: database
priority: medium
version: "1.0.0"
author: "Integration Test"
license: "MIT"
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
dependencies:
  runtime:
    - "sqlx"
  dev:
    - "testcontainers"
templates:
  - name: model
    path: "models/test_model.rs.tera"
tests:
  generate_unit_tests: true
  test_framework: "rstest"
"#;
        
        fs::write(&manifest_path, manifest_content).expect("Failed to write manifest file");
        
        let file_content = fs::read_to_string(&manifest_path).expect("Failed to read manifest file");
        let manifest: ComponentManifest = serde_yaml::from_str(&file_content).expect("Failed to parse manifest");
        
        assert_eq!(manifest.name, "test-component");
        assert!(matches!(manifest.category, crate::component::ComponentCategory::Database));
        assert!(manifest.validate().is_ok());
    }
    
    #[tokio::test]
    async fn test_component_system_list_compatibility() {
        let system = ComponentSystem::new().expect("Should create component system");
        
        // This test would normally require actual component URIs
        // For now, we just test that the method exists and can be called
        let uris = vec![];
        let result = system.list_compatible_components(&uris).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}