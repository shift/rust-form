use crate::studio_handlers::{ComponentInfo, TemplateInfo};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRegistry {
    pub components: HashMap<String, ComponentInfo>,
    pub categories: Vec<String>,
    pub authors: Vec<String>,
    cache_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub keywords: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub templates: Vec<TemplateEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateEntry {
    pub name: String,
    pub description: Option<String>,
    pub target: String,
    pub framework: Option<String>,
    pub language: Option<String>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        let cache_path = std::env::temp_dir().join("rustform-studio-cache");
        fs::create_dir_all(&cache_path).ok();

        Self {
            components: HashMap::new(),
            categories: Vec::new(),
            authors: Vec::new(),
            cache_path,
        }
    }

    /// Discover components from the file system
    pub async fn discover_components(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting component discovery...");

        // Discover from our generated component library
        self.discover_from_directory("./generated_components").await?;
        self.discover_from_directory("./generated_components_100").await?;
        self.discover_from_directory("./components").await?;
        self.discover_from_directory("./examples/components").await?;

        // Build categories and authors lists
        self.build_metadata();

        info!("Component discovery completed. Found {} components", self.components.len());
        Ok(())
    }

    async fn discover_from_directory(&mut self, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(base_path);
        if !path.exists() {
            debug!("Directory {} does not exist, skipping", base_path);
            return Ok(());
        }

        info!("Discovering components in: {}", base_path);

        for entry in WalkDir::new(path)
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                
                // Look for component manifest files
                if file_path.file_name() == Some("rustform-component.yml".as_ref()) ||
                   file_path.file_name() == Some("component.yml".as_ref()) {
                    if let Ok(component) = self.load_component_from_manifest(file_path).await {
                        self.components.insert(component.name.clone(), component);
                    }
                } else if file_path.file_name() == Some("Cargo.toml".as_ref()) {
                    // Try to infer component from Cargo.toml structure
                    if let Ok(component) = self.infer_component_from_cargo(file_path).await {
                        self.components.insert(component.name.clone(), component);
                    }
                }
            }
        }

        Ok(())
    }

    async fn load_component_from_manifest(&self, manifest_path: &Path) -> Result<ComponentInfo, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(manifest_path)?;
        let manifest: ComponentManifest = serde_yaml::from_str(&content)?;

        let component_dir = manifest_path.parent().unwrap();
        let templates = self.discover_templates(component_dir)?;

        Ok(ComponentInfo {
            name: manifest.name,
            version: manifest.version,
            description: manifest.description,
            author: manifest.author,
            keywords: manifest.keywords,
            templates,
            uri: format!("path:{}", component_dir.display()),
            cached_at: Some(chrono::Utc::now().to_rfc3339()),
        })
    }

    async fn infer_component_from_cargo(&self, cargo_path: &Path) -> Result<ComponentInfo, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(cargo_path)?;
        let cargo_toml: toml::Value = toml::from_str(&content)?;

        let package = cargo_toml.get("package").ok_or("No package section in Cargo.toml")?;
        let name = package.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
        let version = package.get("version").and_then(|v| v.as_str()).unwrap_or("0.1.0");
        let description = package.get("description").and_then(|v| v.as_str());
        let authors = package.get("authors").and_then(|v| v.as_array());

        let component_dir = cargo_path.parent().unwrap();
        let templates = self.discover_templates(component_dir)?;

        // Extract category from path structure
        let category = self.extract_category_from_path(component_dir);
        let author = authors
            .and_then(|a| a.first())
            .and_then(|v| v.as_str())
            .or_else(|| Some("rustform"))
            .map(String::from);

        Ok(ComponentInfo {
            name: name.to_string(),
            version: version.to_string(),
            description: description.map(String::from),
            author,
            keywords: vec![category.clone().unwrap_or_else(|| "general".to_string())],
            templates,
            uri: format!("path:{}", component_dir.display()),
            cached_at: Some(chrono::Utc::now().to_rfc3339()),
        })
    }

    fn discover_templates(&self, component_dir: &Path) -> Result<Vec<TemplateInfo>, Box<dyn std::error::Error>> {
        let mut templates = Vec::new();

        // Look for template files in various directories
        let template_dirs = vec!["templates", "src", "ui", "frontend", "backend"];

        for template_dir in template_dirs {
            let dir_path = component_dir.join(template_dir);
            if dir_path.exists() {
                for entry in WalkDir::new(&dir_path)
                    .max_depth(3)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        let file_path = entry.path();
                        
                        if let Some(extension) = file_path.extension() {
                            let ext_str = extension.to_string_lossy();
                            
                            // Identify template files
                            if ext_str == "tera" || 
                               ext_str == "tsx" || 
                               ext_str == "ts" || 
                               ext_str == "rs" ||
                               ext_str == "vue" ||
                               ext_str == "svelte" {
                                
                                let template_name = file_path
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();

                                let target = self.determine_template_target(&ext_str, file_path);
                                
                                templates.push(TemplateInfo {
                                    name: template_name,
                                    description: self.extract_template_description(file_path),
                                    target,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(templates)
    }

    fn determine_template_target(&self, extension: &str, file_path: &Path) -> String {
        match extension {
            "rs" => "Backend".to_string(),
            "tsx" | "ts" | "jsx" | "js" => "Frontend".to_string(),
            "vue" => "Vue".to_string(),
            "svelte" => "Svelte".to_string(),
            "tera" => {
                // Determine from file path or content
                let path_str = file_path.to_string_lossy().to_lowercase();
                if path_str.contains("frontend") || path_str.contains("ui") || path_str.contains("component") {
                    "Frontend".to_string()
                } else if path_str.contains("backend") || path_str.contains("handler") || path_str.contains("model") {
                    "Backend".to_string()
                } else {
                    "Template".to_string()
                }
            }
            _ => "Unknown".to_string(),
        }
    }

    fn extract_template_description(&self, file_path: &Path) -> Option<String> {
        // Try to read first few lines to extract description
        if let Ok(content) = fs::read_to_string(file_path) {
            let lines: Vec<&str> = content.lines().take(10).collect();
            
            for line in lines {
                let trimmed = line.trim();
                if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
                    let comment = trimmed
                        .trim_start_matches("//")
                        .trim_start_matches("/*")
                        .trim_start_matches("*")
                        .trim();
                    
                    if comment.len() > 10 && !comment.contains("TODO") && !comment.contains("FIXME") {
                        return Some(comment.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_category_from_path(&self, path: &Path) -> Option<String> {
        let path_str = path.to_string_lossy();
        
        // Extract category from path components
        if path_str.contains("/auth/") {
            Some("Authentication".to_string())
        } else if path_str.contains("/cms/") {
            Some("CMS".to_string())
        } else if path_str.contains("/ecommerce/") {
            Some("E-commerce".to_string())
        } else if path_str.contains("/dashboard") {
            Some("Dashboard".to_string())
        } else if path_str.contains("/payment") {
            Some("Payment".to_string())
        } else if path_str.contains("/ui") || path_str.contains("/components") {
            Some("UI".to_string())
        } else if path_str.contains("/backend") {
            Some("Backend".to_string())
        } else {
            None
        }
    }

    fn build_metadata(&mut self) {
        let mut categories = std::collections::HashSet::new();
        let mut authors = std::collections::HashSet::new();

        for component in self.components.values() {
            // Extract categories from keywords
            for keyword in &component.keywords {
                categories.insert(keyword.clone());
            }

            if let Some(ref author) = component.author {
                authors.insert(author.clone());
            }
        }

        self.categories = categories.into_iter().collect();
        self.categories.sort();

        self.authors = authors.into_iter().collect();
        self.authors.sort();
    }

    /// Search components by query, category, and author
    pub fn search_components(
        &self,
        query: Option<&str>,
        category: Option<&str>,
        author: Option<&str>,
    ) -> Vec<ComponentInfo> {
        let mut results: Vec<ComponentInfo> = self.components.values().cloned().collect();

        // Apply search query filter
        if let Some(search_query) = query {
            let query_lower = search_query.to_lowercase();
            results.retain(|c| {
                c.name.to_lowercase().contains(&query_lower) ||
                c.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower)) ||
                c.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            });
        }

        // Apply category filter
        if let Some(cat) = category {
            results.retain(|c| c.keywords.iter().any(|k| k.eq_ignore_ascii_case(cat)));
        }

        // Apply author filter
        if let Some(auth) = author {
            results.retain(|c| c.author.as_ref().map_or(false, |a| a.eq_ignore_ascii_case(auth)));
        }

        // Sort by relevance (name match first, then description)
        if let Some(search_query) = query {
            let query_lower = search_query.to_lowercase();
            results.sort_by(|a, b| {
                let a_name_match = a.name.to_lowercase().contains(&query_lower);
                let b_name_match = b.name.to_lowercase().contains(&query_lower);
                
                match (a_name_match, b_name_match) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
        } else {
            results.sort_by(|a, b| a.name.cmp(&b.name));
        }

        results
    }

    /// Get component by name
    pub fn get_component(&self, name: &str) -> Option<&ComponentInfo> {
        self.components.get(name)
    }

    /// Get all categories
    pub fn get_categories(&self) -> &[String] {
        &self.categories
    }

    /// Get all authors
    pub fn get_authors(&self) -> &[String] {
        &self.authors
    }

    /// Save cache to disk
    pub fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = self.cache_path.join("component_cache.json");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(cache_file, json)?;
        Ok(())
    }

    /// Load cache from disk
    pub fn load_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = self.cache_path.join("component_cache.json");
        if cache_file.exists() {
            let json = fs::read_to_string(cache_file)?;
            let cached: ComponentRegistry = serde_json::from_str(&json)?;
            self.components = cached.components;
            self.categories = cached.categories;
            self.authors = cached.authors;
        }
        Ok(())
    }

    /// Refresh the component cache
    pub async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Refreshing component registry...");
        
        // Clear existing data
        self.components.clear();
        self.categories.clear();
        self.authors.clear();

        // Rediscover components
        self.discover_components().await?;

        // Save updated cache
        self.save_cache()?;

        info!("Component registry refreshed with {} components", self.components.len());
        Ok(())
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}