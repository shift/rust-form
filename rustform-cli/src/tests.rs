use crate::cli::{Cli, Commands};
use clap::Parser;
use rstest::*;

#[rstest]
fn test_cli_command_parsing() {
    // Test generate command
    let args = vec!["rustform", "generate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse generate command");

    match cli.command {
        Commands::Generate { config, .. } => {
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
        Commands::Component { .. } => {
            // Component commands exist
        }
        _ => panic!("Expected Component command"),
    }
}

#[rstest]
fn test_cli_init_command() {
    let args = vec!["rustform", "init", "my-project"];
    let cli = Cli::try_parse_from(args).expect("Should parse init command");

    match cli.command {
        Commands::Init { name, .. } => {
            assert_eq!(name, Some("my-project".to_string()));
        }
        _ => panic!("Expected Init command"),
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