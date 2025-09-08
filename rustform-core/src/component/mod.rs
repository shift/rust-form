pub mod manifest;
pub mod uri;
pub mod cache;
pub mod resolver;
pub mod fetcher;
pub mod integrity;
pub mod lockfile;

pub use manifest::*;
pub use uri::*;
pub use cache::*;
pub use resolver::*;
pub use fetcher::*;
pub use integrity::*;
pub use lockfile::*;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ComponentSystem {
    cache: ComponentCache,
    resolver: DependencyResolver,
    fetcher: ComponentFetcher,
    integrity: IntegrityVerifier,
}

impl ComponentSystem {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: ComponentCache::new()?,
            resolver: DependencyResolver::new(),
            fetcher: ComponentFetcher::new(),
            integrity: IntegrityVerifier::new(),
        })
    }

    pub async fn install_component(&mut self, uri: &ComponentUri) -> Result<Component> {
        let manifest = self.fetcher.fetch_manifest(uri).await?;
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
        self.fetcher.fetch_manifest(uri).await
    }

    pub async fn check_availability(&self, uri: &ComponentUri) -> Result<bool> {
        self.fetcher.check_availability(uri).await
    }
}

impl Default for ComponentSystem {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ComponentSystem")
    }
}