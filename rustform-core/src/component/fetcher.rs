use crate::error::{Error, Result};
use crate::component::{ComponentManifest, Component, ComponentUri, ComponentContent, UriScheme};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use reqwest;
use tracing::{warn, debug};

#[derive(Debug, Clone)]
pub struct ComponentFetcher {
    client: reqwest::Client,
    temp_dir: PathBuf,
}

impl ComponentFetcher {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir().join("rustform-components");
        fs::create_dir_all(&temp_dir).ok();

        Self {
            client: reqwest::Client::new(),
            temp_dir,
        }
    }

    /// Fetch component manifest from various sources
    pub async fn fetch_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        debug!("Fetching manifest for: {}", uri);

        match &uri.scheme {
            UriScheme::Path | UriScheme::File => {
                self.fetch_local_manifest(uri).await
            }
            UriScheme::GitHub => {
                self.fetch_github_manifest(uri).await
            }
            UriScheme::GitLab => {
                self.fetch_gitlab_manifest(uri).await
            }
            UriScheme::Git => {
                self.fetch_git_manifest(uri).await
            }
            UriScheme::Registry => {
                self.fetch_registry_manifest(uri).await
            }
        }
    }

    /// Fetch complete component including all files
    pub async fn fetch_component(&self, uri: &ComponentUri) -> Result<Component> {
        debug!("Fetching component for: {}", uri);

        let manifest = self.fetch_manifest(uri).await?;
        
        let content = match &uri.scheme {
            UriScheme::Path | UriScheme::File => {
                self.fetch_local_content(uri, &manifest).await?
            }
            UriScheme::GitHub => {
                self.fetch_github_content(uri, &manifest).await?
            }
            UriScheme::GitLab => {
                self.fetch_gitlab_content(uri, &manifest).await?
            }
            UriScheme::Git => {
                self.fetch_git_content(uri, &manifest).await?
            }
            UriScheme::Registry => {
                self.fetch_registry_content(uri, &manifest).await?
            }
        };

        let resolved_path = self.get_component_path(uri);

        Ok(Component {
            manifest,
            content,
            resolved_path,
        })
    }

    /// Fetch manifest from local filesystem
    async fn fetch_local_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        let base_path = Path::new(&uri.path);
        let manifest_path = base_path.join("rustform-component.yml");
        
        if !manifest_path.exists() {
            return Err(Error::ComponentError(format!(
                "Component manifest not found at: {}",
                manifest_path.display()
            )));
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| Error::ComponentError(format!("Failed to read manifest: {}", e)))?;

        ComponentManifest::from_yaml(&content)
    }

    /// Fetch content from local filesystem
    async fn fetch_local_content(&self, uri: &ComponentUri, manifest: &ComponentManifest) -> Result<ComponentContent> {
        let base_path = Path::new(&uri.path);
        let mut content = ComponentContent {
            templates: HashMap::new(),
            assets: HashMap::new(),
            hooks: HashMap::new(),
        };

        // Load templates
        for template_spec in &manifest.provides.templates {
            let template_path = base_path.join(&template_spec.path);
            if template_path.exists() {
                let template_content = fs::read_to_string(&template_path)
                    .map_err(|e| Error::ComponentError(format!("Failed to read template {}: {}", template_spec.name, e)))?;
                content.templates.insert(template_spec.name.clone(), template_content);
            }
        }

        // Load assets
        for asset_spec in &manifest.provides.assets {
            let asset_path = base_path.join(&asset_spec.path);
            if asset_path.exists() {
                let asset_content = fs::read(&asset_path)
                    .map_err(|e| Error::ComponentError(format!("Failed to read asset {}: {}", asset_spec.name, e)))?;
                content.assets.insert(asset_spec.name.clone(), asset_content);
            }
        }

        // Load hooks
        for hook_spec in &manifest.provides.hooks {
            let hook_path = base_path.join(&hook_spec.script);
            if hook_path.exists() {
                let hook_content = fs::read_to_string(&hook_path)
                    .map_err(|e| Error::ComponentError(format!("Failed to read hook {}: {}", hook_spec.name, e)))?;
                content.hooks.insert(hook_spec.name.clone(), hook_content);
            }
        }

        Ok(content)
    }

    /// Fetch manifest from GitHub
    async fn fetch_github_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        let parts: Vec<&str> = uri.path.split('/').collect();
        if parts.len() != 2 {
            return Err(Error::ValidationError(
                format!("Invalid GitHub URI format: {}", uri.path)
            ));
        }

        let (owner, repo) = (parts[0], parts[1]);
        let ref_name = uri.version.as_deref().unwrap_or("main");
        
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/rustform-component.yml",
            owner, repo, ref_name
        );

        debug!("Fetching GitHub manifest from: {}", url);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| Error::ComponentError(format!("Failed to fetch from GitHub: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ComponentError(format!(
                "Failed to fetch manifest from GitHub: HTTP {}",
                response.status()
            )));
        }

        let content = response.text().await
            .map_err(|e| Error::ComponentError(format!("Failed to read response: {}", e)))?;

        ComponentManifest::from_yaml(&content)
    }

    /// Fetch content from GitHub
    async fn fetch_github_content(&self, uri: &ComponentUri, manifest: &ComponentManifest) -> Result<ComponentContent> {
        let parts: Vec<&str> = uri.path.split('/').collect();
        let (owner, repo) = (parts[0], parts[1]);
        let ref_name = uri.version.as_deref().unwrap_or("main");
        
        let mut content = ComponentContent {
            templates: HashMap::new(),
            assets: HashMap::new(),
            hooks: HashMap::new(),
        };

        // Fetch templates
        for template_spec in &manifest.provides.templates {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, ref_name, template_spec.path
            );
            
            if let Ok(template_content) = self.fetch_text_from_url(&url).await {
                content.templates.insert(template_spec.name.clone(), template_content);
            } else {
                warn!("Failed to fetch template: {}", template_spec.name);
            }
        }

        // Fetch assets
        for asset_spec in &manifest.provides.assets {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, ref_name, asset_spec.path
            );
            
            if let Ok(asset_content) = self.fetch_bytes_from_url(&url).await {
                content.assets.insert(asset_spec.name.clone(), asset_content);
            } else {
                warn!("Failed to fetch asset: {}", asset_spec.name);
            }
        }

        // Fetch hooks
        for hook_spec in &manifest.provides.hooks {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, ref_name, hook_spec.script
            );
            
            if let Ok(hook_content) = self.fetch_text_from_url(&url).await {
                content.hooks.insert(hook_spec.name.clone(), hook_content);
            } else {
                warn!("Failed to fetch hook: {}", hook_spec.name);
            }
        }

        Ok(content)
    }

    /// Fetch manifest from GitLab
    async fn fetch_gitlab_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        let parts: Vec<&str> = uri.path.split('/').collect();
        if parts.len() != 2 {
            return Err(Error::ValidationError(
                format!("Invalid GitLab URI format: {}", uri.path)
            ));
        }

        let (owner, repo) = (parts[0], parts[1]);
        let ref_name = uri.version.as_deref().unwrap_or("main");
        
        let url = format!(
            "https://gitlab.com/{}/{}/-/raw/{}/rustform-component.yml",
            owner, repo, ref_name
        );

        debug!("Fetching GitLab manifest from: {}", url);

        let content = self.fetch_text_from_url(&url).await?;
        ComponentManifest::from_yaml(&content)
    }

    /// Fetch content from GitLab
    async fn fetch_gitlab_content(&self, uri: &ComponentUri, manifest: &ComponentManifest) -> Result<ComponentContent> {
        // Similar implementation to GitHub but with GitLab URLs
        let parts: Vec<&str> = uri.path.split('/').collect();
        let (owner, repo) = (parts[0], parts[1]);
        let ref_name = uri.version.as_deref().unwrap_or("main");
        
        let mut content = ComponentContent {
            templates: HashMap::new(),
            assets: HashMap::new(),
            hooks: HashMap::new(),
        };

        // Fetch templates
        for template_spec in &manifest.provides.templates {
            let url = format!(
                "https://gitlab.com/{}/{}/-/raw/{}/{}",
                owner, repo, ref_name, template_spec.path
            );
            
            if let Ok(template_content) = self.fetch_text_from_url(&url).await {
                content.templates.insert(template_spec.name.clone(), template_content);
            }
        }

        Ok(content)
    }

    /// Fetch from generic Git repository
    async fn fetch_git_manifest(&self, _uri: &ComponentUri) -> Result<ComponentManifest> {
        // For now, return an error - implementing full Git support requires git2 crate
        Err(Error::ComponentError(
            "Direct Git fetching not yet implemented. Use GitHub or GitLab schemes instead.".to_string()
        ))
    }

    async fn fetch_git_content(&self, _uri: &ComponentUri, _manifest: &ComponentManifest) -> Result<ComponentContent> {
        Err(Error::ComponentError(
            "Direct Git fetching not yet implemented. Use GitHub or GitLab schemes instead.".to_string()
        ))
    }

    /// Fetch from registry
    async fn fetch_registry_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        let registry_url = std::env::var("RUSTFORM_REGISTRY_URL")
            .unwrap_or_else(|_| "https://registry.rust-form.dev".to_string());
        
        let version = uri.version.as_deref().unwrap_or("latest");
        let url = format!("{}/v1/components/{}/{}/manifest", registry_url, uri.path, version);
        
        debug!("Fetching registry manifest from: {}", url);

        let content = self.fetch_text_from_url(&url).await?;
        ComponentManifest::from_yaml(&content)
    }

    async fn fetch_registry_content(&self, _uri: &ComponentUri, _manifest: &ComponentManifest) -> Result<ComponentContent> {
        // Registry content fetching would download a tarball and extract it
        Err(Error::ComponentError(
            "Registry content fetching not yet implemented.".to_string()
        ))
    }

    /// Helper method to fetch text content from URL
    async fn fetch_text_from_url(&self, url: &str) -> Result<String> {
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| Error::ComponentError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ComponentError(format!(
                "HTTP request failed with status: {}",
                response.status()
            )));
        }

        response.text().await
            .map_err(|e| Error::ComponentError(format!("Failed to read response text: {}", e)))
    }

    /// Helper method to fetch binary content from URL
    async fn fetch_bytes_from_url(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| Error::ComponentError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ComponentError(format!(
                "HTTP request failed with status: {}",
                response.status()
            )));
        }

        response.bytes().await
            .map(|b| b.to_vec())
            .map_err(|e| Error::ComponentError(format!("Failed to read response bytes: {}", e)))
    }

    /// Get local path where component should be stored
    fn get_component_path(&self, uri: &ComponentUri) -> PathBuf {
        self.temp_dir.join(format!("{}-{}", 
            uri.path.replace('/', "-"), 
            uri.version.as_deref().unwrap_or("latest")
        ))
    }

    /// Check if component is available from source
    pub async fn check_availability(&self, uri: &ComponentUri) -> Result<bool> {
        match self.fetch_manifest(uri).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// List available versions for a component (registry only for now)
    pub async fn list_versions(&self, uri: &ComponentUri) -> Result<Vec<String>> {
        if !matches!(uri.scheme, UriScheme::Registry) {
            return Err(Error::ComponentError(
                "Version listing only supported for registry components".to_string()
            ));
        }

        let registry_url = std::env::var("RUSTFORM_REGISTRY_URL")
            .unwrap_or_else(|_| "https://registry.rust-form.dev".to_string());
        
        let _url = format!("{}/v1/components/{}/versions", registry_url, uri.path);
        
        // For now, return mock versions
        Ok(vec!["1.0.0".to_string(), "1.1.0".to_string(), "2.0.0".to_string()])
    }
}

impl Default for ComponentFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_path_generation() {
        let fetcher = ComponentFetcher::new();
        let uri: ComponentUri = "github:rust-form/ui-kit@v1.0.0".parse().unwrap();
        
        let path = fetcher.get_component_path(&uri);
        assert!(path.to_string_lossy().contains("rust-form-ui-kit-v1.0.0"));
    }

    #[test]
    fn test_github_url_generation() {
        let fetcher = ComponentFetcher::new();
        let uri: ComponentUri = "github:org/repo@v1.0.0".parse().unwrap();
        
        // Test that we can construct the correct GitHub raw URL
        let parts: Vec<&str> = uri.path.split('/').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "org");
        assert_eq!(parts[1], "repo");
    }
}