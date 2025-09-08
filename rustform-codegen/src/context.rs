use serde::{Serialize, Deserialize};
use rustform_core::{Config, ModelConfig, FieldConfig, EndpointConfig, MiddlewareConfig, FieldType};
use tera::Context;
use indexmap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationContext {
    pub project_name: String,
    pub version: String,
    pub database: DatabaseContext,
    pub server: ServerContext,
    pub models: Vec<ModelContext>,
    pub endpoints: Vec<EndpointContext>,
    pub middleware: Vec<MiddlewareContext>,
    pub dependencies: Vec<String>,
    pub features: ProjectFeatures,
    pub metadata: ProjectMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseContext {
    pub db_type: String,
    pub url_env: String,
    pub pool_size: Option<u32>,
    pub timeout: Option<u64>,
    pub driver_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerContext {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelContext {
    pub name: String,
    pub table_name: String,
    pub struct_name: String,
    pub fields: Vec<FieldContext>,
    pub relationships: Vec<RelationshipContext>,
    pub indexes: Vec<IndexContext>,
    pub has_primary_key: bool,
    pub primary_key_field: Option<String>,
    pub primary_key_type: Option<String>,
    pub has_timestamps: bool,
    pub imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldContext {
    pub name: String,
    pub field_type: String,
    pub rust_type: String,
    pub sql_type: String,
    pub is_primary_key: bool,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub auto_increment: bool,
    pub auto_now: bool,
    pub auto_now_add: bool,
    pub default_value: Option<String>,
    pub constraints: Vec<String>,
    pub validation: FieldValidation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldValidation {
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub regex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationshipContext {
    pub name: String,
    pub relationship_type: String,
    pub target_model: String,
    pub foreign_key: Option<String>,
    pub on_delete: String,
    pub on_update: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexContext {
    pub name: String,
    pub fields: Vec<String>,
    pub unique: bool,
    pub index_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointContext {
    pub path: String,
    pub model_name: String,
    pub struct_name: String,
    pub operations: Vec<String>,
    pub has_auth: bool,
    pub auth_type: Option<String>,
    pub has_pagination: bool,
    pub pagination: Option<PaginationContext>,
    pub filters: Vec<FilterContext>,
    pub handler_functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationContext {
    pub default_page_size: u32,
    pub max_page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterContext {
    pub field: String,
    pub filter_type: String,
    pub query_param: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiddlewareContext {
    pub name: String,
    pub config: serde_json::Value,
    pub imports: Vec<String>,
    pub setup_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectFeatures {
    pub has_auth: bool,
    pub has_pagination: bool,
    pub has_filtering: bool,
    pub has_relationships: bool,
    pub has_middleware: bool,
    pub has_validation: bool,
    pub has_json_fields: bool,
    pub has_uuid_fields: bool,
    pub has_datetime_fields: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub generated_at: String,
    pub generator_version: String,
    pub total_models: usize,
    pub total_endpoints: usize,
    pub database_type: String,
}

impl GenerationContext {
    pub fn from_config(config: &Config) -> Result<Self, crate::error::CodeGenError> {
        Ok(ContextBuilder::build_context(config))
    }
}

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build_context(config: &Config) -> GenerationContext {
        let models = Self::build_models_context(&config.api.models);
        let endpoints = Self::build_endpoints_context(&config.api.endpoints, &models);
        let middleware = Self::build_middleware_context(&config.middleware);
        let features = Self::analyze_features(&models, &endpoints, &middleware);
        let dependencies = Self::generate_dependencies(&features, &config.database);
        
        GenerationContext {
            project_name: config.project_name.clone(),
            version: config.version.clone(),
            database: Self::build_database_context(&config.database),
            server: Self::build_server_context(&config.server),
            models: models.clone(),
            endpoints,
            middleware,
            dependencies,
            features,
            metadata: Self::build_metadata(&config, &models),
        }
    }
    
    pub fn to_tera_context(gen_context: &GenerationContext) -> Context {
        let mut context = Context::new();
        context.insert("project", gen_context);
        context.insert("project_name", &gen_context.project_name);
        context.insert("version", &gen_context.version);
        context.insert("database", &gen_context.database);
        context.insert("server", &gen_context.server);
        context.insert("models", &gen_context.models);
        context.insert("endpoints", &gen_context.endpoints);
        context.insert("middleware", &gen_context.middleware);
        context.insert("dependencies", &gen_context.dependencies);
        context.insert("features", &gen_context.features);
        context.insert("metadata", &gen_context.metadata);
        context
    }
    
    fn build_database_context(db_config: &rustform_core::DatabaseConfig) -> DatabaseContext {
        let db_type = match db_config.db_type {
            rustform_core::DatabaseType::Sqlite => "sqlite",
            rustform_core::DatabaseType::Postgres => "postgres", 
            rustform_core::DatabaseType::Mysql => "mysql",
        };
        
        let driver_features = match db_config.db_type {
            rustform_core::DatabaseType::Sqlite => vec!["sqlite".to_string()],
            rustform_core::DatabaseType::Postgres => vec!["postgres".to_string()],
            rustform_core::DatabaseType::Mysql => vec!["mysql".to_string()],
        };
        
        DatabaseContext {
            db_type: db_type.to_string(),
            url_env: db_config.url_env.clone(),
            pool_size: db_config.pool_size,
            timeout: db_config.timeout,
            driver_features,
        }
    }
    
    fn build_server_context(server_config: &rustform_core::ServerConfig) -> ServerContext {
        ServerContext {
            host: server_config.host.clone(),
            port: server_config.port,
        }
    }
    
    fn build_models_context(models: &indexmap::IndexMap<String, ModelConfig>) -> Vec<ModelContext> {
        models
            .iter()
            .map(|(name, model)| {
                let fields = Self::build_fields_context(&model.fields);
                let relationships = Self::build_relationships_context(&model.relationships);
                let indexes = Self::build_indexes_context(&model.indexes);
                
                let primary_key_field = fields
                    .iter()
                    .find(|f| f.is_primary_key)
                    .map(|f| f.name.clone());
                
                let primary_key_type = fields
                    .iter()
                    .find(|f| f.is_primary_key)
                    .map(|f| f.rust_type.clone());
                
                let has_timestamps = fields
                    .iter()
                    .any(|f| f.auto_now || f.auto_now_add);
                
                let imports = Self::generate_model_imports(&fields);
                
                ModelContext {
                    name: name.clone(),
                    table_name: model.table_name.clone(),
                    struct_name: Self::to_pascal_case(name),
                    fields,
                    relationships,
                    indexes,
                    has_primary_key: primary_key_field.is_some(),
                    primary_key_field,
                    primary_key_type,
                    has_timestamps,
                    imports,
                }
            })
            .collect()
    }
    
    fn build_fields_context(fields: &indexmap::IndexMap<String, FieldConfig>) -> Vec<FieldContext> {
        fields
            .iter()
            .map(|(name, field)| {
                let rust_type = Self::field_type_to_rust(&field.field_type, field.nullable || !field.required);
                let sql_type = Self::field_type_to_sql(&field.field_type);
                let constraints = Self::build_field_constraints(field);
                
                FieldContext {
                    name: name.clone(),
                    field_type: Self::field_type_to_string(&field.field_type),
                    rust_type,
                    sql_type,
                    is_primary_key: field.primary_key,
                    is_required: field.required,
                    is_unique: field.unique,
                    is_nullable: field.nullable,
                    auto_increment: field.auto_increment,
                    auto_now: field.auto_now,
                    auto_now_add: field.auto_now_add,
                    default_value: field.default.as_ref().map(|v| format!("{:?}", v)),
                    constraints,
                    validation: FieldValidation {
                        max_length: field.max_length,
                        min_length: field.min_length,
                        min_value: field.min_value,
                        max_value: field.max_value,
                        regex: field.regex.clone(),
                    },
                }
            })
            .collect()
    }
    
    fn build_relationships_context(relationships: &indexmap::IndexMap<String, rustform_core::RelationshipConfig>) -> Vec<RelationshipContext> {
        relationships
            .iter()
            .map(|(name, rel)| RelationshipContext {
                name: name.clone(),
                relationship_type: Self::relationship_type_to_string(&rel.relationship_type),
                target_model: rel.model.clone(),
                foreign_key: rel.foreign_key.clone(),
                on_delete: Self::action_to_string(&rel.on_delete),
                on_update: Self::update_action_to_string(&rel.on_update),
            })
            .collect()
    }
    
    fn build_indexes_context(indexes: &[rustform_core::IndexConfig]) -> Vec<IndexContext> {
        indexes
            .iter()
            .map(|idx| IndexContext {
                name: idx.name.clone(),
                fields: idx.fields.clone(),
                unique: idx.unique,
                index_type: Self::index_type_to_string(&idx.index_type),
            })
            .collect()
    }
    
    fn build_endpoints_context(endpoints: &[EndpointConfig], models: &[ModelContext]) -> Vec<EndpointContext> {
        endpoints
            .iter()
            .map(|endpoint| {
                let model = models.iter().find(|m| m.name == endpoint.model).unwrap();
                let operations = Self::build_crud_operations(&endpoint.crud);
                let has_auth = endpoint.auth.is_some();
                let auth_type = endpoint.auth.as_ref().map(|auth| Self::auth_type_to_string(&auth.auth_type));
                let has_pagination = endpoint.pagination.is_some();
                let pagination = endpoint.pagination.as_ref().map(|p| PaginationContext {
                    default_page_size: p.default_page_size,
                    max_page_size: p.max_page_size,
                });
                let filters = Self::build_filters_context(&endpoint.filters);
                let handler_functions = Self::generate_handler_functions(&operations, &endpoint.path);
                
                EndpointContext {
                    path: endpoint.path.clone(),
                    model_name: endpoint.model.clone(),
                    struct_name: model.struct_name.clone(),
                    operations,
                    has_auth,
                    auth_type,
                    has_pagination,
                    pagination,
                    filters,
                    handler_functions,
                }
            })
            .collect()
    }
    
    fn build_middleware_context(middleware: &[MiddlewareConfig]) -> Vec<MiddlewareContext> {
        middleware
            .iter()
            .map(|mw| match mw {
                MiddlewareConfig::Logger { logger } => MiddlewareContext {
                    name: "logger".to_string(),
                    config: serde_json::json!({ "enabled": logger }),
                    imports: vec!["tower_http::trace::TraceLayer".to_string()],
                    setup_code: "TraceLayer::new_for_http()".to_string(),
                },
                MiddlewareConfig::Cors { cors } => MiddlewareContext {
                    name: "cors".to_string(),
                    config: serde_json::to_value(cors).unwrap_or_default(),
                    imports: vec!["tower_http::cors::CorsLayer".to_string()],
                    setup_code: format!("CorsLayer::new().allow_origin(\"{}\")", cors.allow_origin),
                },
                MiddlewareConfig::RateLimit { rate_limit } => MiddlewareContext {
                    name: "rate_limit".to_string(),
                    config: serde_json::to_value(rate_limit).unwrap_or_default(),
                    imports: vec!["tower::limit::RateLimitLayer".to_string()],
                    setup_code: format!("RateLimitLayer::new({}, Duration::from_secs({}))", 
                                       rate_limit.max_requests, rate_limit.window_seconds),
                },
                MiddlewareConfig::Compression { compression } => MiddlewareContext {
                    name: "compression".to_string(),
                    config: serde_json::to_value(compression).unwrap_or_default(),
                    imports: vec!["tower_http::compression::CompressionLayer".to_string()],
                    setup_code: "CompressionLayer::new()".to_string(),
                },
                MiddlewareConfig::Security { security } => MiddlewareContext {
                    name: "security".to_string(),
                    config: serde_json::to_value(security).unwrap_or_default(),
                    imports: vec!["tower_http::set_header::SetResponseHeaderLayer".to_string()],
                    setup_code: "SetResponseHeaderLayer::overriding(\"x-frame-options\", \"DENY\")".to_string(),
                },
            })
            .collect()
    }
    
    fn analyze_features(models: &[ModelContext], endpoints: &[EndpointContext], middleware: &[MiddlewareContext]) -> ProjectFeatures {
        ProjectFeatures {
            has_auth: endpoints.iter().any(|e| e.has_auth),
            has_pagination: endpoints.iter().any(|e| e.has_pagination),
            has_filtering: endpoints.iter().any(|e| !e.filters.is_empty()),
            has_relationships: models.iter().any(|m| !m.relationships.is_empty()),
            has_middleware: !middleware.is_empty(),
            has_validation: models.iter().any(|m| m.fields.iter().any(|f| 
                f.validation.max_length.is_some() || 
                f.validation.min_length.is_some() || 
                f.validation.regex.is_some()
            )),
            has_json_fields: models.iter().any(|m| m.fields.iter().any(|f| f.field_type == "json")),
            has_uuid_fields: models.iter().any(|m| m.fields.iter().any(|f| f.field_type == "uuid")),
            has_datetime_fields: models.iter().any(|m| m.fields.iter().any(|f| 
                f.field_type == "datetime" || f.field_type == "date" || f.field_type == "time"
            )),
        }
    }
    
    fn generate_dependencies(features: &ProjectFeatures, db_config: &rustform_core::DatabaseConfig) -> Vec<String> {
        let mut deps = vec![
            "axum".to_string(),
            "tokio".to_string(),
            "tower".to_string(),
            "tower-http".to_string(),
            "serde".to_string(),
            "serde_json".to_string(),
            "sqlx".to_string(),
            "anyhow".to_string(),
            "tracing".to_string(),
            "tracing-subscriber".to_string(),
        ];
        
        // Database-specific dependencies
        match db_config.db_type {
            rustform_core::DatabaseType::Sqlite => deps.push("sqlx/sqlite".to_string()),
            rustform_core::DatabaseType::Postgres => deps.push("sqlx/postgres".to_string()),
            rustform_core::DatabaseType::Mysql => deps.push("sqlx/mysql".to_string()),
        }
        
        if features.has_uuid_fields {
            deps.push("uuid".to_string());
        }
        
        if features.has_datetime_fields {
            deps.push("chrono".to_string());
        }
        
        if features.has_validation {
            deps.push("validator".to_string());
        }
        
        deps.sort();
        deps.dedup();
        deps
    }
    
    fn build_metadata(config: &Config, models: &[ModelContext]) -> ProjectMetadata {
        use chrono::Utc;
        
        ProjectMetadata {
            generated_at: Utc::now().to_rfc3339(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            total_models: models.len(),
            total_endpoints: 0, // Will be filled in later
            database_type: match config.database.db_type {
                rustform_core::DatabaseType::Sqlite => "sqlite",
                rustform_core::DatabaseType::Postgres => "postgres",
                rustform_core::DatabaseType::Mysql => "mysql",
            }.to_string(),
        }
    }
    
    // Helper functions
    fn field_type_to_rust(field_type: &FieldType, optional: bool) -> String {
        let base_type = match field_type {
            FieldType::Integer => "i32",
            FieldType::String => "String",
            FieldType::Boolean => "bool",
            FieldType::DateTime => "chrono::DateTime<chrono::Utc>",
            FieldType::Date => "chrono::NaiveDate",
            FieldType::Time => "chrono::NaiveTime",
            FieldType::Uuid => "uuid::Uuid",
            FieldType::Json => "serde_json::Value",
            FieldType::Text => "String",
            FieldType::Float => "f32",
            FieldType::Double => "f64",
            FieldType::Decimal => "rust_decimal::Decimal",
            FieldType::Binary => "Vec<u8>",
        };
        
        if optional {
            format!("Option<{}>", base_type)
        } else {
            base_type.to_string()
        }
    }
    
    fn field_type_to_sql(field_type: &FieldType) -> String {
        match field_type {
            FieldType::Integer => "INTEGER",
            FieldType::String => "VARCHAR",
            FieldType::Boolean => "BOOLEAN",
            FieldType::DateTime => "TIMESTAMP",
            FieldType::Date => "DATE",
            FieldType::Time => "TIME",
            FieldType::Uuid => "UUID",
            FieldType::Json => "JSON",
            FieldType::Text => "TEXT",
            FieldType::Float => "REAL",
            FieldType::Double => "DOUBLE PRECISION",
            FieldType::Decimal => "DECIMAL",
            FieldType::Binary => "BLOB",
        }.to_string()
    }
    
    fn field_type_to_string(field_type: &FieldType) -> String {
        match field_type {
            FieldType::Integer => "integer",
            FieldType::String => "string", 
            FieldType::Boolean => "boolean",
            FieldType::DateTime => "datetime",
            FieldType::Date => "date",
            FieldType::Time => "time",
            FieldType::Uuid => "uuid",
            FieldType::Json => "json",
            FieldType::Text => "text",
            FieldType::Float => "float",
            FieldType::Double => "double",
            FieldType::Decimal => "decimal",
            FieldType::Binary => "binary",
        }.to_string()
    }
    
    fn relationship_type_to_string(rel_type: &rustform_core::RelationshipType) -> String {
        match rel_type {
            rustform_core::RelationshipType::OneToOne => "one_to_one",
            rustform_core::RelationshipType::OneToMany => "one_to_many",
            rustform_core::RelationshipType::ManyToOne => "many_to_one",
            rustform_core::RelationshipType::ManyToMany => "many_to_many",
        }.to_string()
    }
    
    fn action_to_string(action: &rustform_core::OnDeleteAction) -> String {
        match action {
            rustform_core::OnDeleteAction::Cascade => "cascade",
            rustform_core::OnDeleteAction::Restrict => "restrict",
            rustform_core::OnDeleteAction::SetNull => "set_null",
            rustform_core::OnDeleteAction::SetDefault => "set_default",
            rustform_core::OnDeleteAction::NoAction => "no_action",
        }.to_string()
    }
    
    fn update_action_to_string(action: &rustform_core::OnUpdateAction) -> String {
        match action {
            rustform_core::OnUpdateAction::Cascade => "cascade",
            rustform_core::OnUpdateAction::Restrict => "restrict",
            rustform_core::OnUpdateAction::SetNull => "set_null",
            rustform_core::OnUpdateAction::SetDefault => "set_default",
            rustform_core::OnUpdateAction::NoAction => "no_action",
        }.to_string()
    }
    
    fn index_type_to_string(index_type: &rustform_core::IndexType) -> String {
        match index_type {
            rustform_core::IndexType::Btree => "btree",
            rustform_core::IndexType::Hash => "hash",
            rustform_core::IndexType::Gin => "gin",
            rustform_core::IndexType::Gist => "gist",
        }.to_string()
    }
    
    fn auth_type_to_string(auth_type: &rustform_core::AuthType) -> String {
        match auth_type {
            rustform_core::AuthType::Bearer => "bearer",
            rustform_core::AuthType::Basic => "basic",
            rustform_core::AuthType::ApiKey => "api_key",
            rustform_core::AuthType::Jwt => "jwt",
        }.to_string()
    }
    
    fn build_crud_operations(crud: &rustform_core::CrudConfig) -> Vec<String> {
        let mut ops = Vec::new();
        if crud.create { ops.push("create".to_string()); }
        if crud.read_all { ops.push("read_all".to_string()); }
        if crud.read_one { ops.push("read_one".to_string()); }
        if crud.update { ops.push("update".to_string()); }
        if crud.patch { ops.push("patch".to_string()); }
        if crud.delete { ops.push("delete".to_string()); }
        ops
    }
    
    fn build_filters_context(filters: &[rustform_core::FilterConfig]) -> Vec<FilterContext> {
        filters
            .iter()
            .map(|filter| FilterContext {
                field: filter.field.clone(),
                filter_type: Self::filter_type_to_string(&filter.filter_type),
                query_param: Self::to_snake_case(&filter.field),
            })
            .collect()
    }
    
    fn filter_type_to_string(filter_type: &rustform_core::FilterType) -> String {
        match filter_type {
            rustform_core::FilterType::Exact => "exact",
            rustform_core::FilterType::Contains => "contains",
            rustform_core::FilterType::StartsWith => "starts_with",
            rustform_core::FilterType::EndsWith => "ends_with",
            rustform_core::FilterType::GreaterThan => "greater_than",
            rustform_core::FilterType::LessThan => "less_than",
            rustform_core::FilterType::GreaterThanOrEqual => "greater_than_or_equal",
            rustform_core::FilterType::LessThanOrEqual => "less_than_or_equal",
            rustform_core::FilterType::In => "in",
            rustform_core::FilterType::Between => "between",
        }.to_string()
    }
    
    fn generate_handler_functions(operations: &[String], path: &str) -> Vec<String> {
        let base_name = path.trim_start_matches('/').replace('/', "_");
        operations
            .iter()
            .map(|op| format!("{}_{}", op, base_name))
            .collect()
    }
    
    fn build_field_constraints(field: &FieldConfig) -> Vec<String> {
        let mut constraints = Vec::new();
        
        if field.required {
            constraints.push("NOT NULL".to_string());
        }
        
        if field.unique {
            constraints.push("UNIQUE".to_string());
        }
        
        if field.primary_key {
            constraints.push("PRIMARY KEY".to_string());
        }
        
        if field.auto_increment {
            constraints.push("AUTOINCREMENT".to_string());
        }
        
        constraints
    }
    
    fn generate_model_imports(fields: &[FieldContext]) -> Vec<String> {
        let mut imports = Vec::new();
        
        if fields.iter().any(|f| f.field_type == "uuid") {
            imports.push("uuid::Uuid".to_string());
        }
        
        if fields.iter().any(|f| f.field_type == "datetime" || f.field_type == "date" || f.field_type == "time") {
            imports.push("chrono".to_string());
        }
        
        if fields.iter().any(|f| f.field_type == "json") {
            imports.push("serde_json::Value".to_string());
        }
        
        if fields.iter().any(|f| f.field_type == "decimal") {
            imports.push("rust_decimal::Decimal".to_string());
        }
        
        imports.push("serde::{Deserialize, Serialize}".to_string());
        imports.push("sqlx::FromRow".to_string());
        
        imports.sort();
        imports.dedup();
        imports
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
}