#[cfg(test)]
mod component_system_tests {
    use crate::GenerationPipeline;
    use rustform_core::{Config, component::{ComponentSystem, ComponentUri}};
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_component_installation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let component_path = temp_dir.path().join("test-component");
        fs::create_dir_all(&component_path).expect("Failed to create component dir");
        
        // Create a test component manifest
        let manifest_content = r#"
name: test-component
version: "1.0.0"
description: "Test component"
keywords: []
dependencies: {}
files: []

provides:
  templates:
    - name: "test.tera"
      path: "test.tera"
      description: "Test template"
      variables: []
      target: Frontend
  assets: []
  hooks: []
"#;
        
        fs::write(component_path.join("rustform-component.yml"), manifest_content)
            .expect("Failed to write manifest");
        
        fs::write(component_path.join("test.tera"), "Hello {{ name }}!")
            .expect("Failed to write template");
        
        // Test component installation
        let mut component_system = ComponentSystem::new().expect("Failed to create component system");
        let uri: ComponentUri = format!("path:{}", component_path.display()).parse()
            .expect("Failed to parse URI");
        
        let component = component_system.install_component(&uri).await
            .expect("Failed to install component");
        
        assert_eq!(component.manifest.name, "test-component");
        assert_eq!(component.manifest.version, "1.0.0");
        assert_eq!(component.manifest.provides.templates.len(), 1);
    }

    #[tokio::test]
    async fn test_component_validation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let component_path = temp_dir.path().join("invalid-component");
        fs::create_dir_all(&component_path).expect("Failed to create component dir");
        
        // Create invalid manifest
        let invalid_manifest = "invalid: yaml: content:";
        fs::write(component_path.join("rustform-component.yml"), invalid_manifest)
            .expect("Failed to write manifest");
        
        let component_system = ComponentSystem::new().expect("Failed to create component system");
        let uri: ComponentUri = format!("path:{}", component_path.display()).parse()
            .expect("Failed to parse URI");
        
        let result = component_system.fetch_manifest(&uri).await;
        assert!(result.is_err());
    }

    #[tokio::test] 
    async fn test_basic_pipeline_without_components() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().join("output");
        
        // Create basic test config without components
        let config_content = r#"
project_name: test-project
version: "0.1.0"

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
          auto_increment: true
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
        
        let config_path = temp_dir.path().join("test-config.yml");
        fs::write(&config_path, config_content).expect("Failed to write config");
        
        // Load config via YAML parser (since from_file doesn't exist)
        let config_str = fs::read_to_string(&config_path).expect("Failed to read config");
        let config: Config = serde_yaml::from_str(&config_str).expect("Failed to parse config");
        
        // Test generation
        let mut pipeline = GenerationPipeline::new()
            .expect("Failed to create pipeline");
        
        let result = pipeline.generate(&config, &output_dir).await;
        assert!(result.is_ok());
        
        let project = result.unwrap();
        assert_eq!(project.name, "test-project");
        assert!(!project.files.is_empty());
    }
}