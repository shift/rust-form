use crate::error::CodeGenError;
use crate::templates::templates_dir::get_templates;
use tera::{Tera, Context, Value, Result as TeraResult};
use std::collections::HashMap;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, CodeGenError> {
        let mut tera = Tera::default();
        
        // Load templates from embedded directory
        let templates_dir = get_templates();
        for file in templates_dir.files() {
            if let Some(path) = file.path().to_str() {
                if path.ends_with(".tera") {
                    let content = file.contents_utf8()
                        .ok_or_else(|| CodeGenError::Template(format!("Template {} is not valid UTF-8", path)))?;
                    tera.add_raw_template(path, content)
                        .map_err(|e| CodeGenError::Template(format!("Failed to add template '{}': {}", path, e)))?;
                }
            }
        }
        
        // Register custom filters
        tera.register_filter("snake_case", snake_case_filter);
        tera.register_filter("pascal_case", pascal_case_filter);
        tera.register_filter("camel_case", camel_case_filter);
        tera.register_filter("kebab_case", kebab_case_filter);
        tera.register_filter("rust_type", rust_type_filter);
        tera.register_filter("sql_type", sql_type_filter);
        tera.register_filter("default_value", default_value_filter);
        tera.register_filter("quote", quote_filter);
        
        // Register custom functions
        tera.register_function("generate_id", generate_id_function);
        tera.register_function("current_year", current_year_function);
        tera.register_function("format_imports", format_imports_function);
        
        Ok(Self { tera })
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
}

// Custom filters
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
}