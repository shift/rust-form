use crate::component::schema::ComponentConfig;
use std::collections::HashMap;
use tera::{Tera, Context};

/// Documentation generation system for components
pub struct DocGenerator {
    tera: Tera,
}

impl DocGenerator {
    /// Create a new documentation generator
    pub fn new() -> Result<Self, tera::Error> {
        let mut tera = Tera::new("rustform-codegen/templates/docs/**/*")?;
        
        // Register custom filters for documentation generation
        tera.register_filter("markdown_escape", markdown_escape_filter);
        tera.register_filter("code_block", code_block_filter);
        tera.register_filter("api_link", api_link_filter);
        tera.register_filter("example_name", example_name_filter);
        
        Ok(Self { tera })
    }
    
    /// Generate all documentation for a component
    pub fn generate_docs(&self, config: &ComponentConfig) -> Result<HashMap<String, String>, DocGenerationError> {
        let mut generated_docs = HashMap::new();
        
        // Generate main README
        let readme = self.generate_readme(config)?;
        generated_docs.insert("README.md".to_string(), readme);
        
        // Generate API documentation
        if config.documentation.api_docs {
            let api_docs = self.generate_api_docs(config)?;
            generated_docs.insert("docs/api.md".to_string(), api_docs);
        }
        
        // Generate configuration reference
        let config_docs = self.generate_config_docs(config)?;
        generated_docs.insert("docs/configuration.md".to_string(), config_docs);
        
        // Generate integration guide
        let integration_guide = self.generate_integration_guide(config)?;
        generated_docs.insert("docs/integration.md".to_string(), integration_guide);
        
        // Generate examples
        for example in &config.documentation.examples {
            let example_content = self.generate_example(config, example)?;
            generated_docs.insert(
                format!("examples/{}", example),
                example_content
            );
        }
        
        // Generate tutorial if requested
        if config.documentation.tutorial {
            let tutorial = self.generate_tutorial(config)?;
            generated_docs.insert("docs/tutorial.md".to_string(), tutorial);
        }
        
        // Generate troubleshooting guide
        let troubleshooting = self.generate_troubleshooting(config)?;
        generated_docs.insert("docs/troubleshooting.md".to_string(), troubleshooting);
        
        // Generate changelog template
        let changelog = self.generate_changelog(config)?;
        generated_docs.insert("CHANGELOG.md".to_string(), changelog);
        
        Ok(generated_docs)
    }
    
    /// Generate main README.md
    fn generate_readme(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("shields", &self.generate_shields(config));
        context.insert("toc", &self.generate_table_of_contents(config));
        
        self.tera.render("readme.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate API documentation
    fn generate_api_docs(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("api_endpoints", &self.extract_api_endpoints(config));
        context.insert("data_structures", &self.extract_data_structures(config));
        
        self.tera.render("api_docs.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate configuration documentation
    fn generate_config_docs(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("config_examples", &self.generate_config_examples(config));
        
        self.tera.render("config_docs.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate integration guide
    fn generate_integration_guide(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("integration_steps", &self.generate_integration_steps(config));
        context.insert("common_patterns", &self.generate_common_patterns(config));
        
        self.tera.render("integration_guide.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate code examples
    fn generate_example(&self, config: &ComponentConfig, example_name: &str) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("example_name", example_name);
        context.insert("example_type", &self.determine_example_type(example_name));
        
        let template_name = match example_name {
            name if name.contains("basic") => "examples/basic_example.rs.tera",
            name if name.contains("advanced") => "examples/advanced_example.rs.tera",
            name if name.contains("config") => "examples/config_example.rs.tera",
            name if name.contains("async") => "examples/async_example.rs.tera",
            _ => "examples/generic_example.rs.tera",
        };
        
        self.tera.render(template_name, &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate tutorial
    fn generate_tutorial(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("tutorial_steps", &self.generate_tutorial_steps(config));
        
        self.tera.render("tutorial.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate troubleshooting guide
    fn generate_troubleshooting(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("common_issues", &self.generate_common_issues(config));
        
        self.tera.render("troubleshooting.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate changelog template
    fn generate_changelog(&self, config: &ComponentConfig) -> Result<String, DocGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("initial_version", &config.version);
        
        self.tera.render("changelog.md.tera", &context)
            .map_err(DocGenerationError::TemplateError)
    }
    
    /// Generate shield badges for README
    fn generate_shields(&self, config: &ComponentConfig) -> Vec<Shield> {
        vec![
            Shield {
                label: "Version".to_string(),
                message: config.version.clone(),
                color: "blue".to_string(),
                url: None,
            },
            Shield {
                label: "Category".to_string(),
                message: config.category_str().to_string(),
                color: "green".to_string(),
                url: None,
            },
            Shield {
                label: "Priority".to_string(),
                message: config.priority_str().to_string(),
                color: match config.priority_str() {
                    "high" => "red",
                    "medium" => "orange",
                    "low" => "yellow",
                    _ => "gray",
                }.to_string(),
                url: None,
            },
            Shield {
                label: "Complexity".to_string(),
                message: config.complexity_str().to_string(),
                color: match config.complexity_str() {
                    "high" => "red",
                    "medium" => "orange",
                    "low" => "green",
                    _ => "gray",
                }.to_string(),
                url: None,
            },
            Shield {
                label: "License".to_string(),
                message: "MIT".to_string(),
                color: "blue".to_string(),
                url: Some("https://opensource.org/licenses/MIT".to_string()),
            },
        ]
    }
    
    /// Generate table of contents
    fn generate_table_of_contents(&self, config: &ComponentConfig) -> Vec<TocEntry> {
        let mut toc = vec![
            TocEntry { title: "Overview".to_string(), anchor: "overview".to_string() },
            TocEntry { title: "Installation".to_string(), anchor: "installation".to_string() },
            TocEntry { title: "Quick Start".to_string(), anchor: "quick-start".to_string() },
            TocEntry { title: "Configuration".to_string(), anchor: "configuration".to_string() },
        ];
        
        if config.documentation.api_docs {
            toc.push(TocEntry { title: "API Reference".to_string(), anchor: "api-reference".to_string() });
        }
        
        if !config.documentation.examples.is_empty() {
            toc.push(TocEntry { title: "Examples".to_string(), anchor: "examples".to_string() });
        }
        
        toc.extend(vec![
            TocEntry { title: "Contributing".to_string(), anchor: "contributing".to_string() },
            TocEntry { title: "License".to_string(), anchor: "license".to_string() },
        ]);
        
        toc
    }
    
    /// Extract API endpoints from component configuration
    fn extract_api_endpoints(&self, config: &ComponentConfig) -> Vec<ApiEndpoint> {
        let mut endpoints = Vec::new();
        
        // Analyze templates to determine API endpoints
        for template in &config.templates.generates {
            if template.contains("handler") || template.contains("route") {
                match config.category_str() {
                    "auth" => {
                        endpoints.extend(vec![
                            ApiEndpoint {
                                method: "POST".to_string(),
                                path: "/auth/login".to_string(),
                                description: "Authenticate user and return token".to_string(),
                                parameters: vec![
                                    Parameter { name: "email".to_string(), param_type: "string".to_string(), required: true },
                                    Parameter { name: "password".to_string(), param_type: "string".to_string(), required: true },
                                ],
                                responses: vec![
                                    Response { status: 200, description: "Authentication successful".to_string() },
                                    Response { status: 401, description: "Invalid credentials".to_string() },
                                ],
                            },
                            ApiEndpoint {
                                method: "POST".to_string(),
                                path: "/auth/refresh".to_string(),
                                description: "Refresh authentication token".to_string(),
                                parameters: vec![
                                    Parameter { name: "refresh_token".to_string(), param_type: "string".to_string(), required: true },
                                ],
                                responses: vec![
                                    Response { status: 200, description: "Token refreshed successfully".to_string() },
                                    Response { status: 401, description: "Invalid refresh token".to_string() },
                                ],
                            },
                        ]);
                    },
                    "api" => {
                        endpoints.push(ApiEndpoint {
                            method: "GET".to_string(),
                            path: "/health".to_string(),
                            description: "Health check endpoint".to_string(),
                            parameters: vec![],
                            responses: vec![
                                Response { status: 200, description: "Service is healthy".to_string() },
                                Response { status: 503, description: "Service is unhealthy".to_string() },
                            ],
                        });
                    },
                    _ => {}
                }
            }
        }
        
        endpoints
    }
    
    /// Extract data structures from component configuration
    fn extract_data_structures(&self, config: &ComponentConfig) -> Vec<DataStructure> {
        let mut structures = Vec::new();
        
        // Generate common structures based on component type
        match config.category_str() {
            "auth" => {
                structures.push(DataStructure {
                    name: "Claims".to_string(),
                    description: "JWT token claims".to_string(),
                    fields: vec![
                        Field { name: "sub".to_string(), field_type: "String".to_string(), description: "Subject (user ID)".to_string() },
                        Field { name: "exp".to_string(), field_type: "usize".to_string(), description: "Expiration time".to_string() },
                        Field { name: "iat".to_string(), field_type: "usize".to_string(), description: "Issued at time".to_string() },
                    ],
                });
            },
            "database" => {
                structures.push(DataStructure {
                    name: "ConnectionConfig".to_string(),
                    description: "Database connection configuration".to_string(),
                    fields: vec![
                        Field { name: "url".to_string(), field_type: "String".to_string(), description: "Database connection URL".to_string() },
                        Field { name: "max_connections".to_string(), field_type: "u32".to_string(), description: "Maximum connection pool size".to_string() },
                    ],
                });
            },
            _ => {}
        }
        
        structures
    }
    
    /// Generate configuration examples
    fn generate_config_examples(&self, config: &ComponentConfig) -> Vec<ConfigExample> {
        let mut examples = vec![
            ConfigExample {
                title: "Default Configuration".to_string(),
                description: "Basic configuration with default values".to_string(),
                format: "yaml".to_string(),
                content: self.generate_default_config_yaml(config),
            },
            ConfigExample {
                title: "Production Configuration".to_string(),
                description: "Recommended configuration for production environments".to_string(),
                format: "yaml".to_string(),
                content: self.generate_production_config_yaml(config),
            },
        ];
        
        if config.category_str() == "auth" {
            examples.push(ConfigExample {
                title: "High Security Configuration".to_string(),
                description: "Configuration with enhanced security settings".to_string(),
                format: "yaml".to_string(),
                content: self.generate_security_config_yaml(config),
            });
        }
        
        examples
    }
    
    /// Generate default configuration YAML
    fn generate_default_config_yaml(&self, config: &ComponentConfig) -> String {
        format!(r#"# Default configuration for {}
components:
  - name: "{}"
    config:
      # Add default configuration values here
      enabled: true"#, config.name, config.name)
    }
    
    /// Generate production configuration YAML
    fn generate_production_config_yaml(&self, config: &ComponentConfig) -> String {
        format!(r#"# Production configuration for {}
components:
  - name: "{}"
    config:
      # Production-ready configuration
      enabled: true
      log_level: "info"
      monitoring_enabled: true"#, config.name, config.name)
    }
    
    /// Generate security configuration YAML
    fn generate_security_config_yaml(&self, config: &ComponentConfig) -> String {
        format!(r#"# High security configuration for {}
components:
  - name: "{}"
    config:
      # Enhanced security settings
      enabled: true
      security_level: "high"
      audit_logging: true
      rate_limiting: true"#, config.name, config.name)
    }
    
    /// Generate integration steps
    fn generate_integration_steps(&self, config: &ComponentConfig) -> Vec<IntegrationStep> {
        vec![
            IntegrationStep {
                step: 1,
                title: "Add Component to Configuration".to_string(),
                description: format!("Add the {} component to your rustform.yml configuration file.", config.name),
                code: Some(format!(r#"components:
  - name: "{}"
    config:
      # Component configuration here"#, config.name)),
            },
            IntegrationStep {
                step: 2,
                title: "Generate Application".to_string(),
                description: "Run the Rust-form generator to include the component in your application.".to_string(),
                code: Some("rustform generate --config rustform.yml".to_string()),
            },
            IntegrationStep {
                step: 3,
                title: "Start Development Environment".to_string(),
                description: "Enter the Nix development shell to access all component dependencies.".to_string(),
                code: Some("nix develop".to_string()),
            },
            IntegrationStep {
                step: 4,
                title: "Build and Test".to_string(),
                description: "Build the application and run tests to verify component integration.".to_string(),
                code: Some("cargo build && cargo test".to_string()),
            },
        ]
    }
    
    /// Generate common usage patterns
    fn generate_common_patterns(&self, config: &ComponentConfig) -> Vec<UsagePattern> {
        match config.category_str() {
            "auth" => vec![
                UsagePattern {
                    title: "Basic Authentication".to_string(),
                    description: "Simple username/password authentication".to_string(),
                    code: "// Basic authentication example\nlet auth = AuthManager::new();\nlet token = auth.authenticate(username, password).await?;".to_string(),
                },
                UsagePattern {
                    title: "Middleware Integration".to_string(),
                    description: "Using authentication as middleware".to_string(),
                    code: "// Middleware integration\nlet app = Router::new()\n    .route(\"/protected\", get(protected_handler))\n    .layer(middleware::from_fn(auth_middleware));".to_string(),
                },
            ],
            "database" => vec![
                UsagePattern {
                    title: "Connection Pool".to_string(),
                    description: "Setting up database connection pool".to_string(),
                    code: "// Database connection pool\nlet pool = DatabasePool::new(&database_url).await?;\nlet conn = pool.acquire().await?;".to_string(),
                },
                UsagePattern {
                    title: "Transaction Management".to_string(),
                    description: "Using database transactions".to_string(),
                    code: "// Transaction management\nlet mut tx = pool.begin().await?;\n// Perform operations\ntx.commit().await?;".to_string(),
                },
            ],
            _ => vec![
                UsagePattern {
                    title: "Basic Usage".to_string(),
                    description: format!("Basic usage of {}", config.name),
                    code: format!("// Basic usage\nlet component = {}::new();\nlet result = component.process().await?;", 
                        config.name.replace("-", "").to_title_case()),
                },
            ],
        }
    }
    
    /// Generate tutorial steps
    fn generate_tutorial_steps(&self, config: &ComponentConfig) -> Vec<TutorialStep> {
        vec![
            TutorialStep {
                step: 1,
                title: "Installation".to_string(),
                content: format!("Learn how to install and configure the {} component in your Rust-form project.", config.name),
                code: Some(format!(r#"# Add to rustform.yml
components:
  - name: "{}"
    version: "{}"
    config:
      # Configuration options here"#, config.name, config.version)),
            },
            TutorialStep {
                step: 2,
                title: "Basic Configuration".to_string(),
                content: "Configure the component with basic settings for development.".to_string(),
                code: Some("# Development configuration\ndebug: true\nlog_level: \"debug\"".to_string()),
            },
            TutorialStep {
                step: 3,
                title: "First Steps".to_string(),
                content: "Create your first application using the component.".to_string(),
                code: Some(format!("use {}::*;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {{\n    let component = {}::new();\n    component.initialize().await?;\n    Ok(())\n}}", 
                    config.name.replace("-", "_"), 
                    config.name.replace("-", "").to_title_case())),
            },
        ]
    }
    
    /// Generate common issues and solutions
    fn generate_common_issues(&self, config: &ComponentConfig) -> Vec<Issue> {
        let mut issues = vec![
            Issue {
                title: "Component Not Found".to_string(),
                description: "The component is not being recognized or loaded.".to_string(),
                symptoms: vec![
                    "Component not found in registry".to_string(),
                    "Generation fails with component error".to_string(),
                ],
                solutions: vec![
                    "Verify component name in configuration".to_string(),
                    "Check component.yml syntax".to_string(),
                    "Ensure component directory structure is correct".to_string(),
                ],
            },
            Issue {
                title: "Dependency Conflicts".to_string(),
                description: "Conflicts between component dependencies and project dependencies.".to_string(),
                symptoms: vec![
                    "Cargo build fails with dependency errors".to_string(),
                    "Version conflicts in Cargo.lock".to_string(),
                ],
                solutions: vec![
                    "Update component to latest version".to_string(),
                    "Check dependency compatibility".to_string(),
                    "Use cargo tree to identify conflicts".to_string(),
                ],
            },
        ];
        
        // Add category-specific issues
        match config.category_str() {
            "auth" => {
                issues.push(Issue {
                    title: "Authentication Failures".to_string(),
                    description: "Users cannot authenticate or tokens are invalid.".to_string(),
                    symptoms: vec![
                        "401 Unauthorized errors".to_string(),
                        "Token validation failures".to_string(),
                    ],
                    solutions: vec![
                        "Check secret key configuration".to_string(),
                        "Verify token expiration settings".to_string(),
                        "Ensure proper time synchronization".to_string(),
                    ],
                });
            },
            "database" => {
                issues.push(Issue {
                    title: "Connection Pool Exhausted".to_string(),
                    description: "Database connection pool runs out of connections.".to_string(),
                    symptoms: vec![
                        "Timeout errors on database operations".to_string(),
                        "Connection pool exhausted messages".to_string(),
                    ],
                    solutions: vec![
                        "Increase max_connections setting".to_string(),
                        "Implement connection retry logic".to_string(),
                        "Check for connection leaks".to_string(),
                    ],
                });
            },
            _ => {}
        }
        
        issues
    }
    
    /// Determine example type from name
    fn determine_example_type(&self, example_name: &str) -> String {
        if example_name.contains("basic") {
            "basic".to_string()
        } else if example_name.contains("advanced") {
            "advanced".to_string()
        } else if example_name.contains("config") {
            "configuration".to_string()
        } else {
            "generic".to_string()
        }
    }
}

// Helper structs for documentation generation

#[derive(Debug, Clone, serde::Serialize)]
pub struct Shield {
    pub label: String,
    pub message: String,
    pub color: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TocEntry {
    pub title: String,
    pub anchor: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiEndpoint {
    pub method: String,
    pub path: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub responses: Vec<Response>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Response {
    pub status: u16,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DataStructure {
    pub name: String,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ConfigExample {
    pub title: String,
    pub description: String,
    pub format: String,
    pub content: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IntegrationStep {
    pub step: u32,
    pub title: String,
    pub description: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UsagePattern {
    pub title: String,
    pub description: String,
    pub code: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TutorialStep {
    pub step: u32,
    pub title: String,
    pub content: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub symptoms: Vec<String>,
    pub solutions: Vec<String>,
}

/// Documentation generation errors
#[derive(Debug, thiserror::Error)]
pub enum DocGenerationError {
    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid documentation configuration: {0}")]
    InvalidDocConfig(String),
}

// Template filters for documentation

/// Escape markdown special characters
fn markdown_escape_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let text = value.as_str().ok_or_else(|| {
        tera::Error::msg("markdown_escape filter can only be applied to strings")
    })?;
    
    let escaped = text
        .replace("\\", "\\\\")
        .replace("*", "\\*")
        .replace("_", "\\_")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace(".", "\\.")
        .replace("!", "\\!");
    
    Ok(tera::Value::String(escaped))
}

/// Format code block with syntax highlighting
fn code_block_filter(value: &tera::Value, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let code = value.as_str().ok_or_else(|| {
        tera::Error::msg("code_block filter can only be applied to strings")
    })?;
    
    let language = args.get("lang")
        .and_then(|v| v.as_str())
        .unwrap_or("rust");
    
    let formatted = format!("```{}\n{}\n```", language, code);
    Ok(tera::Value::String(formatted))
}

/// Generate API documentation link
fn api_link_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let item_name = value.as_str().ok_or_else(|| {
        tera::Error::msg("api_link filter can only be applied to strings")
    })?;
    
    let link = format!("[`{}`](docs/api.md#{})", item_name, item_name.to_lowercase().replace(" ", "-"));
    Ok(tera::Value::String(link))
}

/// Generate example name
fn example_name_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let filename = value.as_str().ok_or_else(|| {
        tera::Error::msg("example_name filter can only be applied to strings")
    })?;
    
    let name = filename
        .replace(".rs", "")
        .replace("_", " ")
        .replace("-", " ");
    
    let title_case = name
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    
    Ok(tera::Value::String(title_case))
}

trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }
}