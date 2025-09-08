use rstest::*;
use tempfile::TempDir;
use similar_asserts::assert_eq;
use crate::cli::{Cli, Commands};

#[rstest]
fn test_cli_command_parsing() {
    // Test generate command
    let args = vec!["rustform", "generate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse generate command");
    
    match cli.command {
        Commands::Generate { config } => {
            assert_eq!(config.to_string_lossy(), "config.yml");
        }
        _ => panic!("Expected Generate command"),
    }
}

#[rstest]
fn test_cli_component_commands() {
    // Test component list
    let args = vec!["rustform", "component", "list"];
    let cli = Cli::try_parse_from(args).expect("Should parse component list command");
    
    match cli.command {
        Commands::Component { subcommand } => {
            match subcommand {
                crate::cli::ComponentCommands::List { .. } => {
                    // Success
                }
                _ => panic!("Expected List subcommand"),
            }
        }
        _ => panic!("Expected Component command"),
    }
    
    // Test component install
    let args = vec!["rustform", "component", "install", "auth/jwt-manager"];
    let cli = Cli::try_parse_from(args).expect("Should parse component install command");
    
    match cli.command {
        Commands::Component { subcommand } => {
            match subcommand {
                crate::cli::ComponentCommands::Install { uri } => {
                    assert_eq!(uri, "auth/jwt-manager");
                }
                _ => panic!("Expected Install subcommand"),
            }
        }
        _ => panic!("Expected Component command"),
    }
}

#[rstest]
fn test_cli_validation_command() {
    let args = vec!["rustform", "validate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse validate command");
    
    match cli.command {
        Commands::Validate { config } => {
            assert_eq!(config.to_string_lossy(), "config.yml");
        }
        _ => panic!("Expected Validate command"),
    }
}

#[rstest]
fn test_cli_help() {
    let args = vec!["rustform", "--help"];
    let result = Cli::try_parse_from(args);
    
    // Help should cause an error (but it's expected)
    assert!(result.is_err());
}

#[rstest]
fn test_cli_version() {
    let args = vec!["rustform", "--version"];
    let result = Cli::try_parse_from(args);
    
    // Version should cause an error (but it's expected)
    assert!(result.is_err());
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;
    use std::process::Command;
    
    #[test]
    fn test_cli_binary_execution() {
        // This test requires the binary to be built
        let output = Command::new("cargo")
            .args(&["run", "--bin", "rustform", "--", "--help"])
            .output();
            
        if let Ok(output) = output {
            assert!(output.status.success() || output.status.code() == Some(0));
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("rustform"));
        }
        // If cargo run fails, that's okay for this test - it just means the binary isn't built
    }
    
    #[test]
    fn test_config_validation_workflow() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("test_config.yml");
        
        let config_content = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: cli_test
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
        
        std::fs::write(&config_path, config_content).expect("Failed to write config file");
        
        // Test that the config file can be parsed by the validation logic
        let content = std::fs::read_to_string(&config_path).expect("Failed to read config");
        let config: rustform_core::config::Config = serde_yaml::from_str(&content).expect("Failed to parse config");
        
        use rustform_core::config::validation::validate_config;
        assert!(validate_config(&config).is_ok());
    }
}