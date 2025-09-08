use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_name: String,
    pub version: String,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    #[serde(default)]
    pub middleware: Vec<MiddlewareConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    #[serde(rename = "type")]
    pub db_type: String,
    pub url_env: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub models: HashMap<String, ModelConfig>,
    pub endpoints: Vec<EndpointConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    pub table_name: String,
    pub fields: HashMap<String, FieldConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub auto_increment: bool,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EndpointConfig {
    pub path: String,
    pub model: String,
    pub crud: CrudConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrudConfig {
    #[serde(default)]
    pub create: bool,
    #[serde(default)]
    pub read_all: bool,
    #[serde(default)]
    pub read_one: bool,
    #[serde(default)]
    pub update: bool,
    #[serde(default)]
    pub delete: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MiddlewareConfig {
    Logger { logger: bool },
    Cors { cors: CorsConfig },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allow_origin: String,
}