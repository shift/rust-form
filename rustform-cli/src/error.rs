use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(#[from] rustform_core::ConfigError),
    
    #[error("Code generation error: {0}")]
    CodeGen(#[from] rustform_codegen::CodeGenError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}