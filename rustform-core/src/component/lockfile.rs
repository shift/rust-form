use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use crate::error::Result;
use crate::component::{ComponentUri, DependencyGraph};
use semver::Version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLockfile {
    /// Lockfile format version
    pub version: u32,
    
    /// When this lockfile was generated
    pub generated: String,
    
    /// System info when generated  
    pub generator: GeneratorInfo,
    
    /// Resolved dependencies with exact versions
    pub dependencies: HashMap<String, LockedComponent>,
    
    /// Dependency resolution tree
    pub resolution_tree: ResolutionTree,
    
    /// Metadata for reproducible builds
    pub metadata: LockfileMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedComponent {
    /// Original URI as specified
    pub uri: String,
    
    /// Resolved version
    pub version: String,
    
    /// Resolved URL for fetching
    pub resolved: String,
    
    /// Integrity hash (SRI format)
    pub integrity: String,
    
    /// Direct dependencies of this component
    pub dependencies: HashMap<String, String>,
    
    /// When this component was resolved
    pub resolved_at: String,
    
    /// Size in bytes (if known)
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorInfo {
    /// Tool that generated this lockfile
    pub name: String,
    
    /// Version of the generator
    pub version: String,
    
    /// Platform info
    pub platform: PlatformInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub rust_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionTree {
    /// Root component being built
    pub root: String,
    
    /// Tree of resolved dependencies
    pub dependencies: HashMap<String, ResolutionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionNode {
    /// Resolved version
    pub version: String,
    
    /// Why this version was chosen
    pub reason: ResolutionReason,
    
    /// Direct children in the resolution tree
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResolutionReason {
    /// Directly specified in manifest
    Direct { constraint: String },
    
    /// Resolved as transitive dependency
    Transitive { 
        via: String,
        constraint: String,
        resolved_constraint: String,
    },
    
    /// Version chosen to satisfy multiple constraints
    Conflict {
        constraints: Vec<String>,
        chosen_version: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockfileMetadata {
    /// Total number of components
    pub component_count: usize,
    
    /// Total resolved size in bytes
    pub total_size: u64,
    
    /// Resolution statistics
    pub resolution_stats: ResolutionStats,
    
    /// Platform-specific metadata
    pub platform_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionStats {
    /// Time taken to resolve dependencies (milliseconds)
    pub resolution_time_ms: u64,
    
    /// Number of HTTP requests made
    pub http_requests: u32,
    
    /// Number of cache hits
    pub cache_hits: u32,
    
    /// Number of version conflicts resolved
    pub conflicts_resolved: u32,
}

impl ComponentLockfile {
    pub fn new() -> Self {
        Self {
            version: 1,
            generated: chrono::Utc::now().to_rfc3339(),
            generator: GeneratorInfo::current(),
            dependencies: HashMap::new(),
            resolution_tree: ResolutionTree {
                root: String::new(),
                dependencies: HashMap::new(),
            },
            metadata: LockfileMetadata {
                component_count: 0,
                total_size: 0,
                resolution_stats: ResolutionStats {
                    resolution_time_ms: 0,
                    http_requests: 0,
                    cache_hits: 0,
                    conflicts_resolved: 0,
                },
                platform_metadata: HashMap::new(),
            },
        }
    }

    /// Create lockfile from dependency graph
    pub fn from_dependency_graph(graph: &DependencyGraph) -> Self {
        let mut lockfile = Self::new();
        lockfile.resolution_tree.root = graph.root.name.clone();
        
        for (name, resolved) in &graph.resolved {
            let locked_component = LockedComponent {
                uri: resolved.uri.to_string(),
                version: resolved.version.to_string(),
                resolved: resolved.uri.resolve_url().unwrap_or_default(),
                integrity: String::new(), // Would be filled during actual resolution
                dependencies: HashMap::new(), // Would be populated from manifest
                resolved_at: chrono::Utc::now().to_rfc3339(),
                size: None,
            };
            
            lockfile.dependencies.insert(name.clone(), locked_component);
            
            let resolution_node = ResolutionNode {
                version: resolved.version.to_string(),
                reason: ResolutionReason::Direct {
                    constraint: format!("^{}", resolved.version),
                },
                dependencies: resolved.dependencies.iter()
                    .map(|dep| dep.uri.to_string())
                    .collect(),
            };
            
            lockfile.resolution_tree.dependencies.insert(name.clone(), resolution_node);
        }
        
        lockfile.metadata.component_count = graph.resolved.len();
        lockfile
    }

    /// Load lockfile from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Save lockfile to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = self.to_yaml()?;
        fs::write(path, yaml)?;
        Ok(())
    }

    /// Parse lockfile from YAML
    pub fn from_yaml(content: &str) -> Result<Self> {
        serde_yaml::from_str(content).map_err(Into::into)
    }

    /// Convert lockfile to YAML
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    /// Validate lockfile integrity
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        
        // Check version compatibility
        if self.version > 1 {
            issues.push(format!("Unsupported lockfile version: {}", self.version));
        }
        
        // Validate dependency references
        for (name, locked) in &self.dependencies {
            // Check if URI is valid
            if ComponentUri::from_str(&locked.uri).is_err() {
                issues.push(format!("Invalid URI for component '{}': {}", name, locked.uri));
            }
            
            // Check if version is valid semantic version
            if Version::parse(&locked.version).is_err() {
                issues.push(format!("Invalid version for component '{}': {}", name, locked.version));
            }
            
            // Check integrity format
            if !locked.integrity.is_empty() && !locked.integrity.contains('-') {
                issues.push(format!("Invalid integrity format for component '{}': {}", name, locked.integrity));
            }
        }
        
        // Check resolution tree consistency
        for (name, node) in &self.resolution_tree.dependencies {
            if !self.dependencies.contains_key(name) {
                issues.push(format!("Resolution tree references missing component: {}", name));
            }
            
            for dep_name in &node.dependencies {
                if !self.dependencies.contains_key(dep_name) {
                    issues.push(format!("Component '{}' references missing dependency: {}", name, dep_name));
                }
            }
        }
        
        Ok(issues)
    }

    /// Check if lockfile is up to date with manifest dependencies
    pub fn is_up_to_date(&self, manifest_dependencies: &HashMap<String, String>) -> bool {
        // Check if all manifest dependencies are in lockfile
        for (name, constraint) in manifest_dependencies {
            if let Some(locked) = self.dependencies.get(name) {
                // Verify that locked version satisfies constraint
                if let (Ok(version), Ok(req)) = (Version::parse(&locked.version), semver::VersionReq::parse(constraint)) {
                    if !req.matches(&version) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Check if lockfile has extra dependencies not in manifest
        for name in self.dependencies.keys() {
            if !manifest_dependencies.contains_key(name) {
                // This might be a transitive dependency, which is okay
                continue;
            }
        }
        
        true
    }

    /// Get installation order based on resolution tree
    pub fn installation_order(&self) -> Vec<String> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        // Start from root and do depth-first traversal
        if let Some(root_node) = self.resolution_tree.dependencies.get(&self.resolution_tree.root) {
            self.visit_for_installation_order(&self.resolution_tree.root, root_node, &mut visited, &mut result);
        }
        
        result
    }
    
    fn visit_for_installation_order(
        &self,
        name: &str,
        node: &ResolutionNode,
        visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) {
        if visited.contains(name) {
            return;
        }
        
        visited.insert(name.to_string());
        
        // Visit dependencies first
        for dep_name in &node.dependencies {
            if let Some(dep_node) = self.resolution_tree.dependencies.get(dep_name) {
                self.visit_for_installation_order(dep_name, dep_node, visited, result);
            }
        }
        
        result.push(name.to_string());
    }

    /// Update metadata with new statistics
    pub fn update_metadata(&mut self, stats: ResolutionStats) {
        self.metadata.resolution_stats = stats;
        self.metadata.component_count = self.dependencies.len();
        self.metadata.total_size = self.dependencies.values()
            .filter_map(|locked| locked.size)
            .sum();
    }

    /// Add platform-specific metadata
    pub fn add_platform_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.platform_metadata.insert(key, value);
    }

    /// Merge another lockfile into this one (for workspace scenarios)
    pub fn merge(&mut self, other: &ComponentLockfile) -> Result<()> {
        for (name, locked) in &other.dependencies {
            if let Some(existing) = self.dependencies.get(name) {
                // Check for version conflicts
                if existing.version != locked.version {
                    return Err(crate::error::Error::ValidationError(format!(
                        "Version conflict for component '{}': {} vs {}",
                        name, existing.version, locked.version
                    )));
                }
            } else {
                self.dependencies.insert(name.clone(), locked.clone());
            }
        }
        
        // Merge resolution trees
        for (name, node) in &other.resolution_tree.dependencies {
            self.resolution_tree.dependencies.insert(name.clone(), node.clone());
        }
        
        self.metadata.component_count = self.dependencies.len();
        Ok(())
    }
}

impl GeneratorInfo {
    pub fn current() -> Self {
        Self {
            name: "rustform".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            platform: PlatformInfo::current(),
        }
    }
}

impl PlatformInfo {
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            rust_version: option_env!("RUSTC_VERSION").map(|v| v.to_string()),
        }
    }
}

impl Default for ComponentLockfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lockfile_creation() {
        let lockfile = ComponentLockfile::new();
        assert_eq!(lockfile.version, 1);
        assert!(!lockfile.generated.is_empty());
        assert_eq!(lockfile.dependencies.len(), 0);
    }

    #[test]
    fn test_lockfile_serialization() {
        let lockfile = ComponentLockfile::new();
        let yaml = lockfile.to_yaml().unwrap();
        let parsed = ComponentLockfile::from_yaml(&yaml).unwrap();
        
        assert_eq!(lockfile.version, parsed.version);
        assert_eq!(lockfile.generator.name, parsed.generator.name);
    }

    #[test]
    fn test_lockfile_validation() {
        let mut lockfile = ComponentLockfile::new();
        
        // Add invalid component
        let invalid_locked = LockedComponent {
            uri: "invalid-uri-format".to_string(),
            version: "not-a-version".to_string(),
            resolved: "http://example.com".to_string(),
            integrity: "invalid-integrity".to_string(),
            dependencies: HashMap::new(),
            resolved_at: chrono::Utc::now().to_rfc3339(),
            size: None,
        };
        
        lockfile.dependencies.insert("invalid".to_string(), invalid_locked);
        
        let issues = lockfile.validate().unwrap();
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_installation_order() {
        let mut lockfile = ComponentLockfile::new();
        lockfile.resolution_tree.root = "app".to_string();
        
        // Create simple dependency chain: app -> ui-kit -> icons
        let app_node = ResolutionNode {
            version: "1.0.0".to_string(),
            reason: ResolutionReason::Direct { constraint: "^1.0.0".to_string() },
            dependencies: vec!["ui-kit".to_string()],
        };
        
        let ui_kit_node = ResolutionNode {
            version: "2.0.0".to_string(),
            reason: ResolutionReason::Transitive {
                via: "app".to_string(),
                constraint: "^2.0.0".to_string(),
                resolved_constraint: "^2.0.0".to_string(),
            },
            dependencies: vec!["icons".to_string()],
        };
        
        let icons_node = ResolutionNode {
            version: "1.5.0".to_string(),
            reason: ResolutionReason::Transitive {
                via: "ui-kit".to_string(),
                constraint: "^1.0.0".to_string(),
                resolved_constraint: "^1.0.0".to_string(),
            },
            dependencies: vec![],
        };
        
        lockfile.resolution_tree.dependencies.insert("app".to_string(), app_node);
        lockfile.resolution_tree.dependencies.insert("ui-kit".to_string(), ui_kit_node);
        lockfile.resolution_tree.dependencies.insert("icons".to_string(), icons_node);
        
        let order = lockfile.installation_order();
        
        // icons should come before ui-kit, ui-kit before app
        let icons_pos = order.iter().position(|x| x == "icons").unwrap();
        let ui_kit_pos = order.iter().position(|x| x == "ui-kit").unwrap();
        let app_pos = order.iter().position(|x| x == "app").unwrap();
        
        assert!(icons_pos < ui_kit_pos);
        assert!(ui_kit_pos < app_pos);
    }
}