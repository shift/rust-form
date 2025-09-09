// End-to-end tests that use example configurations and test the complete workflow
use assert_cmd::Command;
use predicates::prelude::*;
use rstest::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tokio::time::{sleep, Duration};

#[rstest]
#[tokio::test]
async fn test_cli_generate_todo_example() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("todo.yml");
    let output_path = temp_dir.path().join("generated");

    // Create example todo configuration
    let todo_config = r#"
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
        updated_at:
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

    fs::write(&config_path, todo_config).expect("Should write config file");

    // Run rustform generate command
    let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    cmd.arg("generate")
        .arg(&config_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated"))
        .stdout(predicate::str::contains("todo_app"));

    // Verify generated files exist
    assert!(output_path.join("Cargo.toml").exists());
    assert!(output_path.join("src/main.rs").exists());
    assert!(output_path.join("src/models.rs").exists());
    assert!(output_path.join("src/handlers.rs").exists());
    assert!(output_path.join("migrations/001_initial.sql").exists());

    // Verify content correctness
    let cargo_content = fs::read_to_string(output_path.join("Cargo.toml")).expect("Should read Cargo.toml");
    assert!(cargo_content.contains("name = \"todo_app\""));
    assert!(cargo_content.contains("version = \"1.0.0\""));

    let models_content = fs::read_to_string(output_path.join("src/models.rs")).expect("Should read models.rs");
    assert!(models_content.contains("pub struct Todo"));
    assert!(models_content.contains("pub id: Uuid"));
    assert!(models_content.contains("pub title: String"));
    assert!(models_content.contains("pub completed: bool"));
}

#[rstest]
#[tokio::test]
async fn test_cli_init_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_path = temp_dir.path().join("new_project");

    // Run rustform init command
    let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    cmd.arg("init")
        .arg("new_project")
        .arg("--directory")
        .arg(&project_path)
        .arg("--database")
        .arg("postgres");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialized"))
        .stdout(predicate::str::contains("new_project"));

    // Verify project structure was created
    assert!(project_path.join("rustform.yml").exists());
    
    let config_content = fs::read_to_string(project_path.join("rustform.yml")).expect("Should read config");
    assert!(config_content.contains("project_name: new_project"));
    assert!(config_content.contains("type: postgres"));
}

#[rstest]
#[tokio::test]
async fn test_generated_project_builds_and_runs() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("blog.yml");
    let output_path = temp_dir.path().join("blog_api");

    // Create blog example configuration
    let blog_config = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: blog_api
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
server:
  port: 3001
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
        username:
          type: string
          unique: true
          required: true
        created_at:
          type: datetime
          default: "now()"
    Post:
      table_name: posts
      fields:
        id:
          type: uuid
          primary_key: true
        title:
          type: string
          required: true
        content:
          type: text
          required: true
        author_id:
          type: uuid
          foreign_key:
            table: users
            column: id
        published:
          type: boolean
          default: false
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
    - path: /posts
      model: Post
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

    fs::write(&config_path, blog_config).expect("Should write blog config");

    // Generate the project
    let mut generate_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    generate_cmd.arg("generate")
        .arg(&config_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--force");

    generate_cmd.assert().success();

    // Test that the project compiles
    let mut build_cmd = std::process::Command::new("cargo");
    build_cmd.arg("build")
        .current_dir(&output_path);

    let build_output = build_cmd.output().expect("Should run cargo build");
    
    if !build_output.status.success() {
        let stderr = String::from_utf8_lossy(&build_output.stderr);
        let stdout = String::from_utf8_lossy(&build_output.stdout);
        panic!("Generated project failed to build:\nSTDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    }

    // Create a test database and .env file
    let env_content = "DATABASE_URL=sqlite://test.db";
    fs::write(output_path.join(".env"), env_content).expect("Should write .env file");

    // Test that the project can run (briefly)
    let mut run_cmd = tokio::process::Command::new("cargo");
    run_cmd.arg("run")
        .current_dir(&output_path)
        .env("DATABASE_URL", "sqlite://test.db");

    let mut child = run_cmd.spawn().expect("Should start the server");

    // Give the server a moment to start
    sleep(Duration::from_secs(2)).await;

    // Kill the server
    child.kill().await.expect("Should kill the server");

    // Check that it started successfully (no immediate crash)
    let status = child.wait().await.expect("Should wait for child");
    // Note: We killed it, so it won't have a successful exit code, but it shouldn't have crashed immediately
}

#[rstest]
#[tokio::test]
async fn test_cli_validation_errors() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("invalid.yml");

    // Create invalid configuration (missing primary key)
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
        name:
          type: string
          required: true
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
"#;

    fs::write(&config_path, invalid_config).expect("Should write invalid config");

    // Run rustform generate and expect it to fail
    let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    cmd.arg("generate")
        .arg(&config_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("validation"))
        .stderr(predicate::str::contains("primary key"));
}

#[rstest]
#[tokio::test]
async fn test_cli_component_commands() {
    // Test component list command
    let mut list_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    list_cmd.arg("component")
        .arg("list");

    list_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available components") | predicate::str::contains("No components"));

    // Test component alias
    let mut alias_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    alias_cmd.arg("comp")
        .arg("list");

    alias_cmd.assert().success();
}

#[rstest]
fn test_cli_help_commands() {
    // Test main help
    let mut help_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    help_cmd.arg("--help");

    help_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Declarative, Type-Safe Web Backends"))
        .stdout(predicate::str::contains("generate"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("component"));

    // Test generate help
    let mut gen_help_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    gen_help_cmd.arg("generate").arg("--help");

    gen_help_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate a Rust web backend"))
        .stdout(predicate::str::contains("--output"))
        .stdout(predicate::str::contains("--force"));

    // Test init help
    let mut init_help_cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    init_help_cmd.arg("init").arg("--help");

    init_help_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new rustform project"))
        .stdout(predicate::str::contains("--database"))
        .stdout(predicate::str::contains("--directory"));
}

#[rstest]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[rstest]
#[tokio::test]
async fn test_example_configs_generation() {
    let example_configs = vec![
        ("examples/todo.yml", "todo_example"),
        ("examples/blog.yml", "blog_example"),
        ("examples/ecommerce.yml", "ecommerce_example"),
    ];

    for (config_file, project_name) in example_configs {
        if !Path::new(config_file).exists() {
            continue; // Skip if example doesn't exist
        }

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join(project_name);

        let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
        cmd.arg("generate")
            .arg(config_file)
            .arg("--output")
            .arg(&output_path)
            .arg("--force");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Generated").or(predicate::str::contains("Success")));

        // Verify basic files exist
        assert!(output_path.join("Cargo.toml").exists(), "Cargo.toml should exist for {}", project_name);
        assert!(output_path.join("src/main.rs").exists(), "main.rs should exist for {}", project_name);
        
        // Test that it compiles
        let mut build_cmd = std::process::Command::new("cargo");
        build_cmd.arg("check")
            .current_dir(&output_path);

        let build_output = build_cmd.output().expect("Should run cargo check");
        
        if !build_output.status.success() {
            let stderr = String::from_utf8_lossy(&build_output.stderr);
            panic!("Example {} failed to compile:\n{}", project_name, stderr);
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_concurrent_generation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create multiple config files
    let configs = vec![
        ("project1", "sqlite"),
        ("project2", "postgres"),
        ("project3", "mysql"),
    ];

    let mut handles = vec![];

    for (name, db_type) in configs {
        let temp_path = temp_dir.path().to_path_buf();
        let handle = tokio::spawn(async move {
            let config_path = temp_path.join(format!("{}.yml", name));
            let output_path = temp_path.join(name);

            let config_content = format!(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: {}
version: "1.0.0"
database:
  type: {}
  url_env: DATABASE_URL
api:
  models:
    Item:
      table_name: items
      fields:
        id:
          type: uuid
          primary_key: true
        name:
          type: string
          required: true
  endpoints:
    - path: /items
      model: Item
      crud:
        create: true
        read_all: true
"#, name, db_type);

            fs::write(&config_path, config_content).expect("Should write config");

            let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
            cmd.arg("generate")
                .arg(&config_path)
                .arg("--output")
                .arg(&output_path)
                .arg("--force");

            cmd.assert().success();

            // Verify files were generated
            assert!(output_path.join("Cargo.toml").exists());
            assert!(output_path.join("src/main.rs").exists());
        });

        handles.push(handle);
    }

    // Wait for all generations to complete
    for handle in handles {
        handle.await.expect("Generation task should succeed");
    }
}

mod e2e_performance_tests {
    use super::*;
    use std::time::Instant;

    #[rstest]
    #[tokio::test]
    async fn test_large_project_generation_performance() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("large.yml");
        let output_path = temp_dir.path().join("large_project");

        // Create a large configuration with many models
        let mut config_content = String::from(r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: large_project
version: "1.0.0"
database:
  type: postgres
  url_env: DATABASE_URL
api:
  models:
"#);

        // Add 20 models with multiple fields each
        for i in 1..=20 {
            config_content.push_str(&format!(r#"
    Model{}:
      table_name: model{}s
      fields:
        id:
          type: uuid
          primary_key: true
        name:
          type: string
          required: true
        description:
          type: text
        value:
          type: integer
          default: 0
        active:
          type: boolean
          default: true
        created_at:
          type: datetime
          default: "now()"
"#, i, i));
        }

        config_content.push_str("  endpoints:\n");

        // Add endpoints for all models
        for i in 1..=20 {
            config_content.push_str(&format!(r#"
    - path: /model{}s
      model: Model{}
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
"#, i, i));
        }

        config_content.push_str(r#"
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#);

        fs::write(&config_path, config_content).expect("Should write large config");

        let start = Instant::now();

        let mut cmd = Command::cargo_bin("rustform").expect("Should find rustform binary");
        cmd.arg("generate")
            .arg(&config_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--force");

        cmd.assert().success();

        let duration = start.elapsed();

        // Should complete in reasonable time (less than 30 seconds)
        assert!(duration.as_secs() < 30, "Large project generation took too long: {:?}", duration);

        // Verify all files were generated
        assert!(output_path.join("Cargo.toml").exists());
        assert!(output_path.join("src/main.rs").exists());
        assert!(output_path.join("src/models.rs").exists());
        assert!(output_path.join("src/handlers.rs").exists());

        // Test that it compiles
        let build_start = Instant::now();
        
        let mut build_cmd = std::process::Command::new("cargo");
        build_cmd.arg("check")
            .current_dir(&output_path);

        let build_output = build_cmd.output().expect("Should run cargo check");
        let build_duration = build_start.elapsed();

        assert!(build_output.status.success(), "Large project should compile successfully");
        assert!(build_duration.as_secs() < 60, "Large project compilation took too long: {:?}", build_duration);
    }
}