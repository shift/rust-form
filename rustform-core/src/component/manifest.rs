use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::Result;

/// API compatibility information for external components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Extended dependency configuration supporting multiple ecosystems
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentDependencies {
    #[serde(default)]
    pub rust: Vec<String>,
    #[serde(default)]
    pub nix: Option<NixDependencies>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NixDependencies {
    #[serde(rename = "buildInputs")]
    pub build_inputs: Option<Vec<String>>,
    #[serde(rename = "nativeBuildInputs")]
    pub native_build_inputs: Option<Vec<String>>,
    #[serde(rename = "devShell")]
    pub dev_shell: Option<NixDevShell>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NixDevShell {
    pub packages: Option<Vec<String>>,
}

/// Configuration field specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigField {
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: Option<bool>,
    pub default: Option<serde_yaml::Value>,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    pub items: Option<String>,
    pub properties: Option<HashMap<String, String>>,
}

/// Compliance information for regulated components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub regulation: Option<String>,
    pub articles_implemented: Option<Vec<String>>,
    pub data_categories: Option<Vec<String>>,
    pub retention_periods: Option<HashMap<String, String>>,
    pub security_measures: Option<Vec<String>>,
}

/// Test configuration for components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestConfiguration {
    pub unit: Option<Vec<String>>,
    pub integration: Option<Vec<String>>,
    pub compliance: Option<Vec<String>>,
    pub performance: Option<Vec<String>>,
    pub security: Option<Vec<String>>,
}

/// Documentation configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentationConfig {
    pub compliance_guide: Option<bool>,
    pub dpo_manual: Option<bool>,
    pub api_reference: Option<bool>,
    pub implementation_checklist: Option<bool>,
    pub privacy_notice_template: Option<bool>,
    pub cookie_policy_template: Option<bool>,
    pub data_processing_records: Option<bool>,
    pub breach_response_procedures: Option<bool>,
}

/// Template configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub generates: Option<Vec<String>>,
    pub requires: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    
    // Extended fields for complex components
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub priority: Option<String>,
    pub complexity: Option<String>,
    
    /// API versioning information
    pub api_compatibility: ApiCompatibility,
    
    pub dependencies: ComponentDependencies,
    pub provides: Option<ComponentInterface>,
    
    // Extended configuration and features
    pub config_schema: Option<HashMap<String, ConfigField>>,
    pub compliance: Option<ComplianceInfo>,
    pub tests: Option<TestConfiguration>,
    pub documentation: Option<DocumentationConfig>,
    pub features: Option<HashMap<String, Vec<String>>>,
    pub templates: Option<TemplateConfig>,
    
    pub integrity: Option<String>,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentInterface {
    pub templates: Vec<TemplateSpec>,
    pub assets: Vec<AssetSpec>,
    pub hooks: Vec<HookSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateSpec {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub variables: Vec<VariableSpec>,
    pub target: TemplateTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateTarget {
    Frontend,
    Backend,
    Migration,
    Config,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableSpec {
    pub name: String,
    pub type_name: String,
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetSpec {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Style,
    Script,
    Image,
    Font,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookSpec {
    pub name: String,
    pub phase: HookPhase,
    pub script: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookPhase {
    PreGenerate,
    PostGenerate,
    PreBuild,
    PostBuild,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub manifest: ComponentManifest,
    pub content: ComponentContent,
    pub resolved_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ComponentContent {
    pub templates: HashMap<String, String>,
    pub assets: HashMap<String, Vec<u8>>,
    pub hooks: HashMap<String, String>,
}

impl ComponentManifest {
    pub fn from_yaml(content: &str) -> Result<Self> {
        serde_yaml::from_str(content).map_err(Into::into)
    }

    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    /// Get category as string, defaulting to "general" if not specified
    pub fn category_str(&self) -> &str {
        self.category.as_deref().unwrap_or("general")
    }

    /// Get priority as string, defaulting to "medium" if not specified
    pub fn priority_str(&self) -> &str {
        self.priority.as_deref().unwrap_or("medium")
    }

    /// Get complexity as string, defaulting to "medium" if not specified
    pub fn complexity_str(&self) -> &str {
        self.complexity.as_deref().unwrap_or("medium")
    }

    /// Convert complex dependencies to simple HashMap for compatibility
    pub fn dependencies_as_hashmap(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        // For now, convert rust dependencies to simple name -> version mapping
        for dep_line in &self.dependencies.rust {
            if let Some(eq_pos) = dep_line.find('=') {
                let dep_name = dep_line[..eq_pos].trim().to_string();
                let version_part = dep_line[eq_pos + 1..].trim();
                
                // Extract version from various formats
                let version = if version_part.starts_with('"') && version_part.ends_with('"') {
                    version_part.trim_matches('"').to_string()
                } else if version_part.contains("version") {
                    if let Some(version_start) = version_part.find("version") {
                        let after_version = &version_part[version_start + 7..];
                        if let Some(quote_start) = after_version.find('"') {
                            let after_quote = &after_version[quote_start + 1..];
                            if let Some(quote_end) = after_quote.find('"') {
                                after_quote[..quote_end].to_string()
                            } else {
                                "latest".to_string()
                            }
                        } else {
                            "latest".to_string()
                        }
                    } else {
                        "latest".to_string()
                    }
                } else {
                    "latest".to_string()
                };
                
                result.insert(dep_name, version);
            }
        }
        
        result
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(crate::error::Error::ValidationError(
                "Component name cannot be empty".to_string()
            ));
        }

        if self.version.is_empty() {
            return Err(crate::error::Error::ValidationError(
                "Component version cannot be empty".to_string()
            ));
        }

        // Validate API compatibility
        self.validate_api_compatibility()?;

        Ok(())
    }
    
    /// Validate API compatibility information
    pub fn validate_api_compatibility(&self) -> Result<()> {
        // Validate API version format (semver)
        if !is_valid_semver(&self.api_compatibility.api_version) {
            return Err(crate::error::Error::ValidationError(
                format!("Invalid API version format: {}", self.api_compatibility.api_version)
            ));
        }
        
        // Validate min version format
        if !is_valid_semver(&self.api_compatibility.min_version) {
            return Err(crate::error::Error::ValidationError(
                format!("Invalid min version format: {}", self.api_compatibility.min_version)
            ));
        }
        
        // Validate max version format if provided
        if let Some(ref max_version) = self.api_compatibility.max_version {
            if !is_valid_semver(max_version) {
                return Err(crate::error::Error::ValidationError(
                    format!("Invalid max version format: {}", max_version)
                ));
            }
        }
        
        // Validate version range logic
        if let Some(ref max_version) = self.api_compatibility.max_version {
            if compare_versions(&self.api_compatibility.min_version, max_version)? > 0 {
                return Err(crate::error::Error::ValidationError(
                    "Minimum version cannot be greater than maximum version".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check if this component is compatible with a given rust-form API version
    pub fn is_compatible_with(&self, rust_form_version: &str) -> Result<bool> {
        if !is_valid_semver(rust_form_version) {
            return Err(crate::error::Error::ValidationError(
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
    pub fn compatibility_status(&self, rust_form_version: &str) -> Result<CompatibilityStatus> {
        if !is_valid_semver(rust_form_version) {
            return Err(crate::error::Error::ValidationError(
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
fn compare_versions(v1: &str, v2: &str) -> Result<i32> {
    let parse_version = |v: &str| -> Result<(u32, u32, u32)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return Err(crate::error::Error::ValidationError(format!("Invalid version format: {}", v)));
        }
        
        let major = parts[0].parse::<u32>()
            .map_err(|_| crate::error::Error::ValidationError(format!("Invalid major version: {}", parts[0])))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| crate::error::Error::ValidationError(format!("Invalid minor version: {}", parts[1])))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| crate::error::Error::ValidationError(format!("Invalid patch version: {}", parts[2])))?;
        
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

impl Default for ComponentInterface {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
            assets: Vec::new(),
            hooks: Vec::new(),
        }
    }
}

impl Default for ComponentDependencies {
    fn default() -> Self {
        Self {
            rust: Vec::new(),
            nix: None,
        }
    }
}