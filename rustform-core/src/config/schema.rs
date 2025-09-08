use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

/// Default schema version for new configurations
fn default_schema_version() -> String {
    "1.0.0".to_string()
}

/// Default API version for new configurations  
fn default_api_version() -> String {
    "0.1.0".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Version of the YAML configuration schema format
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    /// Version of rust-form API this configuration targets
    #[serde(default = "default_api_version")]
    pub api_version: String,
    /// Application project name
    pub project_name: String,
    /// Application version (SemVer)
    pub version: String,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    #[serde(default)]
    pub middleware: Vec<MiddlewareConfig>,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub frontend: Option<FrontendConfig>,
    #[serde(default)]
    pub components: Option<IndexMap<String, String>>,
    /// Feature flags for experimental functionality
    #[serde(default)]
    #[cfg(feature = "registry")]
    pub registry: Option<RegistryConfig>,
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
    #[serde(default)]
    pub custom_logic: Option<CustomLogicConfig>,
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
    #[serde(default)]
    pub custom_handlers: Option<CustomHandlersConfig>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrontendConfig {
    pub target: String,
    pub typescript_output_dir: String,
    pub generate_ui_for: Vec<String>,
    #[serde(default)]
    pub auto_generate_types: bool,
    #[serde(default)]
    pub api_base_url: Option<String>,
    #[serde(default)]
    pub components: IndexMap<String, ComponentConfig>,
    #[serde(default)]
    pub framework_config: Option<FrameworkConfig>,
    #[serde(default)]
    pub export: Option<ExportConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComponentConfig {
    #[serde(default)]
    pub generate: Vec<String>,
    #[serde(default)]
    pub form_fields: Vec<String>,
    #[serde(default)]
    pub list_columns: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub pagination: bool,
    #[serde(default)]
    pub search_fields: Vec<String>,
    #[serde(default)]
    pub relationships: IndexMap<String, RelationshipDisplayConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelationshipDisplayConfig {
    pub display_field: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrameworkConfig {
    #[serde(default)]
    pub react: Option<ReactConfig>,
    #[serde(default)]
    pub vue: Option<VueConfig>,
    #[serde(default)]
    pub svelte: Option<SvelteConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReactConfig {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub typescript: bool,
    #[serde(default)]
    pub state_management: Option<String>,
    #[serde(default)]
    pub styling: Option<String>,
    #[serde(default)]
    pub forms: Option<String>,
    #[serde(default)]
    pub routing: Option<String>,
    #[serde(default)]
    pub build_tool: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VueConfig {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub typescript: bool,
    #[serde(default)]
    pub state_management: Option<String>,
    #[serde(default)]
    pub styling: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SvelteConfig {
    #[serde(default)]
    pub kit: bool,
    #[serde(default)]
    pub typescript: bool,
    #[serde(default)]
    pub styling: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExportConfig {
    pub package_name: String,
    pub version: String,
    pub components: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomLogicConfig {
    pub file: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub methods: Vec<String>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub hooks: Option<CustomHooksConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomHooksConfig {
    #[serde(default)]
    pub before_create: Option<String>,
    #[serde(default)]
    pub after_create: Option<String>,
    #[serde(default)]
    pub before_update: Option<String>,
    #[serde(default)]
    pub after_update: Option<String>,
    #[serde(default)]
    pub before_delete: Option<String>,
    #[serde(default)]
    pub after_delete: Option<String>,
    #[serde(default)]
    pub before_query: Option<String>,
    #[serde(default)]
    pub after_query: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomHandlersConfig {
    pub file: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub handlers: Vec<String>,
    #[serde(default)]
    pub middleware: Vec<String>,
    #[serde(default)]
    pub validation: Option<CustomValidationConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomValidationConfig {
    #[serde(default)]
    pub before_create: Option<String>,
    #[serde(default)]
    pub before_update: Option<String>,
    #[serde(default)]
    pub custom_validators: Vec<String>,
}

/// Registry configuration for component management
#[cfg(feature = "registry")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegistryConfig {
    /// Registry URL endpoint
    pub url: String,
    /// Authentication configuration
    pub auth: Option<RegistryAuthConfig>,
    /// Caching configuration
    #[serde(default)]
    pub cache: RegistryCacheConfig,
}

/// Registry authentication configuration
#[cfg(feature = "registry")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegistryAuthConfig {
    /// Authentication type (token, basic, etc.)
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Token or credentials
    pub credentials: String,
}

/// Registry caching configuration
#[cfg(feature = "registry")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegistryCacheConfig {
    /// Enable component caching
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Cache TTL in seconds
    #[serde(default = "default_cache_ttl")]
    pub ttl: u64,
    /// Cache directory
    #[serde(default = "default_cache_dir")]
    pub directory: String,
}

#[cfg(feature = "registry")]
fn default_true() -> bool {
    true
}

#[cfg(feature = "registry")]
fn default_cache_ttl() -> u64 {
    3600 // 1 hour
}

#[cfg(feature = "registry")]
fn default_cache_dir() -> String {
    ".rustform/cache".to_string()
}