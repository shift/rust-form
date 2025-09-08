use rstest::*;
use tempfile::TempDir;
use similar_asserts::assert_eq;
use crate::codegen::{CodegenEngine, CodegenContext, Pipeline};
use std::collections::HashMap;

#[rstest]
fn test_codegen_engine_creation() {
    let engine = CodegenEngine::new().expect("Should create codegen engine");
    assert!(engine.template_exists("main.rs.tera"));
    assert!(engine.template_exists("Cargo.toml.tera"));
}

#[rstest]
fn test_template_rendering() {
    let engine = CodegenEngine::new().expect("Should create codegen engine");
    
    let mut context = CodegenContext::new();
    context.insert("project_name", "test_project");
    context.insert("version", "1.0.0");
    
    let result = engine.render_template("main.rs.tera", &context);
    assert!(result.is_ok());
    
    let rendered = result.unwrap();
    assert!(rendered.contains("test_project"));
    assert!(rendered.contains("1.0.0"));
}

#[rstest]
fn test_pipeline_execution() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    
    let config_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: pipeline_test
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

    let config: crate::config::Config = serde_yaml::from_str(config_yaml).expect("Failed to parse config");
    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    
    let result = pipeline.generate(&config, output_dir);
    assert!(result.is_ok());
    
    // Verify generated files exist
    assert!(output_dir.join("src").join("main.rs").exists());
    assert!(output_dir.join("Cargo.toml").exists());
    assert!(output_dir.join("src").join("models.rs").exists());
}

#[rstest]
#[tokio::test]
async fn test_generated_project_compilation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    
    let config_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: compilation_test
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
        email:
          type: string
          required: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
"#;

    let config: crate::config::Config = serde_yaml::from_str(config_yaml).expect("Failed to parse config");
    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    
    pipeline.generate(&config, output_dir).expect("Should generate project");
    
    // Test that generated project can be compiled
    let compilation_result = tokio::process::Command::new("cargo")
        .arg("check")
        .current_dir(output_dir)
        .output()
        .await;
        
    assert!(compilation_result.is_ok());
    let output = compilation_result.unwrap();
    assert!(output.status.success(), "Generated project should compile successfully");
}

mod codegen_property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_project_name_generation_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}"
        ) {
            let engine = CodegenEngine::new().expect("Should create engine");
            let mut context = CodegenContext::new();
            context.insert("project_name", &name);
            context.insert("version", "1.0.0");
            
            let result = engine.render_template("Cargo.toml.tera", &context);
            prop_assert!(result.is_ok());
            
            let rendered = result.unwrap();
            prop_assert!(rendered.contains(&name));
        }
        
        #[test]
        fn test_model_generation_property(
            model_name in "[A-Z][a-zA-Z0-9]{0,49}",
            table_name in "[a-z][a-z0-9_]{0,49}"
        ) {
            let engine = CodegenEngine::new().expect("Should create engine");
            let mut context = CodegenContext::new();
            
            let mut models = HashMap::new();
            let mut model_config = HashMap::new();
            model_config.insert("table_name", table_name.clone());
            models.insert(model_name.clone(), model_config);
            
            context.insert("models", &models);
            
            let result = engine.render_template("models.rs.tera", &context);
            prop_assert!(result.is_ok());
            
            let rendered = result.unwrap();
            prop_assert!(rendered.contains(&model_name));
            prop_assert!(rendered.contains(&table_name));
        }
    }
}

#[cfg(test)]
mod codegen_integration_tests {
    use super::*;
    use std::fs;
    
    #[tokio::test]
    async fn test_end_to_end_generation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let output_dir = temp_dir.path();
        
        // Use the example ecommerce config
        let config_path = "examples/ecommerce.yml";
        let config_content = fs::read_to_string(config_path).expect("Failed to read example config");
        let mut config: crate::config::Config = serde_yaml::from_str(&config_content).expect("Failed to parse config");
        
        // Update with versioning fields for testing
        config.schema_version = "1.0.0".to_string();
        config.api_version = "0.1.0".to_string();
        
        let mut pipeline = Pipeline::new().expect("Should create pipeline");
        let result = pipeline.generate(&config, output_dir);
        
        assert!(result.is_ok());
        
        // Verify all expected files are generated
        let expected_files = [
            "src/main.rs",
            "src/models.rs", 
            "src/handlers.rs",
            "src/database.rs",
            "src/error.rs",
            "Cargo.toml",
            "migrations/001_initial.sql",
            ".env.example"
        ];
        
        for file in &expected_files {
            let file_path = output_dir.join(file);
            assert!(file_path.exists(), "Expected file {} should exist", file);
            
            let content = fs::read_to_string(&file_path).expect("Should read generated file");
            assert!(!content.is_empty(), "Generated file {} should not be empty", file);
        }
        
        // Test specific content in generated files
        let main_content = fs::read_to_string(output_dir.join("src/main.rs")).expect("Should read main.rs");
        assert!(main_content.contains("ecommerce_api"));
        assert!(main_content.contains("axum"));
        
        let models_content = fs::read_to_string(output_dir.join("src/models.rs")).expect("Should read models.rs");
        assert!(models_content.contains("User"));
        assert!(models_content.contains("Product"));
        assert!(models_content.contains("Order"));
    }
    
    #[test]
    fn test_template_validation() {
        let engine = CodegenEngine::new().expect("Should create engine");
        
        // Test that all required templates exist
        let required_templates = [
            "main.rs.tera",
            "Cargo.toml.tera", 
            "models.rs.tera",
            "handlers.rs.tera",
            "database.rs.tera",
            "error.rs.tera"
        ];
        
        for template in &required_templates {
            assert!(engine.template_exists(template), "Required template {} should exist", template);
        }
    }
    
    #[test]
    fn test_context_building() {
        let config_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: context_test
version: "1.0.0"
database:
  type: postgres
  url_env: DATABASE_URL
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
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#;

        let config: crate::config::Config = serde_yaml::from_str(config_yaml).expect("Failed to parse config");
        let context = CodegenContext::from_config(&config).expect("Should build context from config");
        
        // Verify context contains expected data
        assert!(context.contains_key("project_name"));
        assert!(context.contains_key("version"));
        assert!(context.contains_key("database"));
        assert!(context.contains_key("models"));
        assert!(context.contains_key("endpoints"));
        assert!(context.contains_key("middleware"));
        
        // Verify specific values
        assert_eq!(context.get("project_name").unwrap().as_str().unwrap(), "context_test");
        assert_eq!(context.get("version").unwrap().as_str().unwrap(), "1.0.0");
    }
}