use crate::cli::{Cli, Commands};
use clap::Parser;
use rstest::*;
use std::path::PathBuf;

#[rstest]
fn test_cli_generate_command_parsing() {
    let args = vec!["rustform", "generate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse generate command");

    match cli.command {
        Commands::Generate {
            config,
            output,
            force,
        } => {
            assert_eq!(config, PathBuf::from("config.yml"));
            assert_eq!(output, None);
            assert!(!force);
        }
        _ => panic!("Expected Generate command"),
    }
}

#[rstest]
fn test_cli_generate_with_output() {
    let args = vec![
        "rustform",
        "generate",
        "config.yml",
        "--output",
        "my-project",
    ];
    let cli = Cli::try_parse_from(args).expect("Should parse generate with output");

    match cli.command {
        Commands::Generate {
            config,
            output,
            force,
        } => {
            assert_eq!(config, PathBuf::from("config.yml"));
            assert_eq!(output, Some(PathBuf::from("my-project")));
            assert!(!force);
        }
        _ => panic!("Expected Generate command"),
    }
}

#[rstest]
fn test_cli_generate_with_force() {
    let args = vec!["rustform", "generate", "config.yml", "--force"];
    let cli = Cli::try_parse_from(args).expect("Should parse generate with force");

    match cli.command {
        Commands::Generate {
            config,
            output,
            force,
        } => {
            assert_eq!(config, PathBuf::from("config.yml"));
            assert_eq!(output, None);
            assert!(force);
        }
        _ => panic!("Expected Generate command"),
    }
}

#[rstest]
fn test_cli_init_command_parsing() {
    let args = vec!["rustform", "init", "my-project"];
    let cli = Cli::try_parse_from(args).expect("Should parse init command");

    match cli.command {
        Commands::Init {
            name,
            directory,
            database,
        } => {
            assert_eq!(name, Some("my-project".to_string()));
            assert_eq!(directory, None);
            assert_eq!(database, "sqlite"); // Default value
        }
        _ => panic!("Expected Init command"),
    }
}

#[rstest]
fn test_cli_init_with_directory() {
    let args = vec![
        "rustform",
        "init",
        "my-project",
        "--directory",
        "custom-dir",
    ];
    let cli = Cli::try_parse_from(args).expect("Should parse init with directory");

    match cli.command {
        Commands::Init {
            name,
            directory,
            database,
        } => {
            assert_eq!(name, Some("my-project".to_string()));
            assert_eq!(directory, Some(PathBuf::from("custom-dir")));
            assert_eq!(database, "sqlite");
        }
        _ => panic!("Expected Init command"),
    }
}

#[rstest]
fn test_cli_init_with_database() {
    let args = vec!["rustform", "init", "my-project", "--database", "postgres"];
    let cli = Cli::try_parse_from(args).expect("Should parse init with database");

    match cli.command {
        Commands::Init {
            name,
            directory,
            database,
        } => {
            assert_eq!(name, Some("my-project".to_string()));
            assert_eq!(directory, None);
            assert_eq!(database, "postgres");
        }
        _ => panic!("Expected Init command"),
    }
}

#[rstest]
fn test_cli_init_invalid_database() {
    let args = vec!["rustform", "init", "my-project", "--database", "invalid-db"];
    let result = Cli::try_parse_from(args);

    // Should fail because "invalid-db" is not in the allowed values
    assert!(result.is_err());
}

#[rstest]
fn test_cli_verbose_flag() {
    let args = vec!["rustform", "--verbose", "generate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse with verbose flag");

    assert!(cli.verbose);
    assert!(!cli.quiet);
}

#[rstest]
fn test_cli_quiet_flag() {
    let args = vec!["rustform", "--quiet", "generate", "config.yml"];
    let cli = Cli::try_parse_from(args).expect("Should parse with quiet flag");

    assert!(!cli.verbose);
    assert!(cli.quiet);
}

#[rstest]
fn test_cli_verbose_quiet_conflict() {
    let args = vec!["rustform", "--verbose", "--quiet", "generate", "config.yml"];
    let result = Cli::try_parse_from(args);

    // Should fail because verbose and quiet conflict
    assert!(result.is_err());
}

#[rstest]
fn test_cli_component_command() {
    let args = vec!["rustform", "component", "list"];
    let cli = Cli::try_parse_from(args).expect("Should parse component command");

    match cli.command {
        Commands::Component { component: _ } => {
            // Success - component command was parsed
        }
        _ => panic!("Expected Component command"),
    }
}

#[rstest]
fn test_cli_component_alias() {
    let args = vec!["rustform", "comp", "list"];
    let cli = Cli::try_parse_from(args).expect("Should parse component alias");

    match cli.command {
        Commands::Component { component: _ } => {
            // Success - component alias worked
        }
        _ => panic!("Expected Component command"),
    }
}

#[rstest]
fn test_cli_version() {
    let args = vec!["rustform", "--version"];
    let result = Cli::try_parse_from(args);

    // Version flag should cause an error (but it's expected behavior)
    assert!(result.is_err());

    // Check that the error message contains version information
    let error = result.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains(env!("CARGO_PKG_VERSION")));
}

#[rstest]
fn test_cli_help() {
    let args = vec!["rustform", "--help"];
    let result = Cli::try_parse_from(args);

    // Help flag should cause an error (but it's expected behavior)
    assert!(result.is_err());

    // Check that the error message contains help information
    let error = result.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains("Declarative, Type-Safe Web Backends"));
}

#[rstest]
fn test_cli_missing_subcommand() {
    let args = vec!["rustform"];
    let result = Cli::try_parse_from(args);

    // Should fail because no subcommand was provided
    assert!(result.is_err());
}

#[rstest]
fn test_cli_invalid_subcommand() {
    let args = vec!["rustform", "invalid-command"];
    let result = Cli::try_parse_from(args);

    // Should fail because "invalid-command" is not a valid subcommand
    assert!(result.is_err());
}

#[rstest]
fn test_cli_generate_missing_config() {
    let args = vec!["rustform", "generate"];
    let result = Cli::try_parse_from(args);

    // Should fail because config file argument is required
    assert!(result.is_err());
}

#[rstest]
fn test_cli_short_flags() {
    let args = vec!["rustform", "generate", "config.yml", "-o", "output-dir"];
    let cli = Cli::try_parse_from(args).expect("Should parse with short flags");

    match cli.command {
        Commands::Generate {
            config,
            output,
            force,
        } => {
            assert_eq!(config, PathBuf::from("config.yml"));
            assert_eq!(output, Some(PathBuf::from("output-dir")));
            assert!(!force);
        }
        _ => panic!("Expected Generate command"),
    }
}

mod cli_property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_config_path_parsing_property(
            config_path in "[a-zA-Z][a-zA-Z0-9._/-]{0,99}\\.yml"
        ) {
            let args = vec!["rustform", "generate", &config_path];
            let result = Cli::try_parse_from(args);

            prop_assert!(result.is_ok());

            if let Ok(cli) = result {
                match cli.command {
                    Commands::Generate { config, .. } => {
                        prop_assert_eq!(config, PathBuf::from(&config_path));
                    }
                    _ => prop_assert!(false, "Expected Generate command"),
                }
            }
        }

        #[test]
        fn test_project_name_parsing_property(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,49}"
        ) {
            let args = vec!["rustform", "init", &name];
            let result = Cli::try_parse_from(args);

            prop_assert!(result.is_ok());

            if let Ok(cli) = result {
                match cli.command {
                    Commands::Init { name: parsed_name, .. } => {
                        prop_assert_eq!(parsed_name, Some(name.clone()));
                    }
                    _ => prop_assert!(false, "Expected Init command"),
                }
            }
        }

        #[test]
        fn test_database_type_validation_property(
            db_type in prop::sample::select(vec!["sqlite", "postgres", "mysql"])
        ) {
            let args = vec!["rustform", "init", "test-project", "--database", &db_type];
            let result = Cli::try_parse_from(args);

            prop_assert!(result.is_ok());

            if let Ok(cli) = result {
                match cli.command {
                    Commands::Init { database, .. } => {
                        prop_assert_eq!(database, db_type);
                    }
                    _ => prop_assert!(false, "Expected Init command"),
                }
            }
        }
    }
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_cli_parse_args_method() {
        // Test that the parse_args method works correctly
        // Note: This would normally use actual command line args,
        // but for testing we'll verify the method exists and is callable

        // We can't easily test this without mocking std::env::args(),
        // but we can at least verify the method compiles and is accessible
        let _parse_fn = Cli::parse_args;
        assert!(true); // Method exists and compiles
    }

    #[test]
    fn test_cli_about_text() {
        let args = vec!["rustform", "--help"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();

        // Verify about text is included
    assert!(error_msg.contains("Declarative, Type-Safe Web Backends in Rust"));
        assert!(error_msg.contains("rustform generate config.yml"));
        assert!(error_msg.contains("rustform init my-project"));
    }

    #[test]
    fn test_cli_version_constant() {
        // Verify that the version is set from CARGO_PKG_VERSION
        let args = vec!["rustform", "--version"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();

        // Should contain the package version
        assert!(error_msg.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_command_subcommand_structure() {
        // Test that all expected subcommands are available
        let help_args = vec!["rustform", "--help"];
        let result = Cli::try_parse_from(help_args);

        assert!(result.is_err());
        let help_text = result.unwrap_err().to_string();

        // Verify all main subcommands are listed
        assert!(help_text.contains("generate"));
        assert!(help_text.contains("init"));
        assert!(help_text.contains("component"));
    }
}
