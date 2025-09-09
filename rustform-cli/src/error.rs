use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CliError {
    #[error("Configuration error")]
    #[diagnostic(
        code(rustform::config_error),
        help("Check your configuration file syntax and try again")
    )]
    Config {
        source: rustform_core::ConfigError,
        #[source_code]
        src: String,
        #[label("Error occurred here")]
        span: SourceSpan,
    },

    #[error("Code generation error")]
    #[diagnostic(
        code(rustform::codegen_error),
        help("This is likely a bug. Please report it with your configuration file")
    )]
    CodeGen(#[from] rustform_codegen::CodeGenError),

    #[error("File system error")]
    #[diagnostic(
        code(rustform::io_error),
        help("Check file permissions and disk space")
    )]
    Io(#[from] std::io::Error),

    #[error("Invalid project name: {name}")]
    #[diagnostic(
        code(rustform::invalid_project_name),
        help("Project names must be valid Rust crate names (lowercase, hyphens allowed)")
    )]
    InvalidProjectName { name: String },

    #[error("Output directory already exists: {path}")]
    #[diagnostic(
        code(rustform::directory_exists),
        help("Use --force to overwrite, or choose a different output directory")
    )]
    DirectoryExists { path: String },

    #[error("Configuration file not found: {path}")]
    #[diagnostic(
        code(rustform::config_not_found),
        help("Make sure the file path is correct and the file exists")
    )]
    ConfigNotFound { path: String },

    #[error("Configuration parsing error: {message}")]
    #[diagnostic(
        code(rustform::config_parse_error),
        help("Check your YAML syntax and ensure all required fields are present")
    )]
    ConfigParseError { message: String },

    #[error("Configuration validation error: {message}")]
    #[diagnostic(
        code(rustform::config_validation_error),
        help("Fix the configuration issues listed above")
    )]
    ConfigValidationError { message: String },

    #[error("Logging setup error: {0}")]
    #[diagnostic(
        code(rustform::logging_error),
        help("Failed to initialize logging system")
    )]
    Logging(String),

    #[error("Component error: {0}")]
    #[diagnostic(
        code(rustform::component_error),
        help("Check component URI and ensure it's accessible")
    )]
    ComponentError(String),
}

impl CliError {
    pub fn config_error(source: rustform_core::ConfigError, src: String, span: SourceSpan) -> Self {
        Self::Config { source, src, span }
    }

    pub fn invalid_project_name(name: impl Into<String>) -> Self {
        Self::InvalidProjectName { name: name.into() }
    }

    pub fn directory_exists(path: impl Into<String>) -> Self {
        Self::DirectoryExists { path: path.into() }
    }

    pub fn config_not_found(path: impl Into<String>) -> Self {
        Self::ConfigNotFound { path: path.into() }
    }

    pub fn config_parse_error(message: impl Into<String>) -> Self {
        Self::ConfigParseError {
            message: message.into(),
        }
    }

    pub fn config_validation_error(message: impl Into<String>) -> Self {
        Self::ConfigValidationError {
            message: message.into(),
        }
    }

    pub fn io_error(error: std::io::Error) -> Self {
        Self::Io(error)
    }

    pub fn generation_error(message: impl Into<String>) -> Self {
        Self::CodeGen(rustform_codegen::CodeGenError::Generation(message.into()))
    }
}

impl From<miette::ErrReport> for CliError {
    fn from(err: miette::ErrReport) -> Self {
        Self::Logging(err.to_string())
    }
}
