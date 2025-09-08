use crate::config::Config;
use crate::error::ConfigError;

pub fn validate_config(config: &Config) -> Result<(), ConfigError> {
    if config.project_name.is_empty() {
        return Err(ConfigError::Validation("Project name cannot be empty".to_string()));
    }
    
    // Add more validation rules here
    Ok(())
}