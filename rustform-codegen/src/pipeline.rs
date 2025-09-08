use crate::error::CodeGenError;
use crate::engine::TemplateEngine;
use crate::context::GenerationContext;
use rustform_core::{Config, GeneratedProject, GeneratedFile};
use rustform_core::config::schema::FrontendConfig;
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
    
    pub async fn generate(&mut self, config: &Config, _output_dir: &Path) -> Result<GeneratedProject, CodeGenError> {
        // Install components if any are specified
        if config.components.is_some() {
            self.engine.install_components(config).await?;
        }

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
        
        // Generate frontend if configured
        if let Some(frontend_config) = &config.frontend {
            files.extend(self.generate_frontend(frontend_config, &gen_context, &context)?);
        }

        // Generate component lockfile if components were used
        if config.components.is_some() {
            let lockfile = self.engine.generate_lockfile(config).await?;
            files.push(GeneratedFile {
                path: "rustform.lock".to_string(),
                content: serde_yaml::to_string(&lockfile)
                    .map_err(|e| CodeGenError::Context(format!("Failed to serialize lockfile: {}", e)))?,
            });
        }
        
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
    
    fn generate_frontend(
        &self, 
        frontend_config: &FrontendConfig,
        gen_context: &GenerationContext,
        context: &Context
    ) -> Result<Vec<GeneratedFile>, CodeGenError> {
        let mut files = Vec::new();
        
        // Determine output directory
        let output_dir = &frontend_config.typescript_output_dir;
        
        // Generate TypeScript types
        files.push(self.generate_file(
            "frontend/types.ts.tera", 
            &format!("{}/types.ts", output_dir),
            context
        )?);
        
        // Generate API client
        files.push(self.generate_file(
            "frontend/api-client.ts.tera",
            &format!("{}/api-client.ts", output_dir),
            context
        )?);
        
        // Generate React Query hooks (if React is the target)
        if frontend_config.target == "react" {
            files.push(self.generate_file(
                "frontend/hooks.ts.tera",
                &format!("{}/hooks.ts", output_dir),
                context
            )?);
        }
        
        // Generate package.json for the frontend project
        files.push(self.generate_frontend_package_json(frontend_config, gen_context)?);
        
        // Generate tsconfig.json
        files.push(self.generate_frontend_tsconfig(frontend_config)?);
        
        // Generate framework-specific files based on target
        match frontend_config.target.as_str() {
            "react" => files.extend(self.generate_react_project(frontend_config, gen_context, context)?),
            "vue" => files.extend(self.generate_vue_project(frontend_config, gen_context, context)?),
            "svelte" => files.extend(self.generate_svelte_project(frontend_config, gen_context, context)?),
            _ => return Err(CodeGenError::Template(format!("Unsupported frontend target: {}", frontend_config.target))),
        }
        
        Ok(files)
    }
    
    fn generate_frontend_package_json(&self, frontend_config: &FrontendConfig, gen_context: &GenerationContext) -> Result<GeneratedFile, CodeGenError> {
        let mut dependencies = vec![
            ("typescript", "^5.0.0"),
        ];
        
        let mut dev_dependencies = vec![
            ("@types/node", "^20.0.0"),
            ("vite", "^5.0.0"),
        ];
        
        // Add framework-specific dependencies
        match frontend_config.target.as_str() {
            "react" => {
                dependencies.extend([
                    ("react", "^18.0.0"),
                    ("react-dom", "^18.0.0"),
                    ("@tanstack/react-query", "^5.0.0"),
                ]);
                dev_dependencies.extend([
                    ("@types/react", "^18.0.0"),
                    ("@types/react-dom", "^18.0.0"),
                    ("@vitejs/plugin-react", "^4.0.0"),
                ]);
            }
            "vue" => {
                dependencies.push(("vue", "^3.0.0"));
                dev_dependencies.push(("@vitejs/plugin-vue", "^4.0.0"));
            }
            "svelte" => {
                dependencies.push(("svelte", "^4.0.0"));
                dev_dependencies.push(("@sveltejs/vite-plugin-svelte", "^3.0.0"));
            }
            _ => {}
        }
        
        let content = format!(
            r#"{{
  "name": "{}-frontend",
  "version": "0.1.0",
  "type": "module",
  "scripts": {{
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "type-check": "tsc --noEmit"
  }},
  "dependencies": {{
    {}
  }},
  "devDependencies": {{
    {}
  }}
}}"#,
            gen_context.project_name,
            dependencies.iter()
                .map(|(name, version)| format!("    \"{}\": \"{}\"", name, version))
                .collect::<Vec<_>>()
                .join(",\n"),
            dev_dependencies.iter()
                .map(|(name, version)| format!("    \"{}\": \"{}\"", name, version))
                .collect::<Vec<_>>()
                .join(",\n")
        );
        
        Ok(GeneratedFile {
            path: "frontend/package.json".to_string(),
            content,
        })
    }
    
    fn generate_frontend_tsconfig(&self, _frontend_config: &FrontendConfig) -> Result<GeneratedFile, CodeGenError> {
        let content = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.tsx"],
  "references": [{ "path": "./tsconfig.node.json" }]
}"#;
        
        Ok(GeneratedFile {
            path: "frontend/tsconfig.json".to_string(),
            content: content.to_string(),
        })
    }
    
    fn generate_react_project(
        &self,
        _frontend_config: &FrontendConfig,
        gen_context: &GenerationContext,
        _context: &Context
    ) -> Result<Vec<GeneratedFile>, CodeGenError> {
        let mut files = Vec::new();
        
        // Generate App.tsx
        let app_content = format!(
            r#"import React from 'react';
import {{ QueryClient, QueryClientProvider }} from '@tanstack/react-query';

const queryClient = new QueryClient();

function App() {{
  return (
    <QueryClientProvider client={{queryClient}}>
      <div className="App">
        <h1>{} App</h1>
        <p>Frontend generated with Rust-form</p>
      </div>
    </QueryClientProvider>
  );
}}

export default App;
"#,
            gen_context.project_name
        );
        
        files.push(GeneratedFile {
            path: "frontend/src/App.tsx".to_string(),
            content: app_content,
        });
        
        // Generate main.tsx
        let main_content = r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#;
        
        files.push(GeneratedFile {
            path: "frontend/src/main.tsx".to_string(),
            content: main_content.to_string(),
        });
        
        // Generate index.html
        let html_content = format!(
            r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#,
            gen_context.project_name
        );
        
        files.push(GeneratedFile {
            path: "frontend/index.html".to_string(),
            content: html_content,
        });
        
        // Generate vite.config.ts
        let vite_config = r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': 'http://localhost:3000'
    }
  }
})
"#;
        
        files.push(GeneratedFile {
            path: "frontend/vite.config.ts".to_string(),
            content: vite_config.to_string(),
        });
        
        // Generate basic CSS
        let css_content = r#"body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.App {
  text-align: center;
  padding: 2rem;
}
"#;
        
        files.push(GeneratedFile {
            path: "frontend/src/index.css".to_string(),
            content: css_content.to_string(),
        });
        
        Ok(files)
    }
    
    fn generate_vue_project(
        &self,
        _frontend_config: &FrontendConfig,
        _gen_context: &GenerationContext,
        _context: &Context
    ) -> Result<Vec<GeneratedFile>, CodeGenError> {
        // TODO: Implement Vue project generation
        Ok(vec![])
    }
    
    fn generate_svelte_project(
        &self,
        _frontend_config: &FrontendConfig,
        _gen_context: &GenerationContext,
        _context: &Context
    ) -> Result<Vec<GeneratedFile>, CodeGenError> {
        // TODO: Implement Svelte project generation
        Ok(vec![])
    }
}