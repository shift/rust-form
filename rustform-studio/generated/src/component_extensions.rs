// Component Extensions - Custom Logic Implementation
// Demonstrates URI validation, manifest fetching, and caching

use semver::{Version, VersionReq};
use url::Url;
use std::error::Error;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio;

use crate::models::{Component, CreateComponent, UpdateComponent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub templates: Vec<String>,
    pub integrity: Option<String>,
}

#[derive(Debug)]
pub struct ComponentError {
    pub message: String,
    pub component: String,
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Component error for '{}': {}", self.component, self.message)
    }
}

impl Error for ComponentError {}

impl Component {
    /// Validates component URI format and accessibility
    pub fn validate_uri_format(&self) -> Result<(), Box<dyn Error>> {
        let uri = &self.uri;

        // Parse different URI types
        if uri.starts_with("github:") {
            self.validate_github_uri(uri)?;
        } else if uri.starts_with("gitlab:") {
            self.validate_gitlab_uri(uri)?;
        } else if uri.starts_with("path:") {
            self.validate_path_uri(uri)?;
        } else if uri.starts_with("http://") || uri.starts_with("https://") {
            self.validate_http_uri(uri)?;
        } else {
            return Err(Box::new(ComponentError {
                message: "Invalid URI format. Supported formats: github:, gitlab:, path:, http://, https://".to_string(),
                component: self.name.clone(),
            }));
        }

        Ok(())
    }

    fn validate_github_uri(&self, uri: &str) -> Result<(), Box<dyn Error>> {
        // Format: github:owner/repo@version or github:owner/repo
        let path = uri.strip_prefix("github:").ok_or("Invalid github URI")?;
        let (repo_path, _version) = if path.contains('@') {
            let parts: Vec<&str> = path.split('@').collect();
            (parts[0], Some(parts[1]))
        } else {
            (path, None)
        };

        // Validate repo path format (owner/repo)
        let parts: Vec<&str> = repo_path.split('/').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(Box::new(ComponentError {
                message: "GitHub URI must be in format 'github:owner/repo' or 'github:owner/repo@version'".to_string(),
                component: self.name.clone(),
            }));
        }

        Ok(())
    }

    fn validate_gitlab_uri(&self, uri: &str) -> Result<(), Box<dyn Error>> {
        // Similar to GitHub but for GitLab
        let path = uri.strip_prefix("gitlab:").ok_or("Invalid gitlab URI")?;
        let (repo_path, _version) = if path.contains('@') {
            let parts: Vec<&str> = path.split('@').collect();
            (parts[0], Some(parts[1]))
        } else {
            (path, None)
        };

        let parts: Vec<&str> = repo_path.split('/').collect();
        if parts.len() < 2 || parts.iter().any(|p| p.is_empty()) {
            return Err(Box::new(ComponentError {
                message: "GitLab URI must be in format 'gitlab:owner/repo' or 'gitlab:group/subgroup/repo'".to_string(),
                component: self.name.clone(),
            }));
        }

        Ok(())
    }

    fn validate_path_uri(&self, uri: &str) -> Result<(), Box<dyn Error>> {
        let path = uri.strip_prefix("path:").ok_or("Invalid path URI")?;
        
        // Basic path validation
        if path.is_empty() {
            return Err(Box::new(ComponentError {
                message: "Path URI cannot be empty".to_string(),
                component: self.name.clone(),
            }));
        }

        // Check for potentially dangerous paths
        if path.contains("..") {
            return Err(Box::new(ComponentError {
                message: "Path traversal not allowed in component paths".to_string(),
                component: self.name.clone(),
            }));
        }

        Ok(())
    }

    fn validate_http_uri(&self, uri: &str) -> Result<(), Box<dyn Error>> {
        let _url = Url::parse(uri).map_err(|e| ComponentError {
            message: format!("Invalid HTTP URI: {}", e),
            component: self.name.clone(),
        })?;

        Ok(())
    }

    /// Fetches component manifest from remote source
    pub async fn fetch_remote_manifest(&self) -> Result<ComponentManifest, Box<dyn Error>> {
        let uri = &self.uri;

        if uri.starts_with("github:") {
            self.fetch_github_manifest(uri).await
        } else if uri.starts_with("gitlab:") {
            self.fetch_gitlab_manifest(uri).await
        } else if uri.starts_with("path:") {
            self.fetch_local_manifest(uri).await
        } else if uri.starts_with("http://") || uri.starts_with("https://") {
            self.fetch_http_manifest(uri).await
        } else {
            Err(Box::new(ComponentError {
                message: "Unsupported URI type for manifest fetching".to_string(),
                component: self.name.clone(),
            }))
        }
    }

    async fn fetch_github_manifest(&self, uri: &str) -> Result<ComponentManifest, Box<dyn Error>> {
        // In a real implementation, this would:
        // 1. Parse the GitHub URI to get owner/repo/version
        // 2. Make HTTP request to GitHub API
        // 3. Download the rustform-component.yml file
        // 4. Parse and validate the manifest

        // For demonstration, return a mock manifest
        Ok(ComponentManifest {
            name: self.name.clone(),
            version: self.version.clone().unwrap_or_else(|| "latest".to_string()),
            description: self.description.clone(),
            author: self.author.clone(),
            dependencies: HashMap::new(),
            templates: vec!["button.tera".to_string(), "card.tera".to_string()],
            integrity: Some("sha256-abc123...".to_string()),
        })
    }

    async fn fetch_gitlab_manifest(&self, _uri: &str) -> Result<ComponentManifest, Box<dyn Error>> {
        // Similar to GitHub but for GitLab API
        Ok(ComponentManifest {
            name: self.name.clone(),
            version: self.version.clone().unwrap_or_else(|| "latest".to_string()),
            description: self.description.clone(),
            author: self.author.clone(),
            dependencies: HashMap::new(),
            templates: vec!["form.tera".to_string()],
            integrity: Some("sha256-def456...".to_string()),
        })
    }

    async fn fetch_local_manifest(&self, uri: &str) -> Result<ComponentManifest, Box<dyn Error>> {
        let path = uri.strip_prefix("path:").unwrap();
        
        // In a real implementation, this would read the local file
        // For now, return a mock manifest
        Ok(ComponentManifest {
            name: self.name.clone(),
            version: "local".to_string(),
            description: self.description.clone(),
            author: Some("local".to_string()),
            dependencies: HashMap::new(),
            templates: vec!["input.tera".to_string(), "button.tera".to_string()],
            integrity: None,
        })
    }

    async fn fetch_http_manifest(&self, uri: &str) -> Result<ComponentManifest, Box<dyn Error>> {
        // In a real implementation, this would make HTTP request
        Ok(ComponentManifest {
            name: self.name.clone(),
            version: "remote".to_string(),
            description: self.description.clone(),
            author: self.author.clone(),
            dependencies: HashMap::new(),
            templates: vec!["component.tera".to_string()],
            integrity: Some("sha256-remote123...".to_string()),
        })
    }

    /// Caches component data locally for faster access
    pub fn cache_component_data(&self) -> Result<(), Box<dyn Error>> {
        // In a real implementation, this would:
        // 1. Create cache directory structure
        // 2. Store manifest data
        // 3. Download and cache template files
        // 4. Update cache metadata

        tracing::info!("Caching component data for: {}", self.name);
        
        // For demonstration, we'll just log the action
        tracing::debug!(
            "Component cache: name={}, version={}, uri={}",
            self.name,
            self.version.as_ref().unwrap_or(&"latest".to_string()),
            self.uri
        );

        Ok(())
    }

    /// Checks version compatibility between components
    pub fn check_version_compatibility(&self, required_version: &str) -> Result<bool, Box<dyn Error>> {
        let current_version = self.version.as_ref().ok_or("No version specified")?;
        
        // Parse versions
        let current = Version::parse(current_version).map_err(|e| ComponentError {
            message: format!("Invalid current version '{}': {}", current_version, e),
            component: self.name.clone(),
        })?;

        let requirement = VersionReq::parse(required_version).map_err(|e| ComponentError {
            message: format!("Invalid version requirement '{}': {}", required_version, e),
            component: self.name.clone(),
        })?;

        Ok(requirement.matches(&current))
    }

    /// Hook: Validates component URI before creation
    pub async fn validate_component_uri(&self, data: &mut CreateComponent) -> Result<(), Box<dyn Error>> {
        // Create temporary component for validation
        let temp_component = Component {
            id: None,
            name: data.name.clone(),
            uri: data.uri.clone(),
            manifest_data: None,
            description: data.description.clone(),
            version: data.version.clone(),
            author: data.author.clone(),
            keywords: None,
            cached_at: None,
        };

        temp_component.validate_uri_format()?;
        
        tracing::info!("Component URI validated: {}", data.uri);
        Ok(())
    }

    /// Hook: Fetches and caches manifest after creation
    pub async fn fetch_and_cache_manifest(&self, entity: &Component) -> Result<(), Box<dyn Error>> {
        let manifest = entity.fetch_remote_manifest().await?;
        entity.cache_component_data()?;

        tracing::info!(
            "Component manifest fetched and cached: {} v{}",
            manifest.name,
            manifest.version
        );

        // In a real implementation, this would:
        // - Update the entity with manifest data
        // - Store in database
        // - Schedule background tasks for updates

        Ok(())
    }

    /// Hook: Checks for version updates before updates
    pub async fn check_version_updates(&self, _id: &str, data: &mut UpdateComponent) -> Result<(), Box<dyn Error>> {
        if let Some(ref version) = data.version {
            // Check if the new version is compatible
            if let Some(ref current_version) = self.version {
                let current = Version::parse(current_version)?;
                let new = Version::parse(version)?;

                if new < current {
                    tracing::warn!(
                        "Downgrading component {} from {} to {}",
                        self.name,
                        current_version,
                        version
                    );
                }
            }

            tracing::info!("Component version update: {} -> {}", self.name, version);
        }

        Ok(())
    }
}

/// Utility function to resolve component dependencies
pub async fn resolve_component_dependencies(
    components: &[Component],
) -> Result<Vec<Component>, Box<dyn Error>> {
    let mut resolved = Vec::new();
    let mut visited = std::collections::HashSet::new();

    for component in components {
        if !visited.contains(&component.name) {
            resolve_component_recursive(component, components, &mut resolved, &mut visited).await?;
        }
    }

    Ok(resolved)
}

async fn resolve_component_recursive(
    component: &Component,
    all_components: &[Component],
    resolved: &mut Vec<Component>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    visited.insert(component.name.clone());

    // Fetch manifest to get dependencies
    let manifest = component.fetch_remote_manifest().await?;

    // Resolve dependencies first
    for (dep_name, dep_version) in &manifest.dependencies {
        if let Some(dep_component) = all_components.iter().find(|c| c.name == *dep_name) {
            if !visited.contains(dep_name) {
                // Check version compatibility
                if !dep_component.check_version_compatibility(dep_version)? {
                    return Err(Box::new(ComponentError {
                        message: format!("Version conflict: {} requires {} {}", 
                                       component.name, dep_name, dep_version),
                        component: component.name.clone(),
                    }));
                }

                resolve_component_recursive(dep_component, all_components, resolved, visited).await?;
            }
        } else {
            return Err(Box::new(ComponentError {
                message: format!("Missing dependency: {}", dep_name),
                component: component.name.clone(),
            }));
        }
    }

    resolved.push(component.clone());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_github_uri() {
        let component = Component {
            id: Some(1),
            name: "test-component".to_string(),
            uri: "github:owner/repo@v1.0.0".to_string(),
            manifest_data: None,
            description: None,
            version: Some("1.0.0".to_string()),
            author: None,
            keywords: None,
            cached_at: None,
        };

        assert!(component.validate_uri_format().is_ok());
    }

    #[test]
    fn test_validate_invalid_uri() {
        let component = Component {
            id: Some(1),
            name: "test-component".to_string(),
            uri: "invalid-uri".to_string(),
            manifest_data: None,
            description: None,
            version: None,
            author: None,
            keywords: None,
            cached_at: None,
        };

        assert!(component.validate_uri_format().is_err());
    }

    #[test]
    fn test_version_compatibility() {
        let component = Component {
            id: Some(1),
            name: "test-component".to_string(),
            uri: "github:owner/repo".to_string(),
            manifest_data: None,
            description: None,
            version: Some("1.5.0".to_string()),
            author: None,
            keywords: None,
            cached_at: None,
        };

        assert!(component.check_version_compatibility("^1.0.0").unwrap());
        assert!(!component.check_version_compatibility("^2.0.0").unwrap());
    }
}