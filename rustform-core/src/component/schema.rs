use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API compatibility information for components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCompatibility {
    /// The rust-form API version this component was developed against
    pub api_version: String,
    /// Minimum supported rust-form API version
    pub min_version: String,
    /// Maximum tested rust-form API version (optional)
    pub max_version: Option<String>,
    /// Specific rust-form features this component depends on
    pub required_features: Option<Vec<String>>,
    /// Whether this component uses experimental APIs
    pub experimental: Option<bool>,
}

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
    
    /// API versioning information
    pub api_compatibility: ApiCompatibility,
    
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
        
        // Validate API compatibility
        self.validate_api_compatibility()?;
        
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
    
    /// Validate API compatibility information
    pub fn validate_api_compatibility(&self) -> Result<(), ComponentConfigError> {
        // Validate API version format (semver)
        if !is_valid_semver(&self.api_compatibility.api_version) {
            return Err(ComponentConfigError::InvalidApiVersion(
                format!("Invalid API version format: {}", self.api_compatibility.api_version)
            ));
        }
        
        // Validate min version format
        if !is_valid_semver(&self.api_compatibility.min_version) {
            return Err(ComponentConfigError::InvalidApiVersion(
                format!("Invalid min version format: {}", self.api_compatibility.min_version)
            ));
        }
        
        // Validate max version format if provided
        if let Some(ref max_version) = self.api_compatibility.max_version {
            if !is_valid_semver(max_version) {
                return Err(ComponentConfigError::InvalidApiVersion(
                    format!("Invalid max version format: {}", max_version)
                ));
            }
        }
        
        // Validate version range logic
        if let Some(ref max_version) = self.api_compatibility.max_version {
            if compare_versions(&self.api_compatibility.min_version, max_version)? > 0 {
                return Err(ComponentConfigError::InvalidApiVersion(
                    "Minimum version cannot be greater than maximum version".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check if this component is compatible with a given rust-form API version
    pub fn is_compatible_with(&self, rust_form_version: &str) -> Result<bool, ComponentConfigError> {
        if !is_valid_semver(rust_form_version) {
            return Err(ComponentConfigError::InvalidApiVersion(
                format!("Invalid rust-form version format: {}", rust_form_version)
            ));
        }
        
        // Check minimum version
        if compare_versions(rust_form_version, &self.api_compatibility.min_version)? < 0 {
            return Ok(false);
        }
        
        // Check maximum version if specified
        if let Some(ref max_version) = self.api_compatibility.max_version {
            if compare_versions(rust_form_version, max_version)? > 0 {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Get compatibility status with detailed information
    pub fn compatibility_status(&self, rust_form_version: &str) -> Result<CompatibilityStatus, ComponentConfigError> {
        if !is_valid_semver(rust_form_version) {
            return Err(ComponentConfigError::InvalidApiVersion(
                format!("Invalid rust-form version format: {}", rust_form_version)
            ));
        }
        
        let min_cmp = compare_versions(rust_form_version, &self.api_compatibility.min_version)?;
        
        // Too old
        if min_cmp < 0 {
            return Ok(CompatibilityStatus::TooOld {
                current: rust_form_version.to_string(),
                required_min: self.api_compatibility.min_version.clone(),
            });
        }
        
        // Check maximum version if specified
        if let Some(ref max_version) = self.api_compatibility.max_version {
            let max_cmp = compare_versions(rust_form_version, max_version)?;
            if max_cmp > 0 {
                return Ok(CompatibilityStatus::TooNew {
                    current: rust_form_version.to_string(),
                    supported_max: max_version.clone(),
                });
            }
        }
        
        // Check if experimental
        if self.api_compatibility.experimental.unwrap_or(false) {
            return Ok(CompatibilityStatus::CompatibleExperimental {
                current: rust_form_version.to_string(),
                component_api_version: self.api_compatibility.api_version.clone(),
            });
        }
        
        Ok(CompatibilityStatus::Compatible {
            current: rust_form_version.to_string(),
            component_api_version: self.api_compatibility.api_version.clone(),
        })
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
    
    #[error("Invalid API version: {0}")]
    InvalidApiVersion(String),
    
    #[error("Invalid dependency: {0}")]
    InvalidDependency(String),
    
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

/// Compatibility status between component and rust-form version
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityStatus {
    /// Component is compatible
    Compatible {
        current: String,
        component_api_version: String,
    },
    /// Component is compatible but uses experimental APIs
    CompatibleExperimental {
        current: String,
        component_api_version: String,
    },
    /// rust-form version is too old for this component
    TooOld {
        current: String,
        required_min: String,
    },
    /// rust-form version is newer than component's tested maximum
    TooNew {
        current: String,
        supported_max: String,
    },
}

impl CompatibilityStatus {
    /// Check if the status indicates compatibility
    pub fn is_compatible(&self) -> bool {
        matches!(self, CompatibilityStatus::Compatible { .. } | CompatibilityStatus::CompatibleExperimental { .. })
    }
    
    /// Get a human-readable message
    pub fn message(&self) -> String {
        match self {
            CompatibilityStatus::Compatible { current, component_api_version } => {
                format!("✓ Compatible (rust-form {} ≥ component API {})", current, component_api_version)
            }
            CompatibilityStatus::CompatibleExperimental { current, component_api_version } => {
                format!("⚠ Compatible but experimental (rust-form {} ≥ component API {})", current, component_api_version)
            }
            CompatibilityStatus::TooOld { current, required_min } => {
                format!("✗ rust-form {} is too old, requires ≥ {}", current, required_min)
            }
            CompatibilityStatus::TooNew { current, supported_max } => {
                format!("⚠ rust-form {} is newer than tested maximum {}", current, supported_max)
            }
        }
    }
}

/// Check if a version string follows semver format
fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    parts.iter().all(|part| {
        part.chars().all(|c| c.is_ascii_digit()) && !part.is_empty()
    })
}

/// Compare two semver versions
/// Returns: -1 if v1 < v2, 0 if v1 == v2, 1 if v1 > v2
fn compare_versions(v1: &str, v2: &str) -> Result<i32, ComponentConfigError> {
    let parse_version = |v: &str| -> Result<(u32, u32, u32), ComponentConfigError> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return Err(ComponentConfigError::InvalidApiVersion(format!("Invalid version format: {}", v)));
        }
        
        let major = parts[0].parse::<u32>()
            .map_err(|_| ComponentConfigError::InvalidApiVersion(format!("Invalid major version: {}", parts[0])))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| ComponentConfigError::InvalidApiVersion(format!("Invalid minor version: {}", parts[1])))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| ComponentConfigError::InvalidApiVersion(format!("Invalid patch version: {}", parts[2])))?;
        
        Ok((major, minor, patch))
    };
    
    let (major1, minor1, patch1) = parse_version(v1)?;
    let (major2, minor2, patch2) = parse_version(v2)?;
    
    match major1.cmp(&major2) {
        std::cmp::Ordering::Less => Ok(-1),
        std::cmp::Ordering::Greater => Ok(1),
        std::cmp::Ordering::Equal => {
            match minor1.cmp(&minor2) {
                std::cmp::Ordering::Less => Ok(-1),
                std::cmp::Ordering::Greater => Ok(1),
                std::cmp::Ordering::Equal => {
                    match patch1.cmp(&patch2) {
                        std::cmp::Ordering::Less => Ok(-1),
                        std::cmp::Ordering::Greater => Ok(1),
                        std::cmp::Ordering::Equal => Ok(0),
                    }
                }
            }
        }
    }
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
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
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
        assert_eq!(config.api_compatibility.api_version, "0.1.0");
        assert_eq!(config.api_compatibility.min_version, "0.1.0");
        assert_eq!(config.api_compatibility.max_version, Some("0.2.0".to_string()));
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_api_compatibility_validation() {
        let mut config = create_test_component_config();
        
        // Test invalid API version
        config.api_compatibility.api_version = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Test invalid min version
        config.api_compatibility.api_version = "0.1.0".to_string();
        config.api_compatibility.min_version = "not-semver".to_string();
        assert!(config.validate().is_err());
        
        // Test min > max version
        config.api_compatibility.min_version = "0.2.0".to_string();
        config.api_compatibility.max_version = Some("0.1.0".to_string());
        assert!(config.validate().is_err());
        
        // Test valid configuration
        config.api_compatibility.min_version = "0.1.0".to_string();
        config.api_compatibility.max_version = Some("0.2.0".to_string());
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_version_compatibility_checking() {
        let config = create_test_component_config();
        
        // Test compatible version
        assert!(config.is_compatible_with("0.1.5").unwrap());
        
        // Test too old version
        assert!(!config.is_compatible_with("0.0.9").unwrap());
        
        // Test too new version (if max is specified)
        if config.api_compatibility.max_version.is_some() {
            assert!(!config.is_compatible_with("0.3.0").unwrap());
        }
        
        // Test exact versions
        assert!(config.is_compatible_with(&config.api_compatibility.min_version).unwrap());
        if let Some(ref max_version) = config.api_compatibility.max_version {
            assert!(config.is_compatible_with(max_version).unwrap());
        }
    }
    
    #[test]
    fn test_compatibility_status() {
        let config = create_test_component_config();
        
        // Test compatible
        let status = config.compatibility_status("0.1.5").unwrap();
        assert!(status.is_compatible());
        assert!(status.message().contains("Compatible"));
        
        // Test too old
        let status = config.compatibility_status("0.0.9").unwrap();
        assert!(!status.is_compatible());
        assert!(status.message().contains("too old"));
        
        // Test experimental component
        let mut experimental_config = create_test_component_config();
        experimental_config.api_compatibility.experimental = Some(true);
        let status = experimental_config.compatibility_status("0.1.5").unwrap();
        assert!(status.is_compatible());
        assert!(status.message().contains("experimental"));
    }
    
    fn create_test_component_config() -> ComponentConfig {
        ComponentConfig {
            name: "test-component".to_string(),
            description: "Test component".to_string(),
            version: "1.0.0".to_string(),
            category: ComponentCategory::Auth,
            priority: Priority::High,
            complexity: Complexity::Medium,
            api_compatibility: ApiCompatibility {
                api_version: "0.1.0".to_string(),
                min_version: "0.1.0".to_string(),
                max_version: Some("0.2.0".to_string()),
                required_features: None,
                experimental: None,
            },
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
        }
    }
    
    #[test]
    fn test_invalid_component_name() {
        let mut config = create_test_component_config();
        config.name = "invalid name!".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_component_directory_path() {
        let mut config = create_test_component_config();
        config.name = "jwt-manager".to_string();
        assert_eq!(config.component_dir(), "components/auth/jwt-manager");
    }
}