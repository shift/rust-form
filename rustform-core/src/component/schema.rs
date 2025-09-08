use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component configuration schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    /// Component metadata
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: ComponentCategory,
    pub priority: Priority,
    pub complexity: Complexity,
    
    /// Dependencies configuration
    pub dependencies: DependencyConfig,
    
    /// Template configuration
    pub templates: TemplateConfig,
    
    /// Test generation configuration
    pub tests: TestConfig,
    
    /// Documentation configuration
    pub documentation: DocumentationConfig,
    
    /// Configuration schema for the component
    pub config_schema: Option<HashMap<String, ConfigField>>,
    
    /// Environment variables
    pub environment: Option<HashMap<String, String>>,
    
    /// Build configuration
    pub build_config: Option<BuildConfig>,
    
    /// Setup commands to run in dev shell
    pub setup_commands: Option<Vec<String>>,
    
    /// Features configuration
    pub features: Option<FeaturesConfig>,
}

/// Component categories
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentCategory {
    Auth,
    Database,
    Api,
    Middleware,
    File,
    Communication,
    Payment,
    Search,
    Jobs,
    Monitoring,
    Security,
    Integration,
    Ui,
    Layout,
    Forms,
    Visualization,
    Navigation,
}

/// Component priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// Component complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    Low,
    Medium,
    High,
}

/// Dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConfig {
    /// Rust crate dependencies
    pub rust: Vec<String>,
    
    /// Test-specific dependencies
    pub test_deps: Option<Vec<String>>,
    
    /// Nix package dependencies
    pub nix: NixDependencies,
    
    /// Flake input dependencies
    pub flake_inputs: Option<Vec<FlakeInput>>,
}

/// Nix package dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixDependencies {
    /// Build inputs (runtime dependencies)
    pub buildInputs: Vec<String>,
    
    /// Native build inputs (build-time tools)
    pub nativeBuildInputs: Vec<String>,
    
    /// Development shell configuration
    pub devShell: DevShellConfig,
}

/// Development shell configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevShellConfig {
    /// Additional packages for development
    pub packages: Vec<String>,
}

/// Flake input dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeInput {
    pub name: String,
    pub url: String,
}

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Templates that this component generates
    pub generates: Vec<String>,
    
    /// Templates that this component requires
    pub requires: Option<Vec<String>>,
}

/// Test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Unit tests to generate
    pub unit: Vec<String>,
    
    /// Integration tests to generate
    pub integration: Option<Vec<String>>,
    
    /// Performance tests to generate
    pub performance: Option<Vec<String>>,
}

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Example files to generate
    pub examples: Vec<String>,
    
    /// Whether to generate API docs
    pub api_docs: bool,
    
    /// Whether to generate tutorial
    pub tutorial: bool,
}

/// Configuration field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    /// Field type
    #[serde(rename = "type")]
    pub field_type: ConfigFieldType,
    
    /// Whether the field is required
    pub required: Option<bool>,
    
    /// Default value
    pub default: Option<serde_yaml::Value>,
    
    /// Field description
    pub description: String,
}

/// Configuration field types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFieldType {
    String,
    Integer,
    Boolean,
    Duration,
    Array,
    Object,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Cargo features to enable
    pub features: Option<Vec<String>>,
    
    /// Custom build script
    pub build_script: Option<String>,
}

/// Features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// Default features
    pub default: Vec<String>,
    
    /// Optional features with their dependencies
    pub optional: Option<HashMap<String, Vec<String>>>,
}

impl ComponentConfig {
    /// Parse component configuration from YAML
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
    
    /// Convert to YAML
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
    
    /// Validate the component configuration
    pub fn validate(&self) -> Result<(), ComponentConfigError> {
        // Validate component name
        if self.name.is_empty() {
            return Err(ComponentConfigError::InvalidName("Component name cannot be empty".to_string()));
        }
        
        if !self.name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(ComponentConfigError::InvalidName(
                "Component name can only contain alphanumeric characters, hyphens, and underscores".to_string()
            ));
        }
        
        // Validate version format
        if !self.version.chars().any(|c| c.is_ascii_digit()) {
            return Err(ComponentConfigError::InvalidVersion(
                "Version must contain at least one digit".to_string()
            ));
        }
        
        // Validate dependencies
        for dep in &self.dependencies.rust {
            if dep.is_empty() {
                return Err(ComponentConfigError::InvalidDependency(
                    "Rust dependency cannot be empty".to_string()
                ));
            }
        }
        
        // Validate templates
        for template in &self.templates.generates {
            if !template.ends_with(".rs") && !template.ends_with(".toml") && !template.ends_with(".yml") {
                return Err(ComponentConfigError::InvalidTemplate(
                    format!("Template '{}' must have a valid file extension", template)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Get the component's category as a string
    pub fn category_str(&self) -> &'static str {
        match self.category {
            ComponentCategory::Auth => "auth",
            ComponentCategory::Database => "database",
            ComponentCategory::Api => "api",
            ComponentCategory::Middleware => "middleware",
            ComponentCategory::File => "file",
            ComponentCategory::Communication => "communication",
            ComponentCategory::Payment => "payment",
            ComponentCategory::Search => "search",
            ComponentCategory::Jobs => "jobs",
            ComponentCategory::Monitoring => "monitoring",
            ComponentCategory::Security => "security",
            ComponentCategory::Integration => "integration",
            ComponentCategory::Ui => "ui",
            ComponentCategory::Layout => "layout",
            ComponentCategory::Forms => "forms",
            ComponentCategory::Visualization => "visualization",
            ComponentCategory::Navigation => "navigation",
        }
    }
    
    /// Get the component's priority as a string
    pub fn priority_str(&self) -> &'static str {
        match self.priority {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }
    
    /// Get the component's complexity as a string
    pub fn complexity_str(&self) -> &'static str {
        match self.complexity {
            Complexity::Low => "low",
            Complexity::Medium => "medium",
            Complexity::High => "high",
        }
    }
    
    /// Get all dependencies (rust + test)
    pub fn all_rust_dependencies(&self) -> Vec<String> {
        let mut deps = self.dependencies.rust.clone();
        if let Some(test_deps) = &self.dependencies.test_deps {
            deps.extend(test_deps.clone());
        }
        deps
    }
    
    /// Get the component directory path
    pub fn component_dir(&self) -> String {
        format!("components/{}/{}", self.category_str(), self.name)
    }
}

/// Component configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ComponentConfigError {
    #[error("Invalid component name: {0}")]
    InvalidName(String),
    
    #[error("Invalid version: {0}")]
    InvalidVersion(String),
    
    #[error("Invalid dependency: {0}")]
    InvalidDependency(String),
    
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

impl ConfigFieldType {
    /// Convert to Rust type string
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            ConfigFieldType::String => "String",
            ConfigFieldType::Integer => "i32",
            ConfigFieldType::Boolean => "bool",
            ConfigFieldType::Duration => "std::time::Duration",
            ConfigFieldType::Array => "Vec<String>",
            ConfigFieldType::Object => "serde_json::Value",
        }
    }
    
    /// Get default value for type
    pub fn default_value(&self) -> &'static str {
        match self {
            ConfigFieldType::String => "String::new()",
            ConfigFieldType::Integer => "0",
            ConfigFieldType::Boolean => "false",
            ConfigFieldType::Duration => "std::time::Duration::from_secs(0)",
            ConfigFieldType::Array => "Vec::new()",
            ConfigFieldType::Object => "serde_json::Value::Null",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_config_parsing() {
        let yaml = r#"
name: "jwt-manager"
description: "JWT token management"
version: "1.0.0"
category: "auth"
priority: "high"
complexity: "medium"
dependencies:
  rust:
    - "jsonwebtoken = \"9.0\""
    - "serde = \"1.0\""
  nix:
    buildInputs:
      - "openssl"
    nativeBuildInputs:
      - "rustc"
    devShell:
      packages:
        - "rust-analyzer"
templates:
  generates:
    - "handlers/auth.rs"
tests:
  unit:
    - "test_token_creation"
documentation:
  examples:
    - "basic_usage.rs"
  api_docs: true
  tutorial: true
"#;
        
        let config = ComponentConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.name, "jwt-manager");
        assert_eq!(config.description, "JWT token management");
        assert!(matches!(config.category, ComponentCategory::Auth));
        assert!(matches!(config.priority, Priority::High));
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_component_name() {
        let mut config = ComponentConfig {
            name: "invalid name!".to_string(),
            description: "Test".to_string(),
            version: "1.0.0".to_string(),
            category: ComponentCategory::Auth,
            priority: Priority::High,
            complexity: Complexity::Medium,
            dependencies: DependencyConfig {
                rust: vec![],
                test_deps: None,
                nix: NixDependencies {
                    buildInputs: vec![],
                    nativeBuildInputs: vec![],
                    devShell: DevShellConfig {
                        packages: vec![],
                    },
                },
                flake_inputs: None,
            },
            templates: TemplateConfig {
                generates: vec![],
                requires: None,
            },
            tests: TestConfig {
                unit: vec![],
                integration: None,
                performance: None,
            },
            documentation: DocumentationConfig {
                examples: vec![],
                api_docs: false,
                tutorial: false,
            },
            config_schema: None,
            environment: None,
            build_config: None,
            setup_commands: None,
            features: None,
        };
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_component_directory_path() {
        let config = ComponentConfig {
            name: "jwt-manager".to_string(),
            category: ComponentCategory::Auth,
            // ... other fields with defaults
            description: "Test".to_string(),
            version: "1.0.0".to_string(),
            priority: Priority::High,
            complexity: Complexity::Medium,
            dependencies: DependencyConfig {
                rust: vec![],
                test_deps: None,
                nix: NixDependencies {
                    buildInputs: vec![],
                    nativeBuildInputs: vec![],
                    devShell: DevShellConfig {
                        packages: vec![],
                    },
                },
                flake_inputs: None,
            },
            templates: TemplateConfig {
                generates: vec![],
                requires: None,
            },
            tests: TestConfig {
                unit: vec![],
                integration: None,
                performance: None,
            },
            documentation: DocumentationConfig {
                examples: vec![],
                api_docs: false,
                tutorial: false,
            },
            config_schema: None,
            environment: None,
            build_config: None,
            setup_commands: None,
            features: None,
        };
        
        assert_eq!(config.component_dir(), "components/auth/jwt-manager");
    }
}