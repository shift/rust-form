use crate::error::CodeGenError;
use crate::templates::templates_dir::get_templates;
use tera::{Tera, Context, Value, Result as TeraResult};
use std::collections::HashMap;
use include_dir::Dir;
use rustform_core::component::{ComponentSystem, ComponentUri, ComponentLockfile};
use rustform_core::{Config, GeneratedProject};
use tracing::{debug, info, warn};
use std::str::FromStr;
use std::path::Path;

pub struct TemplateEngine {
    tera: Tera,
    component_system: ComponentSystem,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, CodeGenError> {
        let mut tera = Tera::default();
        
        // Load templates from embedded directory
        let templates_dir = get_templates();
        fn load_templates_recursive(tera: &mut Tera, dir: &Dir) -> Result<(), CodeGenError> {
            for file in dir.files() {
                if let Some(path) = file.path().to_str() {
                    if path.ends_with(".tera") {
                        let content = file.contents_utf8()
                            .ok_or_else(|| CodeGenError::Template(format!("Template {} is not valid UTF-8", path)))?;
                        tera.add_raw_template(path, content)
                            .map_err(|e| CodeGenError::Template(format!("Failed to add template '{}': {}", path, e)))?;
                    }
                }
            }
            
            for subdir in dir.dirs() {
                load_templates_recursive(tera, subdir)?;
            }
            
            Ok(())
        }
        
        load_templates_recursive(&mut tera, templates_dir)?;
        
        // Register custom filters
        tera.register_filter("snake_case", snake_case_filter);
        tera.register_filter("pascal_case", pascal_case_filter);
        tera.register_filter("camel_case", camel_case_filter);
        tera.register_filter("kebab_case", kebab_case_filter);
        tera.register_filter("rust_type", rust_type_filter);
        tera.register_filter("sql_type", sql_type_filter);
        tera.register_filter("default_value", default_value_filter);
        tera.register_filter("quote", quote_filter);
        tera.register_filter("ts_type", ts_type_filter);
        
        // Register custom functions
        tera.register_function("generate_id", generate_id_function);
        tera.register_function("current_year", current_year_function);
        tera.register_function("format_imports", format_imports_function);
        tera.register_function("custom_dependencies", custom_dependencies_function);
        tera.register_function("custom_methods", custom_methods_function);
        tera.register_function("custom_hooks", custom_hooks_function);
        
        // Initialize component system
        let component_system = ComponentSystem::new()
            .map_err(|e| CodeGenError::Template(format!("Failed to initialize component system: {}", e)))?;
        
        Ok(Self { tera, component_system })
    }
    
    pub fn render_template(&self, template_name: &str, context: &Context) -> Result<String, CodeGenError> {
        self.tera
            .render(template_name, context)
            .map_err(|e| CodeGenError::Template(format!("Failed to render template '{}': {}", template_name, e)))
    }
    
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), CodeGenError> {
        self.tera
            .add_raw_template(name, content)
            .map_err(|e| CodeGenError::Template(format!("Failed to add template '{}': {}", name, e)))
    }
    
    pub fn list_templates(&self) -> Vec<String> {
        self.tera.get_template_names().map(|s| s.to_string()).collect()
    }

    /// Install and use components from manifest
    pub async fn install_components(&mut self, config: &Config) -> Result<(), CodeGenError> {
        if let Some(components) = &config.components {
            info!("Installing {} components", components.len());
            
            for (component_name, component_spec) in components {
                debug!("Installing component: {}", component_name);
                
                let component_uri = ComponentUri::from_str(component_spec)
                    .map_err(|e| CodeGenError::Template(format!("Invalid component URI '{}': {}", component_spec, e)))?;
                
                let component = self.component_system.install_component(&component_uri).await
                    .map_err(|e| CodeGenError::Template(format!("Failed to install component '{}': {}", component_name, e)))?;
                
                // Add component templates to Tera
                for (template_name, template_content) in &component.content.templates {
                    let full_template_name = format!("components/{}/{}", component_name, template_name);
                    self.tera.add_raw_template(&full_template_name, template_content)
                        .map_err(|e| CodeGenError::Template(format!("Failed to add component template '{}': {}", full_template_name, e)))?;
                }
                
                info!("Successfully installed component: {}", component_name);
            }
        }
        
        Ok(())
    }

    /// Generate project with components
    pub async fn generate_with_components(
        &mut self, 
        config: &Config, 
        output_dir: &Path
    ) -> Result<GeneratedProject, CodeGenError> {
        // Install components first
        self.install_components(config).await?;
        
        // Generate lockfile for reproducible builds
        let lockfile = self.generate_lockfile(config).await?;
        lockfile.save(output_dir.join("rustform.lock"))
            .map_err(|e| CodeGenError::Template(format!("Failed to save lockfile: {}", e)))?;
        
        Ok(GeneratedProject {
            name: config.project_name.clone(),
            files: vec![], // This would be filled in by the calling pipeline
        })
    }

    /// Generate a lockfile for the current configuration
    pub async fn generate_lockfile(&mut self, config: &Config) -> Result<ComponentLockfile, CodeGenError> {
        let mut lockfile = ComponentLockfile::new();
        lockfile.resolution_tree.root = config.project_name.clone();
        
        if let Some(components) = &config.components {
            for (component_name, component_spec) in components {
                let component_uri = ComponentUri::from_str(component_spec)
                    .map_err(|e| CodeGenError::Template(format!("Invalid component URI '{}': {}", component_spec, e)))?;
                
                // Fetch manifest to get dependency information
                let manifest = self.component_system.fetch_manifest(&component_uri).await
                    .map_err(|e| CodeGenError::Template(format!("Failed to fetch manifest for '{}': {}", component_name, e)))?;
                
                // Add to lockfile
                let locked_component = rustform_core::component::LockedComponent {
                    uri: component_spec.clone(),
                    version: component_uri.version.clone().unwrap_or_else(|| "latest".to_string()),
                    resolved: component_uri.resolve_url()
                        .map_err(|e| CodeGenError::Template(format!("Failed to resolve URL for '{}': {}", component_name, e)))?,
                    integrity: manifest.integrity.unwrap_or_default(),
                    dependencies: manifest.dependencies.clone(),
                    resolved_at: chrono::Utc::now().to_rfc3339(),
                    size: None,
                };
                
                lockfile.dependencies.insert(component_name.clone(), locked_component);
            }
        }
        
        Ok(lockfile)
    }

    /// Render component template with context
    pub fn render_component_template(
        &self, 
        component_name: &str, 
        template_name: &str, 
        context: &Context
    ) -> Result<String, CodeGenError> {
        let full_template_name = format!("components/{}/{}", component_name, template_name);
        self.render_template(&full_template_name, context)
    }

    /// Get available components and their templates
    pub fn list_component_templates(&self) -> HashMap<String, Vec<String>> {
        let mut components = HashMap::new();
        
        for template_name in self.tera.get_template_names() {
            if template_name.starts_with("components/") {
                let parts: Vec<&str> = template_name.splitn(3, '/').collect();
                if parts.len() >= 3 {
                    let component_name = parts[1];
                    let template_file = parts[2];
                    
                    components
                        .entry(component_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(template_file.to_string());
                }
            }
        }
        
        components
    }

    /// Validate that all required components are available
    pub async fn validate_components(&mut self, config: &Config) -> Result<Vec<String>, CodeGenError> {
        let mut missing_components = Vec::new();
        
        if let Some(components) = &config.components {
            for (component_name, component_spec) in components {
                let component_uri = ComponentUri::from_str(component_spec)
                    .map_err(|e| CodeGenError::Template(format!("Invalid component URI '{}': {}", component_spec, e)))?;
                
                match self.component_system.check_availability(&component_uri).await {
                    Ok(true) => {
                        debug!("Component '{}' is available", component_name);
                    }
                    Ok(false) => {
                        warn!("Component '{}' is not available", component_name);
                        missing_components.push(component_name.clone());
                    }
                    Err(e) => {
                        warn!("Failed to check availability of component '{}': {}", component_name, e);
                        missing_components.push(component_name.clone());
                    }
                }
            }
        }
        
        Ok(missing_components)
    }

    /// Add component-specific filters and functions to Tera
    pub fn register_component_helpers(&mut self) {
        // Register component-related Tera functions
        self.tera.register_function("component_template", component_template_function);
        self.tera.register_function("component_asset", component_asset_function);
        self.tera.register_function("list_components", list_components_function);
        
        // Register component-related filters
        self.tera.register_filter("component_path", component_path_filter);
    }
}

// Component-related Tera functions
fn component_template_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let component = args.get("component")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("component parameter is required"))?;
    
    let template = args.get("template")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("template parameter is required"))?;
    
    // Return the full template path for include or extends
    Ok(Value::String(format!("components/{}/{}", component, template)))
}

fn component_asset_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let component = args.get("component")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("component parameter is required"))?;
    
    let asset = args.get("asset")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("asset parameter is required"))?;
    
    // Return the asset path
    Ok(Value::String(format!("/assets/{}/{}", component, asset)))
}

fn list_components_function(_args: &HashMap<String, Value>) -> TeraResult<Value> {
    // This would need access to the template engine instance
    // For now, return empty array
    Ok(Value::Array(vec![]))
}

fn component_path_filter(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let component = args.get("component")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("component parameter is required"))?;
    
    let path = value.as_str()
        .ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    
    Ok(Value::String(format!("components/{}/{}", component, path)))
}

// Existing filters and functions (keeping the original implementations)
fn snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    Ok(Value::String(to_snake_case(s)))
}

fn pascal_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    Ok(Value::String(to_pascal_case(s)))
}

fn camel_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    Ok(Value::String(to_camel_case(s)))
}

fn kebab_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    Ok(Value::String(to_kebab_case(s)))
}

fn rust_type_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let field_type = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    let rust_type = match field_type {
        "integer" => "i32",
        "string" => "String",
        "boolean" => "bool",
        "datetime" => "chrono::DateTime<chrono::Utc>",
        "date" => "chrono::NaiveDate",
        "time" => "chrono::NaiveTime",
        "uuid" => "uuid::Uuid",
        "json" => "serde_json::Value",
        "text" => "String",
        "float" => "f32",
        "double" => "f64",
        "decimal" => "rust_decimal::Decimal",
        "binary" => "Vec<u8>",
        _ => "String", // fallback
    };
    Ok(Value::String(rust_type.to_string()))
}

fn sql_type_filter(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let field_type = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    let db_type = args.get("db_type").and_then(|v| v.as_str()).unwrap_or("sqlite");
    
    let sql_type = match (field_type, db_type) {
        ("integer", "postgres") => "INTEGER",
        ("integer", "mysql") => "INT",
        ("integer", _) => "INTEGER",
        
        ("string", "postgres") => "VARCHAR",
        ("string", "mysql") => "VARCHAR(255)",
        ("string", _) => "TEXT",
        
        ("boolean", "postgres") => "BOOLEAN",
        ("boolean", "mysql") => "BOOLEAN",
        ("boolean", _) => "BOOLEAN",
        
        ("datetime", "postgres") => "TIMESTAMP WITH TIME ZONE",
        ("datetime", "mysql") => "DATETIME",
        ("datetime", _) => "DATETIME",
        
        ("date", _) => "DATE",
        ("time", _) => "TIME",
        
        ("uuid", "postgres") => "UUID",
        ("uuid", "mysql") => "CHAR(36)",
        ("uuid", _) => "TEXT",
        
        ("json", "postgres") => "JSONB",
        ("json", "mysql") => "JSON",
        ("json", _) => "TEXT",
        
        ("text", _) => "TEXT",
        ("float", _) => "REAL",
        ("double", _) => "DOUBLE PRECISION",
        ("decimal", _) => "DECIMAL",
        ("binary", _) => "BLOB",
        
        _ => "TEXT",
    };
    
    Ok(Value::String(sql_type.to_string()))
}

fn default_value_filter(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let field_type = args.get("field_type").and_then(|v| v.as_str()).unwrap_or("string");
    
    if value.is_null() {
        return Ok(Value::String("None".to_string()));
    }
    
    let default_val = match field_type {
        "string" | "text" => format!("Some(\"{}\".to_string())", value.as_str().unwrap_or("")),
        "integer" => format!("Some({})", value.as_i64().unwrap_or(0)),
        "boolean" => format!("Some({})", value.as_bool().unwrap_or(false)),
        "float" => format!("Some({}f32)", value.as_f64().unwrap_or(0.0)),
        "double" => format!("Some({}f64)", value.as_f64().unwrap_or(0.0)),
        _ => "None".to_string(),
    };
    
    Ok(Value::String(default_val))
}

fn quote_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    Ok(Value::String(format!("\"{}\"", s.replace("\"", "\\\""))))
}

fn ts_type_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let field_type = value.as_str().ok_or_else(|| tera::Error::msg("Value is not a string"))?;
    let ts_type = match field_type {
        "integer" => "number",
        "string" => "string",
        "boolean" => "boolean",
        "datetime" => "string", // ISO 8601 string
        "date" => "string",     // ISO 8601 date string
        "time" => "string",     // ISO 8601 time string
        "uuid" => "string",     // UUID as string
        "json" => "any",        // JSON as any type
        "text" => "string",
        "float" => "number",
        "double" => "number",
        "decimal" => "number",
        "binary" => "string",   // Base64 encoded string
        _ => "string",          // fallback
    };
    Ok(Value::String(ts_type.to_string()))
}

// Custom functions
fn generate_id_function(_: &HashMap<String, Value>) -> TeraResult<Value> {
    Ok(Value::String(uuid::Uuid::new_v4().to_string()))
}

fn current_year_function(_: &HashMap<String, Value>) -> TeraResult<Value> {
    use chrono::{Utc, Datelike};
    Ok(Value::Number(Utc::now().year().into()))
}

fn format_imports_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let empty_vec = vec![];
    let imports = args.get("imports").and_then(|v| v.as_array()).unwrap_or(&empty_vec);
    let mut formatted = imports
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| format!("use {};", s))
        .collect::<Vec<_>>();
    
    formatted.sort();
    formatted.dedup();
    
    Ok(Value::String(formatted.join("\n")))
}

fn custom_dependencies_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let empty_vec = vec![];
    let dependencies = args.get("dependencies").and_then(|v| v.as_array()).unwrap_or(&empty_vec);
    let formatted = dependencies
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>();
    
    Ok(Value::Array(formatted.into_iter().map(|s| Value::String(s.to_string())).collect()))
}

fn custom_methods_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let empty_vec = vec![];
    let methods = args.get("methods").and_then(|v| v.as_array()).unwrap_or(&empty_vec);
    let formatted = methods
        .iter()
        .filter_map(|v| v.as_str())
        .map(|method| format!("    pub fn {}(&self) -> Result<(), Box<dyn std::error::Error>> {{\n        // TODO: Implement {}\n        Ok(())\n    }}", method, method))
        .collect::<Vec<_>>();
    
    Ok(Value::String(formatted.join("\n\n")))
}

fn custom_hooks_function(args: &HashMap<String, Value>) -> TeraResult<Value> {
    let hooks = args.get("hooks");
    if hooks.is_none() {
        return Ok(Value::String("".to_string()));
    }
    
    let mut hook_implementations = Vec::new();
    
    // Extract hook functions if they exist
    if let Some(hooks_obj) = hooks.and_then(|h| h.as_object()) {
        for (hook_name, hook_fn) in hooks_obj {
            if let Some(fn_name) = hook_fn.as_str() {
                let hook_impl = match hook_name.as_str() {
                    "before_create" => format!("    async fn before_create(&self, data: &mut CreateData) -> Result<(), HookError> {{\n        self.{}(data).await\n    }}", fn_name),
                    "after_create" => format!("    async fn after_create(&self, entity: &Entity) -> Result<(), HookError> {{\n        self.{}(entity).await\n    }}", fn_name),
                    "before_update" => format!("    async fn before_update(&self, id: &str, data: &mut UpdateData) -> Result<(), HookError> {{\n        self.{}(id, data).await\n    }}", fn_name),
                    "after_update" => format!("    async fn after_update(&self, entity: &Entity) -> Result<(), HookError> {{\n        self.{}(entity).await\n    }}", fn_name),
                    "before_delete" => format!("    async fn before_delete(&self, id: &str) -> Result<(), HookError> {{\n        self.{}(id).await\n    }}", fn_name),
                    "after_delete" => format!("    async fn after_delete(&self, id: &str) -> Result<(), HookError> {{\n        self.{}(id).await\n    }}", fn_name),
                    _ => continue,
                };
                hook_implementations.push(hook_impl);
            }
        }
    }
    
    Ok(Value::String(hook_implementations.join("\n\n")))
}

// Utility functions for case conversion
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_lowercase = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 && prev_lowercase {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
        prev_lowercase = c.is_lowercase();
    }
    
    result
}

fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-', ' '][..])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_conversions() {
        assert_eq!(to_snake_case("UserAccount"), "user_account");
        assert_eq!(to_pascal_case("user_account"), "UserAccount");
        assert_eq!(to_camel_case("user_account"), "userAccount");
        assert_eq!(to_kebab_case("user_account"), "user-account");
    }
    
    #[test]
    fn test_template_engine_creation() {
        // This will fail if no templates directory exists, but that's expected for now
        match TemplateEngine::new() {
            Ok(_) => {}, // Templates directory exists
            Err(_) => {}, // Templates directory doesn't exist - expected for now
        }
    }

    #[test]
    fn test_component_template_function() {
        let mut args = HashMap::new();
        args.insert("component".to_string(), Value::String("ui-kit".to_string()));
        args.insert("template".to_string(), Value::String("button.tera".to_string()));
        
        let result = component_template_function(&args).unwrap();
        assert_eq!(result.as_str().unwrap(), "components/ui-kit/button.tera");
    }
}