use crate::error::CliError;
use std::path::PathBuf;
use tracing::info;

pub struct InitCommand {
    pub name: Option<String>,
    pub directory: Option<PathBuf>,
    pub database: String,
}

impl InitCommand {
    pub fn new(name: Option<String>, directory: Option<PathBuf>, database: String) -> Self {
        Self { name, directory, database }
    }
    
    pub async fn execute(&self) -> Result<(), CliError> {
        // Determine project name
        let project_name = match &self.name {
            Some(name) => {
                self.validate_project_name(name)?;
                name.clone()
            }
            None => {
                println!("Enter project name:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let name = input.trim().to_string();
                self.validate_project_name(&name)?;
                name
            }
        };
        
        // Determine directory
        let project_dir = self.directory.clone()
            .unwrap_or_else(|| PathBuf::from(&project_name));
        
        // Check if directory already exists
        if project_dir.exists() {
            return Err(CliError::directory_exists(project_dir.display().to_string()));
        }
        
        info!("Initializing new rustform project: {}", project_name);
        
        // Create project directory
        std::fs::create_dir_all(&project_dir)?;
        
        // Create basic config.yml
        let config_content = self.create_config_template(&project_name);
        let config_path = project_dir.join("config.yml");
        std::fs::write(&config_path, config_content)?;
        
        // Create .env.example
        let env_content = format!("DATABASE_URL=\"{}:database.db\"\n", self.database);
        let env_path = project_dir.join(".env.example");
        std::fs::write(&env_path, env_content)?;
        
        info!("✅ Successfully initialized project: {}", project_name);
        println!("Initialized new rustform project: {}", project_name);
        println!("Directory: {}", project_dir.display());
        println!();
        println!("Next steps:");
        println!("  cd {}", project_dir.display());
        println!("  # Edit config.yml to define your models and endpoints");
        println!("  rustform generate config.yml");
        
        Ok(())
    }
    
    fn validate_project_name(&self, name: &str) -> Result<(), CliError> {
        if name.is_empty() {
            return Err(CliError::invalid_project_name(name));
        }
        
        // Check for valid Rust crate name
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_') {
            return Err(CliError::invalid_project_name(name));
        }
        
        if name.starts_with('-') || name.ends_with('-') {
            return Err(CliError::invalid_project_name(name));
        }
        
        Ok(())
    }
    
    fn create_config_template(&self, project_name: &str) -> String {
        format!(r#"project_name: {}
version: "0.1.0"

database:
  type: {}
  url_env: DATABASE_URL

api:
  models:
    Example:
      table_name: examples
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        name:
          type: string
          required: true
        description:
          type: string
        created_at:
          type: datetime
          auto_now_add: true

  endpoints:
    - path: /examples
      model: Example
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true

middleware:
  - logger: true
  - cors:
      allow_origin: "*"
"#, project_name, self.database)
    }
}