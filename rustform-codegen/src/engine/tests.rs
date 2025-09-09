use crate::context::{DatabaseContext, GenerationContext, ModelContext, FieldContext, ServerContext, ProjectFeatures, ProjectMetadata};
use crate::engine::TemplateEngine;
use crate::error::CodeGenError;
use rstest::*;
use similar_asserts::assert_eq;
use std::collections::HashMap;
use tempfile::TempDir;

#[rstest]
fn test_template_engine_creation() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test that built-in templates are loaded
    assert!(engine.has_template("main.rs.tera"));
    assert!(engine.has_template("Cargo.toml.tera"));
    assert!(engine.has_template("models.rs.tera"));
    assert!(engine.has_template("handlers.rs.tera"));
}

#[rstest]
fn test_generation_context_creation() {
    let context = GenerationContext {
        project_name: "test_project".to_string(),
        version: "1.0.0".to_string(),
        database: DatabaseContext {
            db_type: "sqlite".to_string(),
            url_env: "DATABASE_URL".to_string(),
            pool_size: Some(10),
            timeout: Some(30),
            driver_features: vec!["runtime-tokio-rustls".to_string()],
        },
        server: ServerContext {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        models: vec![],
        endpoints: vec![],
        middleware: vec![],
        dependencies: vec!["serde".to_string(), "tokio".to_string()],
        features: ProjectFeatures {
            auth: false,
            logging: true,
            cors: true,
            testing: true,
            docs: false,
            metrics: false,
            validation: true,
            migration: true,
            frontend_types: false,
        },
        metadata: ProjectMetadata {
            author: Some("Test Author".to_string()),
            description: Some("Test project description".to_string()),
            license: Some("MIT".to_string()),
            repository: Some("https://github.com/test/test-project".to_string()),
            homepage: None,
            keywords: vec!["api".to_string(), "rust".to_string()],
            categories: vec!["web-programming".to_string()],
            readme: Some("README.md".to_string()),
            documentation: None,
        },
    };

    assert_eq!(context.project_name, "test_project");
    assert_eq!(context.database.db_type, "sqlite");
    assert_eq!(context.server.port, 8080);
    assert!(context.features.logging);
    assert!(!context.features.auth);
}

#[rstest]
fn test_model_context_creation() {
    let model = ModelContext {
        name: "User".to_string(),
        table_name: "users".to_string(),
        struct_name: "User".to_string(),
        fields: vec![
            FieldContext {
                name: "id".to_string(),
                field_type: "uuid".to_string(),
                rust_type: "Uuid".to_string(),
                sql_type: "UUID".to_string(),
                nullable: false,
                primary_key: true,
                unique: false,
                default_value: Some("gen_random_uuid()".to_string()),
                validation: None,
                foreign_key: None,
                indexed: false,
                description: None,
                example: None,
            },
            FieldContext {
                name: "email".to_string(),
                field_type: "string".to_string(),
                rust_type: "String".to_string(),
                sql_type: "VARCHAR".to_string(),
                nullable: false,
                primary_key: false,
                unique: true,
                default_value: None,
                validation: None,
                foreign_key: None,
                indexed: true,
                description: Some("User email address".to_string()),
                example: Some("user@example.com".to_string()),
            },
        ],
        relationships: vec![],
        indexes: vec![],
        has_primary_key: true,
        primary_key_field: Some("id".to_string()),
        primary_key_type: Some("uuid".to_string()),
        has_timestamps: false,
        imports: vec!["uuid::Uuid".to_string()],
        custom_logic: None,
    };

    assert_eq!(model.name, "User");
    assert_eq!(model.table_name, "users");
    assert_eq!(model.fields.len(), 2);
    assert!(model.has_primary_key);
    assert_eq!(model.primary_key_field, Some("id".to_string()));
}

#[rstest]
fn test_field_context_validation() {
    let field = FieldContext {
        name: "age".to_string(),
        field_type: "integer".to_string(),
        rust_type: "i32".to_string(),
        sql_type: "INTEGER".to_string(),
        nullable: false,
        primary_key: false,
        unique: false,
        default_value: None,
        validation: None,
        foreign_key: None,
        indexed: false,
        description: Some("User age in years".to_string()),
        example: Some("25".to_string()),
    };

    assert_eq!(field.name, "age");
    assert_eq!(field.rust_type, "i32");
    assert_eq!(field.sql_type, "INTEGER");
    assert!(!field.nullable);
    assert!(!field.primary_key);
}

#[rstest]
fn test_template_rendering_with_context() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    let context = create_test_context();
    let tera_context = engine.build_template_context(&context).expect("Should build context");

    // Test rendering a basic template
    let result = engine.render_template("Cargo.toml.tera", &tera_context);
    assert!(result.is_ok());

    let rendered = result.unwrap();
    assert!(rendered.contains("test_project"));
    assert!(rendered.contains("1.0.0"));
}

#[rstest]
fn test_template_filters() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test snake_case filter
    let test_template = "{{ name | snake_case }}";
    let mut context = tera::Context::new();
    context.insert("name", "TestProject");
    
    let engine_tera = engine.get_tera();
    let result = engine_tera.render_str(test_template, &context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_project");
}

#[rstest]
fn test_pascal_case_filter() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    let test_template = "{{ name | pascal_case }}";
    let mut context = tera::Context::new();
    context.insert("name", "test_project");
    
    let engine_tera = engine.get_tera();
    let result = engine_tera.render_str(test_template, &context);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "TestProject");
}

#[rstest]
fn test_template_error_handling() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test with non-existent template
    let context = tera::Context::new();
    let result = engine.render_template("non_existent.tera", &context);
    assert!(result.is_err());
    
    if let Err(CodeGenError::Template(msg)) = result {
        assert!(msg.contains("non_existent.tera"));
    } else {
        panic!("Expected Template error");
    }
}

#[rstest]
fn test_component_uri_parsing() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test parsing valid component URIs
    let valid_uris = vec![
        "github://owner/repo@1.0.0",
        "file://./components/auth",
        "registry://auth/jwt-manager@latest",
    ];

    for uri_str in valid_uris {
        let result = engine.parse_component_uri(uri_str);
        assert!(result.is_ok(), "Failed to parse URI: {}", uri_str);
    }
}

#[rstest]
fn test_invalid_component_uri() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test parsing invalid component URIs
    let invalid_uris = vec![
        "invalid-uri",
        "http://example.com",  // Wrong protocol
        "github://",           // Missing repo
    ];

    for uri_str in invalid_uris {
        let result = engine.parse_component_uri(uri_str);
        assert!(result.is_err(), "Should fail to parse invalid URI: {}", uri_str);
    }
}

#[rstest]
#[tokio::test]
async fn test_component_system_integration() {
    let engine = TemplateEngine::new().expect("Should create template engine");
    
    // Test component system initialization
    let system = engine.get_component_system();
    assert_eq!(system.rust_form_version(), env!("CARGO_PKG_VERSION"));
}

fn create_test_context() -> GenerationContext {
    GenerationContext {
        project_name: "test_project".to_string(),
        version: "1.0.0".to_string(),
        database: DatabaseContext {
            db_type: "sqlite".to_string(),
            url_env: "DATABASE_URL".to_string(),
            pool_size: Some(10),
            timeout: Some(30),
            driver_features: vec!["runtime-tokio-rustls".to_string()],
        },
        server: ServerContext {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        models: vec![
            ModelContext {
                name: "User".to_string(),
                table_name: "users".to_string(),
                struct_name: "User".to_string(),
                fields: vec![
                    FieldContext {
                        name: "id".to_string(),
                        field_type: "integer".to_string(),
                        rust_type: "i32".to_string(),
                        sql_type: "INTEGER".to_string(),
                        nullable: false,
                        primary_key: true,
                        unique: false,
                        default_value: None,
                        validation: None,
                        foreign_key: None,
                        indexed: false,
                        description: None,
                        example: None,
                    },
                ],
                relationships: vec![],
                indexes: vec![],
                has_primary_key: true,
                primary_key_field: Some("id".to_string()),
                primary_key_type: Some("integer".to_string()),
                has_timestamps: false,
                imports: vec![],
                custom_logic: None,
            },
        ],
        endpoints: vec![],
        middleware: vec![],
        dependencies: vec!["serde".to_string()],
        features: ProjectFeatures {
            auth: false,
            logging: true,
            cors: true,
            testing: true,
            docs: false,
            metrics: false,
            validation: true,
            migration: true,
            frontend_types: false,
        },
        metadata: ProjectMetadata {
            author: Some("Test Author".to_string()),
            description: Some("Test project".to_string()),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            keywords: vec![],
            categories: vec![],
            readme: None,
            documentation: None,
        },
    }
}

mod engine_property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_project_name_filtering_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}"
        ) {
            let engine = TemplateEngine::new().expect("Should create engine");
            
            let test_template = "{{ name | snake_case }}";
            let mut context = tera::Context::new();
            context.insert("name", &name);
            
            let engine_tera = engine.get_tera();
            let result = engine_tera.render_str(test_template, &context);
            prop_assert!(result.is_ok());
            
            let rendered = result.unwrap();
            // Snake case should be lowercase with underscores
            prop_assert!(rendered.chars().all(|c| c.is_lowercase() || c == '_' || c.is_ascii_digit()));
        }

        #[test]
        fn test_pascal_case_filtering_property(
            name in "[a-z][a-z0-9_]{0,49}"
        ) {
            let engine = TemplateEngine::new().expect("Should create engine");
            
            let test_template = "{{ name | pascal_case }}";
            let mut context = tera::Context::new();
            context.insert("name", &name);
            
            let engine_tera = engine.get_tera();
            let result = engine_tera.render_str(test_template, &context);
            prop_assert!(result.is_ok());
            
            let rendered = result.unwrap();
            // Pascal case should start with uppercase
            if !rendered.is_empty() {
                prop_assert!(rendered.chars().next().unwrap().is_uppercase());
            }
        }
    }
}

#[cfg(test)]
mod engine_integration_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_full_template_generation() {
        let engine = TemplateEngine::new().expect("Should create template engine");
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        
        let context = create_test_context();
        let tera_context = engine.build_template_context(&context).expect("Should build context");

        // Test generating all main templates
        let templates = vec![
            "main.rs.tera",
            "Cargo.toml.tera", 
            "models.rs.tera",
            "handlers.rs.tera",
            "database.rs.tera",
            "error.rs.tera",
        ];

        for template_name in templates {
            let result = engine.render_template(template_name, &tera_context);
            assert!(result.is_ok(), "Failed to render template: {}", template_name);
            
            let rendered = result.unwrap();
            assert!(!rendered.is_empty(), "Template {} generated empty content", template_name);
            
            // Write to file to verify content
            let file_name = template_name.replace(".tera", "");
            let file_path = temp_dir.path().join(&file_name);
            fs::write(&file_path, &rendered).expect("Should write rendered template");
            
            // Verify file was written
            assert!(file_path.exists(), "Generated file {} should exist", file_name);
            let file_content = fs::read_to_string(&file_path).expect("Should read generated file");
            assert_eq!(file_content, rendered);
        }
    }

    #[test]
    fn test_context_serialization_roundtrip() {
        let context = create_test_context();
        
        // Test JSON serialization
        let json = serde_json::to_string(&context).expect("Should serialize to JSON");
        let deserialized: GenerationContext = serde_json::from_str(&json).expect("Should deserialize from JSON");
        
        assert_eq!(context.project_name, deserialized.project_name);
        assert_eq!(context.version, deserialized.version);
        assert_eq!(context.database.db_type, deserialized.database.db_type);
        assert_eq!(context.models.len(), deserialized.models.len());
        
        // Test YAML serialization
        let yaml = serde_yaml::to_string(&context).expect("Should serialize to YAML");
        let deserialized_yaml: GenerationContext = serde_yaml::from_str(&yaml).expect("Should deserialize from YAML");
        
        assert_eq!(context.project_name, deserialized_yaml.project_name);
        assert_eq!(context.features.logging, deserialized_yaml.features.logging);
    }
}