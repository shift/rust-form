use crate::component::{Component, ComponentManifest, ComponentUri, IntegrityVerifier};
use crate::error::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct ComponentCache {
    cache_dir: PathBuf,
    components: HashMap<String, CachedComponent>,
    integrity_verifier: IntegrityVerifier,
}

#[derive(Debug, Clone)]
struct CachedComponent {
    component: Component,
    cached_at: chrono::DateTime<chrono::Utc>,
    access_count: u64,
    last_verified: Option<chrono::DateTime<chrono::Utc>>,
}

impl ComponentCache {
    pub fn new() -> Result<Self> {
        let cache_dir = std::env::var("RUSTFORM_CACHE_DIR").unwrap_or_else(|_| {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from(".cache"))
                .join("rustform")
                .to_string_lossy()
                .to_string()
        });

        let cache_path = PathBuf::from(cache_dir);
        fs::create_dir_all(&cache_path).ok();

        let mut cache = Self {
            cache_dir: cache_path,
            components: HashMap::new(),
            integrity_verifier: IntegrityVerifier::new(),
        };

        // Load existing cache from disk
        cache.load_from_disk()?;

        Ok(cache)
    }

    /// Get component from cache
    pub fn get(&mut self, uri: &ComponentUri) -> Result<Option<Component>> {
        let key = uri.cache_key();

        if let Some(cached) = self.components.get_mut(&key) {
            // Update access statistics
            cached.access_count += 1;

            // Verify integrity periodically (every 24 hours)
            let should_verify = cached.last_verified.map_or(true, |last| {
                chrono::Utc::now().signed_duration_since(last).num_hours() > 24
            });

            if should_verify {
                match self.integrity_verifier.verify(&cached.component) {
                    Ok(()) => {
                        cached.last_verified = Some(chrono::Utc::now());
                        debug!(
                            "Integrity verification passed for cached component: {}",
                            key
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Integrity verification failed for cached component {}: {}",
                            key, e
                        );
                        // Remove corrupted component from cache
                        self.components.remove(&key);
                        self.remove_from_disk(&key)?;
                        return Ok(None);
                    }
                }
            }

            debug!("Cache hit for component: {}", key);
            Ok(Some(cached.component.clone()))
        } else {
            debug!("Cache miss for component: {}", key);
            Ok(None)
        }
    }

    /// Store component in cache
    pub fn store(&mut self, component: &Component) -> Result<()> {
        let uri = ComponentUri::from_str(&component.manifest.name)?;
        let key = uri.cache_key();

        // Verify integrity before storing
        self.integrity_verifier.verify(component)?;

        let cached_component = CachedComponent {
            component: component.clone(),
            cached_at: chrono::Utc::now(),
            access_count: 1,
            last_verified: Some(chrono::Utc::now()),
        };

        self.components.insert(key.clone(), cached_component);
        self.save_to_disk(&key, component)?;

        info!("Stored component in cache: {}", key);
        Ok(())
    }

    /// Remove component from cache
    pub fn invalidate(&mut self, uri: &ComponentUri) -> Result<()> {
        let key = uri.cache_key();
        self.components.remove(&key);
        self.remove_from_disk(&key)?;
        info!("Invalidated cached component: {}", key);
        Ok(())
    }

    /// Clear entire cache
    pub fn clear(&mut self) -> Result<()> {
        self.components.clear();
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
        }
        info!("Cleared component cache");
        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let mut total_access_count = 0;
        let mut oldest_cached = None;
        let mut newest_cached = None;

        for cached in self.components.values() {
            total_access_count += cached.access_count;

            if oldest_cached.is_none() || cached.cached_at < oldest_cached.unwrap() {
                oldest_cached = Some(cached.cached_at);
            }

            if newest_cached.is_none() || cached.cached_at > newest_cached.unwrap() {
                newest_cached = Some(cached.cached_at);
            }
        }

        CacheStats {
            component_count: self.components.len(),
            total_access_count,
            oldest_cached,
            newest_cached,
            cache_dir: self.cache_dir.clone(),
        }
    }

    /// Cleanup old or unused components
    pub fn cleanup(&mut self, max_age_days: i64, min_access_count: u64) -> Result<usize> {
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(max_age_days);
        let mut removed_count = 0;
        let mut to_remove = Vec::new();

        for (key, cached) in &self.components {
            if cached.cached_at < cutoff_date && cached.access_count < min_access_count {
                to_remove.push(key.clone());
            }
        }

        for key in to_remove {
            self.components.remove(&key);
            self.remove_from_disk(&key)?;
            removed_count += 1;
        }

        if removed_count > 0 {
            info!("Cleaned up {} old components from cache", removed_count);
        }

        Ok(removed_count)
    }

    /// Check if component exists in cache
    pub fn contains(&self, uri: &ComponentUri) -> bool {
        let key = uri.cache_key();
        self.components.contains_key(&key)
    }

    /// Get cache size in bytes (approximate)
    pub fn size_bytes(&self) -> u64 {
        let mut total_size = 0;

        for cached in self.components.values() {
            // Approximate size calculation
            total_size += cached
                .component
                .manifest
                .to_yaml()
                .unwrap_or_default()
                .len() as u64;

            for template in cached.component.content.templates.values() {
                total_size += template.len() as u64;
            }

            for asset in cached.component.content.assets.values() {
                total_size += asset.len() as u64;
            }

            for hook in cached.component.content.hooks.values() {
                total_size += hook.len() as u64;
            }
        }

        total_size
    }

    /// List all cached components
    pub fn list_components(&self) -> Vec<String> {
        self.components.keys().cloned().collect()
    }

    /// Load cache from disk
    fn load_from_disk(&mut self) -> Result<()> {
        if !self.cache_dir.exists() {
            return Ok(());
        }

        let entries = fs::read_dir(&self.cache_dir)?;
        let mut loaded_count = 0;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(component_name) = path.file_name().and_then(|n| n.to_str()) {
                    match self.load_component_from_disk(component_name) {
                        Ok(Some((key, component))) => {
                            let cached_component = CachedComponent {
                                component,
                                cached_at: chrono::Utc::now(), // Reset cache time on load
                                access_count: 0,
                                last_verified: None,
                            };
                            self.components.insert(key, cached_component);
                            loaded_count += 1;
                        }
                        Ok(None) => {
                            debug!("Skipped loading component: {}", component_name);
                        }
                        Err(e) => {
                            warn!("Failed to load cached component {}: {}", component_name, e);
                        }
                    }
                }
            }
        }

        if loaded_count > 0 {
            info!("Loaded {} components from disk cache", loaded_count);
        }

        Ok(())
    }

    /// Load a specific component from disk
    fn load_component_from_disk(
        &self,
        component_name: &str,
    ) -> Result<Option<(String, Component)>> {
        let component_dir = self.cache_dir.join(component_name);
        let manifest_path = component_dir.join("manifest.yml");

        if !manifest_path.exists() {
            return Ok(None);
        }

        let manifest_content = fs::read_to_string(manifest_path)?;
        let manifest = ComponentManifest::from_yaml(&manifest_content)?;

        // Load content files would go here...
        // For now, return empty content
        let content = crate::component::ComponentContent {
            templates: HashMap::new(),
            assets: HashMap::new(),
            hooks: HashMap::new(),
        };

        let component = Component {
            manifest,
            content,
            resolved_path: component_dir,
        };

        let uri = ComponentUri::from_str(&component.manifest.name)?;
        let key = uri.cache_key();

        Ok(Some((key, component)))
    }

    /// Save component to disk
    fn save_to_disk(&self, key: &str, component: &Component) -> Result<()> {
        let component_dir = self.cache_dir.join(key);
        fs::create_dir_all(&component_dir)?;

        // Save manifest
        let manifest_path = component_dir.join("manifest.yml");
        let manifest_yaml = component.manifest.to_yaml()?;
        fs::write(manifest_path, manifest_yaml)?;

        // Save templates
        let templates_dir = component_dir.join("templates");
        fs::create_dir_all(&templates_dir)?;
        for (name, content) in &component.content.templates {
            let template_path = templates_dir.join(format!("{}.tera", name));
            fs::write(template_path, content)?;
        }

        // Save assets
        let assets_dir = component_dir.join("assets");
        fs::create_dir_all(&assets_dir)?;
        for (name, content) in &component.content.assets {
            let asset_path = assets_dir.join(name);
            fs::write(asset_path, content)?;
        }

        // Save hooks
        let hooks_dir = component_dir.join("hooks");
        fs::create_dir_all(&hooks_dir)?;
        for (name, content) in &component.content.hooks {
            let hook_path = hooks_dir.join(name);
            fs::write(hook_path, content)?;
        }

        Ok(())
    }

    /// Remove component from disk
    fn remove_from_disk(&self, key: &str) -> Result<()> {
        let component_dir = self.cache_dir.join(key);
        if component_dir.exists() {
            fs::remove_dir_all(component_dir)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub component_count: usize,
    pub total_access_count: u64,
    pub oldest_cached: Option<chrono::DateTime<chrono::Utc>>,
    pub newest_cached: Option<chrono::DateTime<chrono::Utc>>,
    pub cache_dir: PathBuf,
}

impl Default for ComponentCache {
    fn default() -> Self {
        Self::new().expect("Failed to create ComponentCache")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::ComponentManifest;

    #[test]
    fn test_cache_key_generation() {
        let uri: ComponentUri = "github:org/repo@v1.0.0".parse().unwrap();
        let key = uri.cache_key();
        assert!(key.contains("github:org/repo"));
        assert!(key.contains("v1.0.0"));
    }

    #[test]
    fn test_cache_stats() {
        let cache = ComponentCache::new().unwrap();
        let stats = cache.stats();
        assert_eq!(stats.component_count, 0);
        assert_eq!(stats.total_access_count, 0);
    }
}
