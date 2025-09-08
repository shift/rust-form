use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] rustform_core::ConfigError),
}