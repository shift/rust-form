use clap::Args;
use rustform_core::component::{ComponentSystem, ComponentUri};
use std::path::PathBuf;
use std::fs;
use crate::error::CliError;

#[derive(Args)]
pub struct ComponentCommand {
    #[command(subcommand)]
    pub action: ComponentAction,
}

#[derive(clap::Subcommand)]
pub enum ComponentAction {
    /// Install a component
    Install {
        /// Component URI (e.g., github:org/repo@v1.0.0, path:./local)
        uri: String,
        /// Output directory for components
        #[arg(short, long, default_value = ".rustform/components")]
        output: PathBuf,
    },
    /// List installed components
    List {
        /// Components directory
        #[arg(short, long, default_value = ".rustform/components")]
        directory: PathBuf,
    },
    /// Remove a component
    Remove {
        /// Component name
        name: String,
        /// Components directory
        #[arg(short, long, default_value = ".rustform/components")]
        directory: PathBuf,
    },
    /// Validate component manifest
    Validate {
        /// Path to component directory or manifest file
        path: PathBuf,
    },
}

impl ComponentCommand {
    pub async fn execute(&self) -> Result<(), CliError> {
        match &self.action {
            ComponentAction::Install { uri, output } => {
                self.install_component(uri, output).await
            }
            ComponentAction::List { directory } => {
                self.list_components(directory).await
            }
            ComponentAction::Remove { name, directory } => {
                self.remove_component(name, directory).await
            }
            ComponentAction::Validate { path } => {
                self.validate_component(path).await
            }
        }
    }

    async fn install_component(&self, uri: &str, _output: &PathBuf) -> Result<(), CliError> {
        let component_uri = uri.parse::<ComponentUri>()
            .map_err(|e| CliError::ComponentError(format!("Invalid URI: {}", e)))?;
        
        let mut component_system = ComponentSystem::new()
            .map_err(|e| CliError::ComponentError(format!("Failed to initialize component system: {}", e)))?;
        
        println!("Installing component from: {}", uri);
        
        let component = component_system.install_component(&component_uri).await
            .map_err(|e| CliError::ComponentError(format!("Failed to install component: {}", e)))?;
        
        println!("✅ Successfully installed component: {}", component.manifest.name);
        println!("   Version: {}", component.manifest.version);
        if let Some(description) = &component.manifest.description {
            println!("   Description: {}", description);
        }
        
        Ok(())
    }

    async fn list_components(&self, directory: &PathBuf) -> Result<(), CliError> {
        if !directory.exists() {
            println!("No components directory found at: {}", directory.display());
            return Ok(());
        }

        let entries = fs::read_dir(directory)
            .map_err(|e| CliError::ComponentError(format!("Failed to read components directory: {}", e)))?;

        let mut components = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| CliError::ComponentError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                let manifest_path = path.join("rustform-component.yml");
                if manifest_path.exists() {
                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = serde_yaml::from_str::<rustform_core::component::ComponentManifest>(&content) {
                            components.push((path.file_name().unwrap().to_string_lossy().to_string(), manifest));
                        }
                    }
                }
            }
        }

        if components.is_empty() {
            println!("No components installed.");
        } else {
            println!("Installed components:");
            for (dir_name, manifest) in components {
                println!("  {} ({})", manifest.name, manifest.version);
                if let Some(description) = &manifest.description {
                    println!("    {}", description);
                }
                println!("    Directory: {}", dir_name);
                println!();
            }
        }

        Ok(())
    }

    async fn remove_component(&self, name: &str, directory: &PathBuf) -> Result<(), CliError> {
        let component_path = directory.join(name);
        
        if !component_path.exists() {
            return Err(CliError::ComponentError(format!("Component '{}' not found", name)));
        }

        fs::remove_dir_all(&component_path)
            .map_err(|e| CliError::ComponentError(format!("Failed to remove component: {}", e)))?;
        
        println!("✅ Successfully removed component: {}", name);
        Ok(())
    }

    async fn validate_component(&self, path: &PathBuf) -> Result<(), CliError> {
        let manifest_path = if path.is_file() && path.file_name().unwrap() == "rustform-component.yml" {
            path.clone()
        } else if path.is_dir() {
            path.join("rustform-component.yml")
        } else {
            return Err(CliError::ComponentError("Path must be a directory or rustform-component.yml file".to_string()));
        };

        if !manifest_path.exists() {
            return Err(CliError::ComponentError("Component manifest not found".to_string()));
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| CliError::ComponentError(format!("Failed to read manifest: {}", e)))?;

        let manifest = serde_yaml::from_str::<rustform_core::component::ComponentManifest>(&content)
            .map_err(|e| CliError::ComponentError(format!("Invalid manifest format: {}", e)))?;

        println!("✅ Component manifest is valid:");
        println!("   Name: {}", manifest.name);
        println!("   Version: {}", manifest.version);
        if let Some(description) = &manifest.description {
            println!("   Description: {}", description);
        }
        println!("   Templates: {}", manifest.provides.templates.len());
        println!("   Assets: {}", manifest.provides.assets.len());
        println!("   Hooks: {}", manifest.provides.hooks.len());
        
        Ok(())
    }
}