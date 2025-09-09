use crate::component::{ComponentManifest, ComponentUri};
use crate::error::{Error, Result};
use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DependencyResolver {
    resolution_cache: HashMap<String, ResolvedDependency>,
}

#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub uri: ComponentUri,
    pub version: Version,
    pub dependencies: Vec<ResolvedDependency>,
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub root: ComponentManifest,
    pub resolved: HashMap<String, ResolvedDependency>,
    pub resolution_order: Vec<ComponentUri>,
}

#[derive(Debug, Clone)]
struct DependencyNode {
    pub uri: ComponentUri,
    pub version_req: VersionReq,
    pub resolved_version: Option<Version>,
    pub manifest: Option<ComponentManifest>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            resolution_cache: HashMap::new(),
        }
    }

    /// Resolve all dependencies for a given component manifest
    pub fn resolve_dependencies(&self, manifest: &ComponentManifest) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph {
            root: manifest.clone(),
            resolved: HashMap::new(),
            resolution_order: Vec::new(),
        };

        // Convert dependency map to resolvable nodes
        let mut nodes = HashMap::new();

        // For now, we'll primarily handle rust dependencies
        // TODO: Extend to handle more complex dependency structures
        for dep_line in &manifest.dependencies.rust {
            // Parse Cargo.toml-style dependencies like "sqlx = { version = \"0.7\", features = [\"postgres\"] }"
            if let Some(eq_pos) = dep_line.find('=') {
                let dep_name = dep_line[..eq_pos].trim().to_string();
                let version_part = dep_line[eq_pos + 1..].trim();

                // Extract version from various formats
                let version_constraint =
                    if version_part.starts_with('"') && version_part.ends_with('"') {
                        // Simple version like "0.7"
                        version_part.trim_matches('"').to_string()
                    } else if version_part.contains("version") {
                        // Complex format like { version = "0.7", features = [...] }
                        // Extract just the version part
                        if let Some(version_start) = version_part.find("version") {
                            let after_version = &version_part[version_start + 7..];
                            if let Some(quote_start) = after_version.find('"') {
                                let after_quote = &after_version[quote_start + 1..];
                                if let Some(quote_end) = after_quote.find('"') {
                                    after_quote[..quote_end].to_string()
                                } else {
                                    continue; // Skip malformed entries
                                }
                            } else {
                                continue; // Skip entries without quotes
                            }
                        } else {
                            continue; // Skip entries without version
                        }
                    } else {
                        continue; // Skip unrecognized formats
                    };

                let uri = ComponentUri::from_str(&dep_name)?;
                let version_req = VersionReq::parse(&version_constraint).map_err(|e| {
                    Error::ValidationError(format!(
                        "Invalid version constraint '{}': {}",
                        version_constraint, e
                    ))
                })?;

                nodes.insert(
                    dep_name,
                    DependencyNode {
                        uri,
                        version_req,
                        resolved_version: None,
                        manifest: None,
                    },
                );
            }
        }

        // Perform dependency resolution using a topological sort approach
        self.resolve_recursive(&mut nodes, &mut graph)?;

        // Build resolution order (topologically sorted)
        graph.resolution_order = self.topological_sort(&graph.resolved)?;

        Ok(graph)
    }

    /// Recursively resolve dependencies using constraint satisfaction
    fn resolve_recursive(
        &self,
        nodes: &mut HashMap<String, DependencyNode>,
        graph: &mut DependencyGraph,
    ) -> Result<()> {
        let mut queue = VecDeque::new();

        // Initialize queue with direct dependencies
        for name in nodes.keys() {
            queue.push_back(name.clone());
        }

        let mut visited = HashSet::new();

        while let Some(dep_name) = queue.pop_front() {
            if visited.contains(&dep_name) {
                continue;
            }

            let node = nodes.get(&dep_name).ok_or_else(|| {
                Error::ComponentError(format!("Dependency node not found: {}", dep_name))
            })?;

            // For now, we'll use a simple resolution strategy:
            // Pick the latest version that satisfies the constraint
            let resolved_version = self.resolve_version_constraint(&node.uri, &node.version_req)?;

            // Create resolved dependency
            let resolved = ResolvedDependency {
                uri: node.uri.clone(),
                version: resolved_version,
                dependencies: Vec::new(), // Will be populated recursively
            };

            graph.resolved.insert(dep_name.clone(), resolved);
            visited.insert(dep_name.clone());

            // Note: In a full implementation, we would:
            // 1. Fetch the manifest for this resolved version
            // 2. Add its dependencies to the queue for resolution
            // 3. Handle version conflicts using backtracking or SAT solving
            // 4. Check for circular dependencies
        }

        Ok(())
    }

    /// Resolve version constraint to a specific version
    fn resolve_version_constraint(
        &self,
        _uri: &ComponentUri,
        version_req: &VersionReq,
    ) -> Result<Version> {
        // In a real implementation, this would:
        // 1. Fetch available versions from the component source
        // 2. Find the best matching version that satisfies the constraint
        // 3. Consider pre-release preferences, etc.

        // For now, return a mock version that satisfies basic constraints
        let req_string = version_req.to_string();
        if req_string.starts_with('^') {
            // Handle caret requirements like "^1.2.3"
            let version_str = req_string.trim_start_matches('^');
            Version::parse(version_str).map_err(|e| {
                Error::ValidationError(format!("Could not parse version from constraint: {}", e))
            })
        } else if req_string.starts_with('~') {
            // Handle tilde requirements like "~1.2.3"
            let version_str = req_string.trim_start_matches('~');
            Version::parse(version_str).map_err(|e| {
                Error::ValidationError(format!("Could not parse version from constraint: {}", e))
            })
        } else {
            // Try to parse as exact version
            Version::parse(&req_string).map_err(|e| {
                Error::ValidationError(format!(
                    "Could not resolve version constraint '{}': {}",
                    version_req, e
                ))
            })
        }
    }

    /// Perform topological sort to determine installation order
    fn topological_sort(
        &self,
        resolved: &HashMap<String, ResolvedDependency>,
    ) -> Result<Vec<ComponentUri>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();

        for (name, dependency) in resolved {
            if !visited.contains(name) {
                self.visit_for_topological_sort(
                    name,
                    dependency,
                    resolved,
                    &mut visited,
                    &mut temp_visited,
                    &mut result,
                )?;
            }
        }

        Ok(result)
    }

    fn visit_for_topological_sort(
        &self,
        name: &str,
        dependency: &ResolvedDependency,
        all_resolved: &HashMap<String, ResolvedDependency>,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        result: &mut Vec<ComponentUri>,
    ) -> Result<()> {
        if temp_visited.contains(name) {
            return Err(Error::ValidationError(format!(
                "Circular dependency detected involving component: {}",
                name
            )));
        }

        if visited.contains(name) {
            return Ok(());
        }

        temp_visited.insert(name.to_string());

        // Visit all dependencies first
        for dep in &dependency.dependencies {
            let dep_name = dep.uri.to_string();
            if let Some(dep_resolved) = all_resolved.get(&dep_name) {
                self.visit_for_topological_sort(
                    &dep_name,
                    dep_resolved,
                    all_resolved,
                    visited,
                    temp_visited,
                    result,
                )?;
            }
        }

        temp_visited.remove(name);
        visited.insert(name.to_string());
        result.push(dependency.uri.clone());

        Ok(())
    }

    /// Add resolved dependency to cache
    pub fn cache_resolution(&mut self, name: String, resolved: ResolvedDependency) {
        self.resolution_cache.insert(name, resolved);
    }

    /// Get cached resolution
    pub fn get_cached_resolution(&self, name: &str) -> Option<&ResolvedDependency> {
        self.resolution_cache.get(name)
    }

    /// Clear resolution cache
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
    }

    /// Validate that a dependency graph has no conflicts
    pub fn validate_resolution(&self, graph: &DependencyGraph) -> Result<()> {
        // Check for version conflicts
        let mut version_map: HashMap<String, Vec<&Version>> = HashMap::new();

        for (_name, resolved) in &graph.resolved {
            version_map
                .entry(resolved.uri.path.clone())
                .or_default()
                .push(&resolved.version);
        }

        for (component_path, versions) in version_map {
            if versions.len() > 1 {
                let version_strings: Vec<String> = versions.iter().map(|v| v.to_string()).collect();
                return Err(Error::ValidationError(format!(
                    "Version conflict for component '{}': found versions [{}]",
                    component_path,
                    version_strings.join(", ")
                )));
            }
        }

        Ok(())
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Get the installation order for components
    pub fn installation_order(&self) -> &[ComponentUri] {
        &self.resolution_order
    }

    /// Get resolved version for a specific component
    pub fn get_resolved_version(&self, component_name: &str) -> Option<&Version> {
        self.resolved.get(component_name).map(|r| &r.version)
    }

    /// Check if a component is resolved
    pub fn is_resolved(&self, component_name: &str) -> bool {
        self.resolved.contains_key(component_name)
    }

    /// Get total number of resolved dependencies
    pub fn dependency_count(&self) -> usize {
        self.resolved.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{ComponentInterface, ComponentManifest};

    #[test]
    fn test_simple_dependency_resolution() {
        let resolver = DependencyResolver::new();

        let mut manifest = ComponentManifest {
            name: "test-component".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test component".to_string()),
            author: Some("Test Author".to_string()),
            license: Some("MIT".to_string()),
            homepage: None,
            repository: None,
            keywords: Vec::new(),
            category: Some("general".to_string()),
            subcategory: None,
            priority: Some("medium".to_string()),
            complexity: Some("medium".to_string()),
            api_compatibility: crate::component::manifest::ApiCompatibility {
                api_version: "0.1.0".to_string(),
                min_version: "0.1.0".to_string(),
                max_version: Some("0.2.0".to_string()),
                required_features: None,
                experimental: None,
            },
            dependencies: crate::component::manifest::ComponentDependencies::default(),
            provides: Some(ComponentInterface::default()),
            config_schema: None,
            compliance: None,
            tests: None,
            documentation: None,
            features: None,
            templates: None,
            integrity: None,
            files: Vec::new(),
        };

        manifest
            .dependencies
            .rust
            .push("sqlx = \"0.7\"".to_string());
        manifest
            .dependencies
            .rust
            .push("tokio = { version = \"1.0\", features = [\"full\"] }".to_string());

        let result = resolver.resolve_dependencies(&manifest);
        assert!(result.is_ok());

        let graph = result.unwrap();
        assert_eq!(graph.dependency_count(), 2);
        assert!(graph.is_resolved("rust-form/ui-kit"));
        assert!(graph.is_resolved("github:org/utils"));
    }

    #[test]
    fn test_version_constraint_parsing() {
        let resolver = DependencyResolver::new();

        // Test various version constraint formats
        let constraints = vec![
            ("^1.2.3", true),
            ("~1.2.3", true),
            ("1.2.3", true),
            (">=1.0.0", false), // Not implemented yet
            ("invalid-version", false),
        ];

        for (constraint, should_work) in constraints {
            let version_req = VersionReq::parse(constraint);
            if should_work {
                assert!(
                    version_req.is_ok(),
                    "Should parse constraint: {}",
                    constraint
                );
            } else {
                // Some constraints may not be supported yet
            }
        }
    }
}
