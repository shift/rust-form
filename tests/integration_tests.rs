// Integration tests for the complete generation pipeline
use rustform_cli::commands::generate::GenerateCommand;
use rustform_core::config::{Config, validation::validate_config};
use rustform_codegen::pipeline::Pipeline;
use rstest::*;
use similar_asserts::assert_eq;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tokio::process::Command;

#[rstest]
#[tokio::test]
async fn test_complete_todo_generation_pipeline() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("todo_app");

    let config_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: todo_app
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
server:
  port: 8080
api:
  models:
    Todo:
      table_name: todos
      fields:
        id:
          type: uuid
          primary_key: true
          default: "gen_random_uuid()"
        title:
          type: string
          required: true
          validation:
            min_length: 1
            max_length: 200
        description:
          type: text
          optional: true
        completed:
          type: boolean
          default: false
        created_at:
          type: datetime
          default: "now()"
  endpoints:
    - path: /todos
      model: Todo
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

    // Parse and validate configuration
    let config: Config = serde_yaml::from_str(config_yaml).expect("Failed to parse config");
    validate_config(&config).expect("Config validation should pass");

    // Generate the project using the pipeline
    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    pipeline.generate(&config, &output_dir).expect("Generation should succeed");

    // Verify generated files exist
    let expected_files = vec![
        "src/main.rs",
        "src/models.rs",
        "src/handlers.rs",
        "src/database.rs",
        "src/error.rs",
        "Cargo.toml",
        "migrations/001_initial.sql",
        ".env.example",
    ];

    for file in &expected_files {
        let file_path = output_dir.join(file);
        assert!(file_path.exists(), "Expected file {} should exist", file);

        let content = fs::read_to_string(&file_path).expect("Should read generated file");
        assert!(!content.is_empty(), "Generated file {} should not be empty", file);
    }

    // Verify Cargo.toml contains correct project name and dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).expect("Should read Cargo.toml");
    assert!(cargo_content.contains("name = \"todo_app\""));
    assert!(cargo_content.contains("axum"));
    assert!(cargo_content.contains("sqlx"));
    assert!(cargo_content.contains("tokio"));

    // Verify main.rs contains correct setup
    let main_content = fs::read_to_string(output_dir.join("src/main.rs")).expect("Should read main.rs");
    assert!(main_content.contains("use axum"));
    assert!(main_content.contains("8080"));
    assert!(main_content.contains("todo_app"));

    // Verify models.rs contains Todo struct
    let models_content = fs::read_to_string(output_dir.join("src/models.rs")).expect("Should read models.rs");
    assert!(models_content.contains("struct Todo"));
    assert!(models_content.contains("id: Uuid"));
    assert!(models_content.contains("title: String"));
    assert!(models_content.contains("completed: bool"));

    // Verify handlers.rs contains CRUD endpoints
    let handlers_content = fs::read_to_string(output_dir.join("src/handlers.rs")).expect("Should read handlers.rs");
    assert!(handlers_content.contains("create_todo"));
    assert!(handlers_content.contains("get_todos"));
    assert!(handlers_content.contains("get_todo"));
    assert!(handlers_content.contains("update_todo"));
    assert!(handlers_content.contains("delete_todo"));

    // Verify migration file contains correct SQL
    let migration_content = fs::read_to_string(output_dir.join("migrations/001_initial.sql")).expect("Should read migration");
    assert!(migration_content.contains("CREATE TABLE todos"));
    assert!(migration_content.contains("id UUID PRIMARY KEY"));
    assert!(migration_content.contains("title VARCHAR NOT NULL"));
}

#[rstest]
#[tokio::test]
async fn test_ecommerce_generation_pipeline() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("ecommerce_api");

    let config_yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: ecommerce_api
version: "1.0.0"
database:
  type: postgres
  url_env: DATABASE_URL
  pool_size: 20
server:
  port: 3000
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
          unique: true
          required: true
        password_hash:
          type: string
          required: true
        created_at:
          type: datetime
          default: "now()"
    Product:
      table_name: products
      fields:
        id:
          type: uuid
          primary_key: true
        name:
          type: string
          required: true
        description:
          type: text
        price:
          type: decimal
          required: true
        stock:
          type: integer
          default: 0
    Order:
      table_name: orders
      fields:
        id:
          type: uuid
          primary_key: true
        user_id:
          type: uuid
          foreign_key:
            table: users
            column: id
        total:
          type: decimal
          required: true
        status:
          type: string
          default: "pending"
        created_at:
          type: datetime
          default: "now()"
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
    - path: /products
      model: Product
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
    - path: /orders
      model: Order
      crud:
        create: true
        read_all: true
        read_one: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#;

    let config: Config = serde_yaml::from_str(config_yaml).expect("Failed to parse ecommerce config");
    validate_config(&config).expect("Ecommerce config validation should pass");

    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    pipeline.generate(&config, &output_dir).expect("Ecommerce generation should succeed");

    // Verify all models are generated
    let models_content = fs::read_to_string(output_dir.join("src/models.rs")).expect("Should read models.rs");
    assert!(models_content.contains("struct User"));
    assert!(models_content.contains("struct Product"));
    assert!(models_content.contains("struct Order"));

    // Verify foreign key relationship
    assert!(models_content.contains("user_id: Uuid"));

    // Verify handlers for all models
    let handlers_content = fs::read_to_string(output_dir.join("src/handlers.rs")).expect("Should read handlers.rs");
    assert!(handlers_content.contains("create_user"));
    assert!(handlers_content.contains("create_product"));
    assert!(handlers_content.contains("create_order"));

    // Verify PostgreSQL specific content
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).expect("Should read Cargo.toml");
    assert!(cargo_content.contains("postgres"));

    // Verify migration includes all tables
    let migration_content = fs::read_to_string(output_dir.join("migrations/001_initial.sql")).expect("Should read migration");
    assert!(migration_content.contains("CREATE TABLE users"));
    assert!(migration_content.contains("CREATE TABLE products"));
    assert!(migration_content.contains("CREATE TABLE orders"));
    assert!(migration_content.contains("FOREIGN KEY"));
}

#[rstest]
#[tokio::test]
async fn test_generated_project_compilation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("compilation_test");

    let minimal_config = r#"
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

    let config: Config = serde_yaml::from_str(minimal_config).expect("Failed to parse minimal config");
    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    pipeline.generate(&config, &output_dir).expect("Minimal generation should succeed");

    // Test that the generated project compiles
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(&output_dir)
        .output()
        .await
        .expect("Should run cargo check");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "Generated project failed to compile:\nSTDOUT:\n{}\nSTDERR:\n{}",
            stdout, stderr
        );
    }
}

#[rstest]
#[tokio::test]
async fn test_validation_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("validation_test");

    let config_with_validation = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: validation_test
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
          type: uuid
          primary_key: true
        email:
          type: string
          required: true
          unique: true
          validation:
            email: true
        age:
          type: integer
          validation:
            min: 0
            max: 150
        username:
          type: string
          required: true
          validation:
            min_length: 3
            max_length: 50
            pattern: "^[a-zA-Z0-9_]+$"
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
"#;

    let config: Config = serde_yaml::from_str(config_with_validation).expect("Failed to parse validation config");
    validate_config(&config).expect("Validation config should pass validation");

    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    pipeline.generate(&config, &output_dir).expect("Validation generation should succeed");

    // Verify validation logic is included
    let models_content = fs::read_to_string(output_dir.join("src/models.rs")).expect("Should read models.rs");
    assert!(models_content.contains("validator")); // Should include validator derives or functions

    let handlers_content = fs::read_to_string(output_dir.join("src/handlers.rs")).expect("Should read handlers.rs");
    // Should include validation calls in handlers
    assert!(handlers_content.contains("validate") || handlers_content.contains("Validate"));
}

#[rstest]
#[tokio::test]
async fn test_middleware_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("middleware_test");

    let config_with_middleware = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: middleware_test
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    Post:
      table_name: posts
      fields:
        id:
          type: uuid
          primary_key: true
        title:
          type: string
          required: true
  endpoints:
    - path: /posts
      model: Post
      crud:
        create: true
        read_all: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
      allow_methods: ["GET", "POST", "PUT", "DELETE"]
      allow_headers: ["Content-Type", "Authorization"]
  - rate_limit:
      requests_per_minute: 100
"#;

    let config: Config = serde_yaml::from_str(config_with_middleware).expect("Failed to parse middleware config");
    validate_config(&config).expect("Middleware config should pass validation");

    let mut pipeline = Pipeline::new().expect("Should create pipeline");
    pipeline.generate(&config, &output_dir).expect("Middleware generation should succeed");

    // Verify middleware setup in main.rs
    let main_content = fs::read_to_string(output_dir.join("src/main.rs")).expect("Should read main.rs");
    assert!(main_content.contains("tower_http::trace")); // Logger middleware
    assert!(main_content.contains("tower_http::cors")); // CORS middleware
    
    // Verify CORS configuration
    assert!(main_content.contains("allow_origin"));
    assert!(main_content.contains("allow_methods"));
}

#[rstest]
#[tokio::test]
async fn test_error_handling_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Test with invalid configuration
    let invalid_config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: invalid_test
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    User:
      table_name: users
      fields:
        # Missing required primary key
        name:
          type: string
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    let config_result: Result<Config, _> = serde_yaml::from_str(invalid_config);
    assert!(config_result.is_ok()); // YAML parsing should succeed

    let config = config_result.unwrap();
    let validation_result = validate_config(&config);
    assert!(validation_result.is_err()); // But validation should fail

    // Test with non-existent model reference
    let invalid_endpoint_config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: invalid_endpoint_test
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
    - path: /posts
      model: Post  # This model doesn't exist
      crud:
        create: true
"#;

    let config: Config = serde_yaml::from_str(invalid_endpoint_config).expect("Should parse YAML");
    let validation_result = validate_config(&config);
    assert!(validation_result.is_err()); // Should fail due to missing model
}

mod pipeline_property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[ignore] // These tests take longer to run
        fn test_project_name_generation_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{2,30}"
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let temp_dir = TempDir::new().expect("Failed to create temp dir");
                let output_dir = temp_dir.path().join(&name);

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

                let config: Config = serde_yaml::from_str(&config_yaml).expect("Should parse config");
                prop_assert!(validate_config(&config).is_ok());

                let mut pipeline = Pipeline::new().expect("Should create pipeline");
                prop_assert!(pipeline.generate(&config, &output_dir).is_ok());

                // Verify project name appears in generated files
                let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).expect("Should read Cargo.toml");
                prop_assert!(cargo_content.contains(&format!("name = \"{}\"", name)));
            });
        }
    }
}

#[cfg(test)]
mod integration_performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_generation_performance() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().join("performance_test");

        // Create a moderately complex configuration
        let complex_config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: performance_test
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
        id: { type: uuid, primary_key: true }
        email: { type: string, unique: true }
        name: { type: string }
        created_at: { type: datetime, default: "now()" }
    Post:
      table_name: posts
      fields:
        id: { type: uuid, primary_key: true }
        title: { type: string }
        content: { type: text }
        author_id: { type: uuid, foreign_key: { table: users, column: id } }
        published: { type: boolean, default: false }
        created_at: { type: datetime, default: "now()" }
    Comment:
      table_name: comments
      fields:
        id: { type: uuid, primary_key: true }
        content: { type: text }
        post_id: { type: uuid, foreign_key: { table: posts, column: id } }
        author_id: { type: uuid, foreign_key: { table: users, column: id } }
        created_at: { type: datetime, default: "now()" }
  endpoints:
    - path: /users
      model: User
      crud: { create: true, read_all: true, read_one: true, update: true, delete: true }
    - path: /posts
      model: Post
      crud: { create: true, read_all: true, read_one: true, update: true, delete: true }
    - path: /comments
      model: Comment
      crud: { create: true, read_all: true, read_one: true, update: true, delete: true }
middleware:
  - logger: true
  - cors: { allow_origin: "*" }
"#;

        let config: Config = serde_yaml::from_str(complex_config).expect("Should parse complex config");
        
        let start = Instant::now();
        let mut pipeline = Pipeline::new().expect("Should create pipeline");
        pipeline.generate(&config, &output_dir).expect("Complex generation should succeed");
        let duration = start.elapsed();

        // Generation should complete in reasonable time (less than 10 seconds)
        assert!(duration.as_secs() < 10, "Generation took too long: {:?}", duration);

        // Verify all files were generated
        let expected_files = ["src/main.rs", "src/models.rs", "src/handlers.rs", "Cargo.toml"];
        for file in &expected_files {
            assert!(output_dir.join(file).exists(), "File {} should be generated", file);
        }
    }
}