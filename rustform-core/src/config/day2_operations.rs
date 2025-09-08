use crate::config::Config;
use crate::error::{ValidationError, ValidationResult};

/// Day 2 operations for configuration management
pub struct Day2Operations;

impl Day2Operations {
    /// Check for deprecated features and emit warnings
    pub fn check_deprecations(config: &Config) -> Vec<DeprecationWarning> {
        let mut warnings = Vec::new();
        
        // Check schema version for deprecations
        if let Ok((major, minor, _)) = parse_version(&config.schema_version) {
            if major == 0 && minor < 9 {
                warnings.push(DeprecationWarning {
                    feature: "Schema version < 0.9.0".to_string(),
                    message: "This schema version is deprecated and will be removed in v2.0.0".to_string(),
                    suggestion: "Update schema_version to 1.0.0 or newer".to_string(),
                    removal_version: Some("2.0.0".to_string()),
                });
            }
        }
        
        // Check for deprecated middleware configurations
        for middleware in &config.middleware {
            match middleware {
                crate::config::MiddlewareConfig::Security { security } => {
                    if security.helmet == Some(true) {
                        warnings.push(DeprecationWarning {
                            feature: "helmet middleware option".to_string(),
                            message: "The 'helmet' option is deprecated".to_string(),
                            suggestion: "Use individual security headers configuration instead".to_string(),
                            removal_version: Some("0.2.0".to_string()),
                        });
                    }
                }
                _ => {}
            }
        }
        
        warnings
    }
    
    /// Generate migration guide from old to new schema version
    pub fn generate_migration_guide(from_version: &str, to_version: &str) -> ValidationResult<MigrationGuide> {
        let (from_major, from_minor, _) = parse_version(from_version)?;
        let (to_major, to_minor, _) = parse_version(to_version)?;
        
        let mut steps = Vec::new();
        
        // Migration from 0.x to 1.0
        if from_major == 0 && to_major == 1 {
            steps.push(MigrationStep {
                description: "Add schema_version and api_version fields".to_string(),
                automated: true,
                required: true,
                yaml_changes: vec![
                    YamlChange {
                        path: "root".to_string(),
                        action: ChangeAction::Add,
                        old_value: None,
                        new_value: Some("schema_version: \"1.0.0\"".to_string()),
                    },
                    YamlChange {
                        path: "root".to_string(), 
                        action: ChangeAction::Add,
                        old_value: None,
                        new_value: Some("api_version: \"0.1.0\"".to_string()),
                    },
                ],
            });
            
            steps.push(MigrationStep {
                description: "Update middleware security configuration".to_string(),
                automated: false,
                required: false,
                yaml_changes: vec![
                    YamlChange {
                        path: "middleware.security.helmet".to_string(),
                        action: ChangeAction::Replace,
                        old_value: Some("helmet: true".to_string()),
                        new_value: Some("x_frame_options: \"DENY\"".to_string()),
                    },
                ],
            });
        }
        
        Ok(MigrationGuide {
            from_version: from_version.to_string(),
            to_version: to_version.to_string(),
            breaking_changes: from_major != to_major,
            steps,
        })
    }
    
    /// Check compatibility matrix for components and features
    pub fn check_compatibility_matrix(config: &Config) -> CompatibilityReport {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
        // Check API version compatibility
        if let Err(e) = validate_api_compatibility(&config.api_version) {
            issues.push(CompatibilityIssue {
                severity: Severity::Error,
                component: "rust-form core".to_string(),
                message: format!("API version incompatibility: {}", e),
                resolution: "Update api_version in configuration or upgrade rust-form".to_string(),
            });
        }
        
        // Check feature flag compatibility
        #[cfg(feature = "registry")]
        if config.registry.is_some() {
            warnings.push(CompatibilityIssue {
                severity: Severity::Warning,
                component: "registry".to_string(),
                message: "Registry feature is experimental".to_string(),
                resolution: "Use with caution in production environments".to_string(),
            });
        }
        
        // Check component compatibility if components are defined
        if let Some(components) = &config.components {
            for (name, version) in components {
                if let Err(_) = parse_version(version) {
                    issues.push(CompatibilityIssue {
                        severity: Severity::Error,
                        component: name.clone(),
                        message: "Invalid component version".to_string(),
                        resolution: "Use semantic versioning (e.g., 1.0.0)".to_string(),
                    });
                }
            }
        }
        
        CompatibilityReport {
            rust_form_version: "0.1.0".to_string(),
            config_api_version: config.api_version.clone(),
            config_schema_version: config.schema_version.clone(),
            issues,
            warnings,
            overall_compatible: issues.is_empty(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeprecationWarning {
    pub feature: String,
    pub message: String,
    pub suggestion: String,
    pub removal_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MigrationGuide {
    pub from_version: String,
    pub to_version: String,
    pub breaking_changes: bool,
    pub steps: Vec<MigrationStep>,
}

#[derive(Debug, Clone)]
pub struct MigrationStep {
    pub description: String,
    pub automated: bool,
    pub required: bool,
    pub yaml_changes: Vec<YamlChange>,
}

#[derive(Debug, Clone)]
pub struct YamlChange {
    pub path: String,
    pub action: ChangeAction,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ChangeAction {
    Add,
    Remove,
    Replace,
    Move,
}

#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    pub rust_form_version: String,
    pub config_api_version: String,
    pub config_schema_version: String,
    pub issues: Vec<CompatibilityIssue>,
    pub warnings: Vec<CompatibilityIssue>,
    pub overall_compatible: bool,
}

#[derive(Debug, Clone)]
pub struct CompatibilityIssue {
    pub severity: Severity,
    pub component: String,
    pub message: String,
    pub resolution: String,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

// Helper function to parse semantic versions
fn parse_version(version: &str) -> Result<(u32, u32, u32), ValidationError> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return Err(ValidationError::InvalidVersion {
            version: version.to_string(),
        });
    }
    
    let major = parts[0].parse::<u32>().map_err(|_| ValidationError::InvalidVersion {
        version: version.to_string(),
    })?;
    let minor = parts[1].parse::<u32>().map_err(|_| ValidationError::InvalidVersion {
        version: version.to_string(),
    })?;
    let patch = parts[2].parse::<u32>().map_err(|_| ValidationError::InvalidVersion {
        version: version.to_string(),
    })?;
    
    Ok((major, minor, patch))
}

fn validate_api_compatibility(api_version: &str) -> ValidationResult<()> {
    const CURRENT_RUSTFORM_VERSION: &str = "0.1.0";
    
    let (api_major, api_minor, _) = parse_version(api_version)?;
    let (current_major, current_minor, _) = parse_version(CURRENT_RUSTFORM_VERSION)?;
    
    if api_major > current_major {
        return Err(ValidationError::IncompatibleApiVersion {
            requested: api_version.to_string(),
            current: CURRENT_RUSTFORM_VERSION.to_string(),
            reason: "Major version too new".to_string(),
        });
    }
    
    if api_major < current_major {
        return Err(ValidationError::IncompatibleApiVersion {
            requested: api_version.to_string(),
            current: CURRENT_RUSTFORM_VERSION.to_string(),
            reason: "Major version too old".to_string(),
        });
    }
    
    if api_minor > current_minor {
        return Err(ValidationError::IncompatibleApiVersion {
            requested: api_version.to_string(),
            current: CURRENT_RUSTFORM_VERSION.to_string(),
            reason: "Minor version too new".to_string(),
        });
    }
    
    Ok(())
}