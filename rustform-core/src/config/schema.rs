use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub project_name: String,
    pub version: String,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    #[serde(default)]
    pub middleware: Vec<MiddlewareConfig>,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    #[serde(rename = "type")]
    pub db_type: DatabaseType,
    pub url_env: String,
    #[serde(default)]
    pub pool_size: Option<u32>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    Mysql,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiConfig {
    pub models: IndexMap<String, ModelConfig>,
    pub endpoints: Vec<EndpointConfig>,
    #[serde(default)]
    pub prefix: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelConfig {
    pub table_name: String,
    pub fields: IndexMap<String, FieldConfig>,
    #[serde(default)]
    pub relationships: IndexMap<String, RelationshipConfig>,
    #[serde(default)]
    pub indexes: Vec<IndexConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub auto_increment: bool,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub unique: bool,
    #[serde(default)]
    pub nullable: bool,
    pub default: Option<serde_yaml::Value>,
    #[serde(default)]
    pub auto_now: bool,
    #[serde(default)]
    pub auto_now_add: bool,
    #[serde(default)]
    pub max_length: Option<u32>,
    #[serde(default)]
    pub min_length: Option<u32>,
    #[serde(default)]
    pub min_value: Option<i64>,
    #[serde(default)]
    pub max_value: Option<i64>,
    #[serde(default)]
    pub regex: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Integer,
    String,
    Boolean,
    DateTime,
    Date,
    Time,
    Uuid,
    Json,
    Text,
    Float,
    Double,
    Decimal,
    Binary,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelationshipConfig {
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
    pub model: String,
    pub foreign_key: Option<String>,
    #[serde(default)]
    pub on_delete: OnDeleteAction,
    #[serde(default)]
    pub on_update: OnUpdateAction,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OnDeleteAction {
    Cascade,
    Restrict,
    SetNull,
    SetDefault,
    NoAction,
}

impl Default for OnDeleteAction {
    fn default() -> Self {
        Self::Restrict
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OnUpdateAction {
    Cascade,
    Restrict,
    SetNull,
    SetDefault,
    NoAction,
}

impl Default for OnUpdateAction {
    fn default() -> Self {
        Self::Cascade
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IndexConfig {
    pub name: String,
    pub fields: Vec<String>,
    #[serde(default)]
    pub unique: bool,
    #[serde(rename = "type", default)]
    pub index_type: IndexType,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    Btree,
    Hash,
    Gin,
    Gist,
}

impl Default for IndexType {
    fn default() -> Self {
        Self::Btree
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointConfig {
    pub path: String,
    pub model: String,
    pub crud: CrudConfig,
    #[serde(default)]
    pub auth: Option<AuthConfig>,
    #[serde(default)]
    pub pagination: Option<PaginationConfig>,
    #[serde(default)]
    pub filters: Vec<FilterConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    #[serde(default)]
    pub patch: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    #[serde(rename = "type")]
    pub auth_type: AuthType,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Bearer,
    Basic,
    ApiKey,
    Jwt,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginationConfig {
    #[serde(default = "default_page_size")]
    pub default_page_size: u32,
    #[serde(default = "default_max_page_size")]
    pub max_page_size: u32,
}

fn default_page_size() -> u32 {
    20
}

fn default_max_page_size() -> u32 {
    100
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FilterConfig {
    pub field: String,
    #[serde(rename = "type")]
    pub filter_type: FilterType,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilterType {
    Exact,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    In,
    Between,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum MiddlewareConfig {
    Logger { 
        logger: bool 
    },
    Cors { 
        cors: CorsConfig 
    },
    RateLimit { 
        rate_limit: RateLimitConfig 
    },
    Compression { 
        compression: CompressionConfig 
    },
    Security { 
        security: SecurityConfig 
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CorsConfig {
    pub allow_origin: String,
    #[serde(default)]
    pub allow_methods: Option<Vec<String>>,
    #[serde(default)]
    pub allow_headers: Option<Vec<String>>,
    #[serde(default)]
    pub allow_credentials: bool,
    #[serde(default)]
    pub max_age: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_seconds: u32,
    #[serde(default)]
    pub burst: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompressionConfig {
    #[serde(default)]
    pub gzip: bool,
    #[serde(default)]
    pub brotli: bool,
    #[serde(default)]
    pub level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    #[serde(default)]
    pub helmet: bool,
    #[serde(default)]
    pub content_security_policy: Option<String>,
    #[serde(default)]
    pub x_frame_options: Option<String>,
}