use cucumber::{given, then, when, World};
use tempfile::TempDir;
use std::path::PathBuf;

#[derive(Debug, Default, World)]
pub struct RustFormWorld {
    pub config_content: Option<String>,
    pub config_path: Option<PathBuf>,
    pub temp_dir: Option<TempDir>,
    pub generation_result: Option<Result<(), String>>,
    pub generated_files: Vec<PathBuf>,
}

#[given("a valid rust-form configuration")]
async fn given_valid_config(world: &mut RustFormWorld) {
    let config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: cucumber_test
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
        created_at:
          type: datetime
          auto_now_add: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#;
    
    world.config_content = Some(config.to_string());
    world.temp_dir = Some(TempDir::new().expect("Failed to create temp dir"));
}

#[given("a configuration with GDPR compliance requirements")]
async fn given_gdpr_config(world: &mut RustFormWorld) {
    let config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: gdpr_test
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
          unique: true
        first_name:
          type: string
          required: true
        last_name:
          type: string
          required: true
        consent_given:
          type: boolean
          default: false
        data_retention_expires:
          type: datetime
          nullable: true
        created_at:
          type: datetime
          auto_now_add: true
      compliance:
        gdpr:
          personal_data_fields: [email, first_name, last_name]
          consent_field: consent_given
          retention_field: data_retention_expires
          right_to_deletion: true
          right_to_portability: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
      compliance:
        gdpr:
          consent_required: true
          audit_logging: true
middleware:
  - logger: true
  - gdpr_compliance:
      audit_enabled: true
      consent_checking: true
"#;
    
    world.config_content = Some(config.to_string());
    world.temp_dir = Some(TempDir::new().expect("Failed to create temp dir"));
}

#[when("I generate the rust-form project")]
async fn when_generate_project(world: &mut RustFormWorld) {
    let config_content = world.config_content.as_ref().expect("Config content should be set");
    let temp_dir = world.temp_dir.as_ref().expect("Temp dir should be set");
    
    // Write config to file
    let config_path = temp_dir.path().join("config.yml");
    std::fs::write(&config_path, config_content).expect("Failed to write config file");
    world.config_path = Some(config_path.clone());
    
    // Parse and validate config
    let config: rustform_core::config::Config = serde_yaml::from_str(config_content)
        .map_err(|e| format!("Failed to parse config: {}", e))
        .expect("Config should parse successfully");
    
    // Validate config
    use rustform_core::config::validation::validate_config;
    validate_config(&config)
        .map_err(|e| format!("Config validation failed: {}", e))
        .expect("Config should be valid");
    
    // Generate project
    let output_dir = temp_dir.path().join("generated");
    std::fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    
    let mut pipeline = rustform_codegen::Pipeline::new().expect("Failed to create pipeline");
    
    match pipeline.generate(&config, &output_dir) {
        Ok(_) => {
            world.generation_result = Some(Ok(()));
            
            // Collect generated files
            world.generated_files = collect_generated_files(&output_dir);
        }
        Err(e) => {
            world.generation_result = Some(Err(format!("Generation failed: {}", e)));
        }
    }
}

#[then("the project should be generated successfully")]
async fn then_project_generated(world: &mut RustFormWorld) {
    assert!(world.generation_result.is_some(), "Generation should have been attempted");
    
    match &world.generation_result {
        Some(Ok(_)) => {
            // Success - check that files were generated
            assert!(!world.generated_files.is_empty(), "Should have generated files");
        }
        Some(Err(e)) => {
            panic!("Generation failed: {}", e);
        }
        None => {
            panic!("Generation was not attempted");
        }
    }
}

#[then("the generated project should compile")]
async fn then_project_compiles(world: &mut RustFormWorld) {
    let temp_dir = world.temp_dir.as_ref().expect("Temp dir should be set");
    let output_dir = temp_dir.path().join("generated");
    
    // Run cargo check on the generated project
    let output = tokio::process::Command::new("cargo")
        .arg("check")
        .current_dir(&output_dir)
        .output()
        .await
        .expect("Failed to run cargo check");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Generated project failed to compile: {}", stderr);
    }
}

#[then("GDPR compliance handlers should be generated")]
async fn then_gdpr_handlers_generated(world: &mut RustFormWorld) {
    let required_files = [
        "src/compliance/gdpr.rs",
        "src/compliance/mod.rs",
        "src/compliance/audit.rs",
    ];
    
    for file in &required_files {
        let file_path = world.generated_files.iter()
            .find(|path| path.to_string_lossy().contains(file));
        
        assert!(file_path.is_some(), "GDPR compliance file {} should be generated", file);
    }
}

#[then("the generated code should include data subject rights endpoints")]
async fn then_data_subject_rights_endpoints(world: &mut RustFormWorld) {
    let temp_dir = world.temp_dir.as_ref().expect("Temp dir should be set");
    let handlers_file = temp_dir.path().join("generated").join("src").join("handlers.rs");
    
    assert!(handlers_file.exists(), "Handlers file should exist");
    
    let content = std::fs::read_to_string(&handlers_file).expect("Failed to read handlers file");
    
    // Check for GDPR-specific endpoints
    assert!(content.contains("delete_user_data"), "Should contain data deletion endpoint");
    assert!(content.contains("export_user_data"), "Should contain data export endpoint");
    assert!(content.contains("consent"), "Should contain consent management");
}

#[then("all required files should be present")]
async fn then_required_files_present(world: &mut RustFormWorld) {
    let required_files = [
        "src/main.rs",
        "src/models.rs",
        "src/handlers.rs",
        "src/database.rs",
        "src/error.rs",
        "Cargo.toml",
        "migrations/001_initial.sql",
        ".env.example",
    ];
    
    for file in &required_files {
        let file_exists = world.generated_files.iter()
            .any(|path| path.to_string_lossy().contains(file));
        
        assert!(file_exists, "Required file {} should be generated", file);
    }
}

fn collect_generated_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_generated_files(&path));
            } else {
                files.push(path);
            }
        }
    }
    
    files
}

#[tokio::main]
async fn main() {
    RustFormWorld::run("tests/features").await;
}