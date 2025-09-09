use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rustform")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Declarative, Type-Safe Web Backends in Rust")]
#[command(long_about = r#"
Rustফর্ম generates high-performance, memory-safe web backends from simple YAML configuration files.

Define your models, endpoints, and middleware in a config.yml file, and rustform will generate
a complete Rust web service using Axum, SQLx, and other modern libraries.

Example:
  rustform generate config.yml --output my-api
  rustform init my-project
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress most output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a Rust web backend from configuration
    Generate {
        /// Path to the configuration file
        #[arg(value_name = "CONFIG")]
        config: PathBuf,

        /// Output directory for generated project
        #[arg(short, long, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Overwrite existing files without prompting
        #[arg(long)]
        force: bool,
    },

    /// Initialize a new rustform project
    Init {
        /// Project name
        #[arg(value_name = "NAME")]
        name: Option<String>,

        /// Project directory (defaults to project name)
        #[arg(short, long, value_name = "DIR")]
        directory: Option<PathBuf>,

        /// Database type
        #[arg(long, default_value = "sqlite")]
        #[arg(value_parser = ["sqlite", "postgres", "mysql"])]
        database: String,
    },

    /// Manage components
    #[command(alias = "comp")]
    Component {
        #[command(flatten)]
        component: crate::commands::ComponentCommand,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests;
