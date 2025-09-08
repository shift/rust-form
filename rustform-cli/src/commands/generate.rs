use crate::error::CliError;
use rustform_core::Config;
use std::path::PathBuf;
use tracing::{info, warn};

pub struct GenerateCommand {
    pub config: PathBuf,
    pub output: Option<PathBuf>,
    pub force: bool,
}

impl GenerateCommand {
    pub fn new(config: PathBuf, output: Option<PathBuf>, force: bool) -> Self {
        Self { config, output, force }
    }
    
    pub async fn execute(&self) -> Result<(), CliError> {
        info!("Starting generation from config: {}", self.config.display());
        
        // Check if config file exists
        if !self.config.exists() {
            return Err(CliError::config_not_found(self.config.display().to_string()));
        }
        
        // Read and parse configuration
        let config_content = std::fs::read_to_string(&self.config)?;
        let config: Config = serde_yaml::from_str(&config_content)
            .map_err(|e| {
                let span = (0, config_content.len()).into();
                CliError::config_error(
                    rustform_core::ConfigError::Yaml(e),
                    config_content.clone(),
                    span
                )
            })?;
        
        // Validate configuration
        rustform_core::validate_config(&config)
            .map_err(|e| {
                let span = (0, config_content.len()).into();
                CliError::config_error(e, config_content.clone(), span)
            })?;
        
        // Determine output directory
        let output_dir = self.output.clone()
            .unwrap_or_else(|| PathBuf::from(&config.project_name));
        
        // Check if output directory exists
        if output_dir.exists() && !self.force {
            return Err(CliError::directory_exists(output_dir.display().to_string()));
        }
        
        if output_dir.exists() && self.force {
            warn!("Overwriting existing directory: {}", output_dir.display());
        }
        
        // Generate the project
        info!("Generating project '{}' in: {}", config.project_name, output_dir.display());
        
        let pipeline = rustform_codegen::GenerationPipeline::new();
        pipeline.generate()?;
        
        info!("✅ Successfully generated project: {}", config.project_name);
        println!("Generated project '{}' in: {}", config.project_name, output_dir.display());
        println!();
        println!("Next steps:");
        println!("  cd {}", output_dir.display());
        println!("  cargo build --release");
        println!("  ./target/release/{}", config.project_name);
        
        Ok(())
    }
}