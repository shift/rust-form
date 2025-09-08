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
    
    #[error("Logging setup error: {0}")]
    #[diagnostic(
        code(rustform::logging_error),
        help("Failed to initialize logging system")
    )]
    Logging(String),
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
}

impl From<miette::ErrReport> for CliError {
    fn from(err: miette::ErrReport) -> Self {
        Self::Logging(err.to_string())
    }
}