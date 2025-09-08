use crate::error::CodeGenError;
use crate::engine::TemplateEngine;
use crate::context::GenerationContext;
use rustform_core::{Config, GeneratedProject, GeneratedFile};
use std::path::Path;
use tera::Context;

pub struct GenerationPipeline {
    engine: TemplateEngine,
}

impl GenerationPipeline {
    pub fn new() -> Result<Self, CodeGenError> {
        let engine = TemplateEngine::new()?;
        Ok(Self { engine })
    }
    
    pub fn generate(&self, config: &Config, output_dir: &Path) -> Result<GeneratedProject, CodeGenError> {
        // Build generation context from config
        let gen_context = GenerationContext::from_config(config)?;
        
        // Convert to Tera context
        let context = Context::from_serialize(&gen_context)
            .map_err(|e| CodeGenError::Context(format!("Failed to serialize context: {}", e)))?;
        
        let mut files = Vec::new();
        
        // Generate main.rs
        files.push(self.generate_file("main.rs.tera", "src/main.rs", &context)?);
        
        // Generate Cargo.toml
        files.push(self.generate_file("Cargo.toml.tera", "Cargo.toml", &context)?);
        
        // Generate models.rs
        files.push(self.generate_file("models.rs.tera", "src/models.rs", &context)?);
        
        // Generate handlers.rs
        files.push(self.generate_file("handlers.rs.tera", "src/handlers.rs", &context)?);
        
        // Generate database.rs
        files.push(self.generate_file("database.rs.tera", "src/database.rs", &context)?);
        
        // Generate error.rs
        files.push(self.generate_file("error.rs.tera", "src/error.rs", &context)?);
        
        // Generate migration files
        files.extend(self.generate_migrations(&gen_context)?);
        
        // Generate .env example
        files.push(self.generate_env_file(&gen_context)?);
        
        // Generate README.md
        files.push(self.generate_readme(&gen_context)?);
        
        Ok(GeneratedProject {
            name: config.project_name.clone(),
            files,
        })
    }
    
    fn generate_migrations(&self, context: &GenerationContext) -> Result<Vec<GeneratedFile>, CodeGenError> {
        let mut files = Vec::new();
        
        // Generate initial migration
        let mut migration_content = String::new();
        
        for model in &context.models {
            migration_content.push_str(&format!(
                "CREATE TABLE {} (\n",
                model.table_name
            ));
            
            for (i, field) in model.fields.iter().enumerate() {
                let mut column_def = format!("  {} {}", field.name, self.sql_type_for_field(field));
                
                if field.is_primary_key {
                    column_def.push_str(" PRIMARY KEY");
                }
                if field.auto_increment {
                    column_def.push_str(" AUTOINCREMENT");
                }
                if !field.is_nullable && !field.is_primary_key {
                    column_def.push_str(" NOT NULL");
                }
                if let Some(ref default) = field.default_value {
                    // Format default values properly for SQL
                    let sql_default = match field.field_type.as_str() {
                        "string" | "text" => format!("'{}'", default.trim_matches('"')),
                        "boolean" => if default.contains("true") { "1" } else { "0" }.to_string(),
                        "integer" => default.chars().filter(|c| c.is_ascii_digit() || *c == '-').collect(),
                        _ => default.clone(),
                    };
                    column_def.push_str(&format!(" DEFAULT {}", sql_default));
                }
                
                if i < model.fields.len() - 1 {
                    column_def.push(',');
                }
                
                migration_content.push_str(&column_def);
                migration_content.push('\n');
            }
            
            migration_content.push_str(");\n\n");
            
            // Add indexes
            for index in &model.indexes {
                migration_content.push_str(&format!(
                    "CREATE {} INDEX {} ON {} ({});\n",
                    if index.unique { "UNIQUE" } else { "" },
                    index.name,
                    model.table_name,
                    index.fields.join(", ")
                ));
            }
            migration_content.push('\n');
        }
        
        files.push(GeneratedFile {
            path: "migrations/001_initial.sql".to_string(),
            content: migration_content,
        });
        
        Ok(files)
    }
    
    fn sql_type_for_field(&self, field: &crate::context::FieldContext) -> &'static str {
        match field.field_type.as_str() {
            "integer" => "INTEGER",
            "string" | "text" => "TEXT", 
            "boolean" => "BOOLEAN",
            "float" | "double" => "REAL",
            "datetime" | "date" | "time" => "DATETIME",
            "uuid" => "TEXT",
            _ => "TEXT",
        }
    }
    
    fn generate_env_file(&self, context: &GenerationContext) -> Result<GeneratedFile, CodeGenError> {
        let content = format!(
            "# Database Configuration\n{}=sqlite:{}.db\n\n# Server Configuration\nRUST_LOG=debug\nSERVER_HOST=127.0.0.1\nSERVER_PORT=3000\n",
            context.database.url_env,
            context.project_name
        );
        
        Ok(GeneratedFile {
            path: ".env.example".to_string(),
            content,
        })
    }
    
    fn generate_readme(&self, context: &GenerationContext) -> Result<GeneratedFile, CodeGenError> {
        let mut content = format!(
            "# {}\n\nGenerated Rust web API using [Rustফর্ম](https://github.com/your-org/rust-form)\n\n",
            context.project_name
        );
        
        content.push_str("## Getting Started\n\n");
        content.push_str("1. Copy the environment file:\n");
        content.push_str("   ```bash\n   cp .env.example .env\n   ```\n\n");
        content.push_str("2. Set up the database:\n");
        content.push_str("   ```bash\n   cargo install sqlx-cli\n");
        content.push_str(&format!("   export {}=sqlite:{}.db\n", context.database.url_env, context.project_name));
        content.push_str("   sqlx database create\n   ```\n\n");
        content.push_str("3. Run the application:\n");
        content.push_str("   ```bash\n   cargo run\n   ```\n\n");
        
        content.push_str("## API Endpoints\n\n");
        for endpoint in &context.endpoints {
            content.push_str(&format!("### {} ({})\n\n", endpoint.model_name, endpoint.path));
            for operation in &endpoint.operations {
                let method = match operation.as_str() {
                    "read_all" => "GET",
                    "create" => "POST", 
                    "read_one" => "GET",
                    "update" => "PUT",
                    "patch" => "PATCH",
                    "delete" => "DELETE",
                    _ => "GET",
                };
                let path = if operation == "read_all" || operation == "create" {
                    endpoint.path.clone()
                } else {
                    format!("{}/:id", endpoint.path)
                };
                content.push_str(&format!("- `{} {}` - {}\n", method, path, operation));
            }
            content.push('\n');
        }
        
        content.push_str("## Models\n\n");
        for model in &context.models {
            content.push_str(&format!("### {}\n\n", model.struct_name));
            content.push_str(&format!("Table: `{}`\n\n", model.table_name));
            content.push_str("| Field | Type | Constraints |\n");
            content.push_str("|-------|------|-------------|\n");
            for field in &model.fields {
                let constraints = if field.is_primary_key {
                    "Primary Key"
                } else if field.is_required {
                    "Required"
                } else {
                    "Optional"
                };
                content.push_str(&format!("| {} | {} | {} |\n", field.name, field.rust_type, constraints));
            }
            content.push('\n');
        }
        
        Ok(GeneratedFile {
            path: "README.md".to_string(),
            content,
        })
    }
    
    fn generate_file(&self, template_name: &str, output_path: &str, context: &Context) -> Result<GeneratedFile, CodeGenError> {
        let content = self.engine.render_template(template_name, context)?;
        
        Ok(GeneratedFile {
            path: output_path.to_string(),
            content,
        })
    }
}