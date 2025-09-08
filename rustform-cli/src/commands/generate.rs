use crate::error::CliError;
use rustform_core::parse_config_file;
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
        
        // Parse and validate configuration using enhanced parser
        let config = parse_config_file(&self.config)
            .map_err(|e| {
                // Convert ConfigError to CliError with proper error reporting
                match e {
                    rustform_core::ConfigError::Yaml(yaml_error) => {
                        CliError::config_parse_error(format!(
                            "YAML syntax error in {}: {}",
                            self.config.display(),
                            yaml_error
                        ))
                    }
                    rustform_core::ConfigError::Validation(validation_error) => {
                        CliError::config_validation_error(format!(
                            "Configuration validation failed in {}: {}",
                            self.config.display(),
                            validation_error
                        ))
                    }
                    rustform_core::ConfigError::Io(io_error) => {
                        CliError::io_error(io_error)
                    }
                }
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
        
        let pipeline = rustform_codegen::GenerationPipeline::new()
            .map_err(|e| CliError::generation_error(format!("Failed to initialize pipeline: {}", e)))?;
        
        let generated_project = pipeline.generate(&config, &output_dir)
            .map_err(|e| CliError::generation_error(format!("Generation failed: {}", e)))?;
        
        // Create output directory
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| CliError::io_error(e))?;
        
        // Write generated files
        for file in &generated_project.files {
            let file_path = output_dir.join(&file.path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| CliError::io_error(e))?;
            }
            
            std::fs::write(&file_path, &file.content)
                .map_err(|e| CliError::io_error(e))?;
                
            info!("Generated: {}", file.path);
        }
        
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