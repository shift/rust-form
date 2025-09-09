pub mod cache;
pub mod fetcher;
pub mod integrity;
pub mod lockfile;
pub mod manifest;
pub mod resolver;
pub mod uri;

#[cfg(test)]
mod tests;

pub use cache::*;
pub use fetcher::*;
pub use integrity::*;
pub use lockfile::*;
pub use manifest::*;
pub use resolver::*;
pub use uri::*;

use crate::error::Result;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct ComponentSystem {
    cache: ComponentCache,
    resolver: DependencyResolver,
    fetcher: ComponentFetcher,
    integrity: IntegrityVerifier,
    /// Current rust-form API version
    rust_form_version: String,
}

impl ComponentSystem {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: ComponentCache::new()?,
            resolver: DependencyResolver::new(),
            fetcher: ComponentFetcher::new(),
            integrity: IntegrityVerifier::new(),
            rust_form_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    pub fn with_version(version: String) -> Result<Self> {
        Ok(Self {
            cache: ComponentCache::new()?,
            resolver: DependencyResolver::new(),
            fetcher: ComponentFetcher::new(),
            integrity: IntegrityVerifier::new(),
            rust_form_version: version,
        })
    }

    pub async fn install_component(&mut self, uri: &ComponentUri) -> Result<Component> {
        let manifest = self.fetcher.fetch_manifest(uri).await?;

        // Check API compatibility
        self.check_compatibility(&manifest)?;

        let _resolved = self.resolver.resolve_dependencies(&manifest)?;
        let component = self.fetcher.fetch_component(uri).await?;
        self.integrity.verify(&component)?;
        self.cache.store(&component)?;
        Ok(component)
    }

    pub fn get_component(&mut self, uri: &ComponentUri) -> Result<Option<Component>> {
        self.cache.get(uri)
    }

    pub async fn fetch_manifest(&self, uri: &ComponentUri) -> Result<ComponentManifest> {
        let manifest = self.fetcher.fetch_manifest(uri).await?;

        // Check API compatibility when fetching manifest
        self.check_compatibility(&manifest)?;

        Ok(manifest)
    }

    pub async fn check_availability(&self, uri: &ComponentUri) -> Result<bool> {
        self.fetcher.check_availability(uri).await
    }

    /// Check if a component is compatible with the current rust-form version
    pub fn check_compatibility(&self, manifest: &ComponentManifest) -> Result<()> {
        let status = manifest.compatibility_status(&self.rust_form_version)?;

        match status {
            crate::component::CompatibilityStatus::Compatible { .. } => {
                info!(
                    "Component '{}' is compatible: {}",
                    manifest.name,
                    status.message()
                );
                Ok(())
            }
            crate::component::CompatibilityStatus::CompatibleExperimental { .. } => {
                warn!(
                    "Component '{}' uses experimental APIs: {}",
                    manifest.name,
                    status.message()
                );
                Ok(())
            }
            crate::component::CompatibilityStatus::TooOld { .. } => {
                Err(crate::error::Error::ComponentError(format!(
                    "Component '{}' is incompatible: {}",
                    manifest.name,
                    status.message()
                )))
            }
            crate::component::CompatibilityStatus::TooNew { .. } => {
                warn!(
                    "Component '{}' may not be fully compatible: {}",
                    manifest.name,
                    status.message()
                );
                Ok(())
            }
        }
    }

    /// Get the current rust-form version this system is using
    pub fn rust_form_version(&self) -> &str {
        &self.rust_form_version
    }

    /// Get a list of components with their compatibility status
    pub async fn list_compatible_components(
        &self,
        uris: &[ComponentUri],
    ) -> Result<Vec<ComponentCompatibilityInfo>> {
        let mut results = Vec::new();

        for uri in uris {
            match self.fetcher.fetch_manifest(uri).await {
                Ok(manifest) => {
                    let status = manifest.compatibility_status(&self.rust_form_version)?;
                    results.push(ComponentCompatibilityInfo {
                        uri: uri.clone(),
                        name: manifest.name.clone(),
                        version: manifest.version.clone(),
                        status,
                    });
                }
                Err(e) => {
                    results.push(ComponentCompatibilityInfo {
                        uri: uri.clone(),
                        name: "Unknown".to_string(),
                        version: "Unknown".to_string(),
                        status: crate::component::CompatibilityStatus::TooOld {
                            current: self.rust_form_version.clone(),
                            required_min: format!("Error: {}", e),
                        },
                    });
                }
            }
        }

        Ok(results)
    }
}

/// Information about a component's compatibility status
#[derive(Debug, Clone)]
pub struct ComponentCompatibilityInfo {
    pub uri: ComponentUri,
    pub name: String,
    pub version: String,
    pub status: crate::component::CompatibilityStatus,
}

impl Default for ComponentSystem {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ComponentSystem")
    }
}
