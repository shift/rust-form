# Configuration Reference

This document provides a comprehensive reference for all configuration options available in Rust-form YAML files.

## 📋 Table of Contents

- [Project Configuration](#project-configuration)
- [Database Configuration](#database-configuration)
- [Server Configuration](#server-configuration)
- [Models](#models)
- [Frontend Configuration](#frontend-configuration)
- [Complete Examples](#complete-examples)

## 🏗️ Project Configuration

Basic project settings and metadata.

```yaml
project:
  name: "my_app"                    # Required: Project name (snake_case recommended)
  description: "My awesome app"     # Optional: Project description
  version: "0.1.0"                  # Optional: Version (defaults to "0.1.0")
  authors: ["Your Name <email>"]    # Optional: List of authors
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | ✅ | Project name, used for generated binary and package names |
| `description` | `string` | ❌ | Human-readable project description |
| `version` | `string` | ❌ | Semantic version (default: "0.1.0") |
| `authors` | `string[]` | ❌ | List of project authors |

## 🗄️ Database Configuration

Database connection and settings.

```yaml
database:
  type: "sqlite"                    # Required: sqlite, postgres, mysql
  url_env: "DATABASE_URL"           # Required: Environment variable name
  
  # Optional: Connection pool settings
  pool:
    max_connections: 10
    min_connections: 1
    connect_timeout: 30
    idle_timeout: 600
    
  # Optional: Migration settings
  migrations:
    auto_run: true                  # Run migrations on startup
    directory: "./migrations"       # Migration files directory
```

### Supported Database Types

| Type | Description | Connection String Example |
|------|-------------|---------------------------|
| `sqlite` | SQLite database | `sqlite:./app.db` |
| `postgres` | PostgreSQL database | `postgres://user:pass@localhost/db` |
| `mysql` | MySQL/MariaDB database | `mysql://user:pass@localhost/db` |

### Pool Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_connections` | `i32` | `10` | Maximum number of connections |
| `min_connections` | `i32` | `1` | Minimum number of connections |
| `connect_timeout` | `i32` | `30` | Connection timeout in seconds |
| `idle_timeout` | `i32` | `600` | Idle connection timeout in seconds |

## 🌐 Server Configuration

Web server and middleware settings.

```yaml
server:
  port: 8080                        # Required: Server port
  host: "127.0.0.1"                # Optional: Bind address (default: "127.0.0.1")
  
  # Optional: CORS configuration
  cors:
    origins: ["http://localhost:3000", "https://myapp.com"]
    methods: ["GET", "POST", "PUT", "DELETE"]
    headers: ["content-type", "authorization"]
    credentials: true
    max_age: 3600
    
  # Optional: Security headers
  security:
    x_frame_options: "DENY"
    x_content_type_options: "nosniff"
    x_xss_protection: "1; mode=block"
    strict_transport_security: "max-age=31536000; includeSubDomains"
    
  # Optional: Rate limiting
  rate_limit:
    requests_per_minute: 60
    burst: 10
```

### CORS Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `origins` | `string[]` | `["*"]` | Allowed origins |
| `methods` | `string[]` | `["GET", "POST", "PUT", "DELETE"]` | Allowed HTTP methods |
| `headers` | `string[]` | `["content-type"]` | Allowed headers |
| `credentials` | `boolean` | `false` | Allow credentials |
| `max_age` | `i32` | `3600` | Preflight cache duration |

## 📊 Models

Define your data models and their fields.

```yaml
models:
  User:
    fields:
      id:
        type: "uuid"
        primary_key: true
        default: "gen_random_uuid()"
      
      email:
        type: "string"
        unique: true
        validation:
          format: "email"
          message: "Must be a valid email address"
      
      name:
        type: "string"
        validation:
          min_length: 1
          max_length: 100
          pattern: "^[a-zA-Z\\s]+$"
          message: "Name can only contain letters and spaces"
      
      age:
        type: "i32"
        optional: true
        validation:
          min: 13
          max: 120
          message: "Age must be between 13 and 120"
      
      is_active:
        type: "boolean"
        default: false
      
      tags:
        type: "json"
        default: "[]"
        array_of: "string"
        
      metadata:
        type: "json"
        default: "{}"
        schema:
          properties:
            preferences:
              type: "object"
              properties:
                theme: { type: "string", enum: ["light", "dark"] }
                language: { type: "string", default: "en" }
      
      created_at:
        type: "datetime"
        default: "now()"
      
      updated_at:
        type: "datetime"
        auto_update: true
```

### Field Types

| Type | Rust Type | TypeScript Type | Description |
|------|-----------|-----------------|-------------|
| `string` | `String` | `string` | Text string |
| `text` | `String` | `string` | Long text (TEXT column) |
| `i32` | `i32` | `number` | 32-bit integer |
| `i64` | `i64` | `number` | 64-bit integer |
| `f32` | `f32` | `number` | 32-bit float |
| `f64` | `f64` | `number` | 64-bit float |
| `boolean` | `bool` | `boolean` | Boolean value |
| `uuid` | `uuid::Uuid` | `string` | UUID identifier |
| `date` | `chrono::NaiveDate` | `string` | Date only |
| `datetime` | `chrono::DateTime<Utc>` | `string` | Date and time |
| `time` | `chrono::NaiveTime` | `string` | Time only |
| `json` | `serde_json::Value` | `any` | JSON data |
| `decimal` | `rust_decimal::Decimal` | `string` | Precise decimal |
| `enum` | Custom enum | Union type | Enumeration |

### Field Properties

| Property | Type | Description |
|----------|------|-------------|
| `type` | `string` | Field data type (required) |
| `primary_key` | `boolean` | Mark as primary key |
| `unique` | `boolean` | Unique constraint |
| `optional` | `boolean` | Allow NULL values |
| `default` | `string` | Default value |
| `auto_update` | `boolean` | Auto-update on modification |
| `validation` | `object` | Validation rules |
| `references` | `string` | Foreign key reference |
| `on_delete` | `string` | Foreign key delete action |

### Validation Rules

```yaml
validation:
  # String validation
  min_length: 1
  max_length: 255
  pattern: "^[a-zA-Z0-9]+$"
  format: "email"                   # email, url, uuid
  
  # Number validation
  min: 0
  max: 100
  
  # Custom validation
  custom_validator: "my_validator"
  message: "Custom error message"
  
  # Array validation
  min_items: 1
  max_items: 10
  unique_items: true
```

### Enum Fields

```yaml
status:
  type: "enum"
  values: ["draft", "published", "archived"]
  default: "draft"
```

### JSON Fields

```yaml
# Simple JSON array
tags:
  type: "json"
  array_of: "string"
  default: "[]"

# JSON with schema validation
settings:
  type: "json"
  default: "{}"
  schema:
    type: "object"
    properties:
      theme: { type: "string", enum: ["light", "dark"] }
      notifications: { type: "boolean", default: true }
```

### Relationships

```yaml
models:
  Post:
    fields:
      id: { type: "uuid", primary_key: true }
      author_id:
        type: "uuid"
        references: "User.id"          # Foreign key
        on_delete: "CASCADE"           # CASCADE, RESTRICT, SET_NULL
      category_id:
        type: "uuid"
        references: "Category.id"
        on_delete: "SET_NULL"
        optional: true
```

## 🎨 Frontend Configuration

Configure frontend framework and component generation.

```yaml
frontend:
  # Framework selection
  target: "react"                   # react, vue, svelte
  
  # Output configuration
  typescript_output_dir: "../frontend/src/generated"
  
  # Component generation
  generate_ui_for: ["User", "Post", "Category"]
  
  # Per-model component configuration
  components:
    User:
      generate: ["form", "list", "card"]
      form_fields: ["name", "email", "age"]
      list_columns: ["name", "email", "created_at"]
      features: ["create", "edit", "delete", "search"]
      pagination: true
      
    Post:
      generate: ["form", "list"]
      form_fields: ["title", "content", "category_id"]
      list_columns: ["title", "author", "category", "created_at"]
      features: ["create", "edit", "delete", "search", "filter"]
      
  # Framework-specific configuration
  framework_config:
    react:
      version: "18"
      typescript: true
      state_management: "react-query"  # react-query, zustand, redux-toolkit
      styling: "tailwind"              # tailwind, styled-components, mui
      forms: "react-hook-form"         # react-hook-form, formik
      routing: "react-router"          # react-router, next-router
      
    vue:
      version: "3"
      composition_api: true
      state_management: "pinia"        # pinia, vuex
      styling: "tailwind"              # tailwind, vuetify, quasar
      forms: "vee-validate"            # vee-validate, vue-form
      
    svelte:
      version: "4"
      state_management: "svelte-store" # svelte-store, nanostores
      styling: "tailwind"              # tailwind, svelte-ui, skeleton
      forms: "svelte-forms-lib"        # svelte-forms-lib, felte
      
  # Global settings
  auto_generate_types: true          # Generate TypeScript from Rust
  api_base_url: "http://localhost:8080"
  
  # Component library export
  export:
    package_name: "@myapp/components"
    version: "1.0.0"
    registry: "npm"
```

### Frontend Framework Targets

| Target | Description | Status |
|--------|-------------|--------|
| `react` | React with TypeScript | ✅ Available |
| `vue` | Vue 3 with Composition API | 🔄 Planned |
| `svelte` | Svelte/SvelteKit | 🔄 Planned |
| `angular` | Angular with TypeScript | 🔄 Future |
| `solid` | SolidJS | 🔄 Future |

### Component Generation Options

| Component | Description | Generated Files |
|-----------|-------------|-----------------|
| `form` | Create/edit forms | `ModelForm.tsx` |
| `list` | Data tables with pagination | `ModelList.tsx` |
| `card` | Individual item display | `ModelCard.tsx` |
| `detail` | Detailed view component | `ModelDetail.tsx` |

### Component Features

| Feature | Description |
|---------|-------------|
| `create` | Create new items |
| `edit` | Edit existing items |
| `delete` | Delete items |
| `search` | Search/filter functionality |
| `filter` | Advanced filtering |
| `sort` | Column sorting |
| `pagination` | Paginated results |
| `export` | Export to CSV/Excel |

## 🔧 Advanced Configuration

### Environment Variables

```yaml
# Reference environment variables
database:
  url_env: "DATABASE_URL"
  
# With defaults
server:
  port_env: "PORT"
  port_default: 8080
  
# Boolean environment variables
features:
  debug_env: "DEBUG_MODE"
  debug_default: false
```

### Conditional Configuration

```yaml
# Environment-specific settings
environments:
  development:
    database:
      type: "sqlite"
      url_env: "DEV_DATABASE_URL"
    server:
      port: 3000
      
  production:
    database:
      type: "postgres"
      url_env: "DATABASE_URL"
    server:
      port_env: "PORT"
      
  test:
    database:
      type: "sqlite"
      url: ":memory:"
```

### Custom Middleware

```yaml
middleware:
  - logger:
      format: "combined"
      
  - cors:
      origins: ["*"]
      
  - rate_limit:
      requests_per_minute: 60
      
  - auth:
      jwt_secret_env: "JWT_SECRET"
      public_routes: ["/health", "/docs"]
      
  - compression:
      level: 6
```

## 📝 Complete Examples

### Simple Blog

```yaml
project:
  name: "blog_api"
  description: "A simple blog API"

database:
  type: "sqlite"
  url_env: "DATABASE_URL"

server:
  port: 8080

models:
  Post:
    fields:
      id: { type: "uuid", primary_key: true }
      title: { type: "string", validation: { min_length: 1, max_length: 200 } }
      content: { type: "text" }
      published: { type: "boolean", default: false }
      created_at: { type: "datetime", default: "now()" }

frontend:
  target: "react"
  generate_ui_for: ["Post"]
```

### E-commerce Platform

```yaml
project:
  name: "ecommerce_api"
  description: "E-commerce platform API"

database:
  type: "postgres"
  url_env: "DATABASE_URL"

server:
  port: 8080
  cors:
    origins: ["https://mystore.com"]

models:
  User:
    fields:
      id: { type: "uuid", primary_key: true }
      email: { type: "string", unique: true, validation: { format: "email" } }
      name: { type: "string", validation: { min_length: 1, max_length: 100 } }
      created_at: { type: "datetime", default: "now()" }

  Product:
    fields:
      id: { type: "uuid", primary_key: true }
      name: { type: "string", validation: { min_length: 1, max_length: 200 } }
      price: { type: "decimal", validation: { min: 0.01 } }
      description: { type: "text", optional: true }
      stock: { type: "i32", default: 0, validation: { min: 0 } }
      created_at: { type: "datetime", default: "now()" }

  Order:
    fields:
      id: { type: "uuid", primary_key: true }
      user_id: { type: "uuid", references: "User.id", on_delete: "RESTRICT" }
      total: { type: "decimal", validation: { min: 0 } }
      status: { type: "enum", values: ["pending", "paid", "shipped", "delivered"] }
      created_at: { type: "datetime", default: "now()" }

frontend:
  target: "react"
  generate_ui_for: ["Product", "Order"]
  components:
    Product:
      form_fields: ["name", "price", "description", "stock"]
      list_columns: ["name", "price", "stock", "created_at"]
      features: ["create", "edit", "delete", "search"]
    Order:
      list_columns: ["user", "total", "status", "created_at"]
      features: ["edit", "search", "filter"]
```

### Multi-tenant SaaS

```yaml
project:
  name: "saas_platform"
  description: "Multi-tenant SaaS platform"

database:
  type: "postgres"
  url_env: "DATABASE_URL"
  pool:
    max_connections: 20

server:
  port: 8080
  security:
    strict_transport_security: "max-age=31536000; includeSubDomains"

models:
  Tenant:
    fields:
      id: { type: "uuid", primary_key: true }
      name: { type: "string", unique: true }
      plan: { type: "enum", values: ["free", "pro", "enterprise"] }
      settings: { type: "json", default: "{}" }
      created_at: { type: "datetime", default: "now()" }

  User:
    fields:
      id: { type: "uuid", primary_key: true }
      tenant_id: { type: "uuid", references: "Tenant.id", on_delete: "CASCADE" }
      email: { type: "string", validation: { format: "email" } }
      role: { type: "enum", values: ["admin", "user"], default: "user" }
      created_at: { type: "datetime", default: "now()" }

  Project:
    fields:
      id: { type: "uuid", primary_key: true }
      tenant_id: { type: "uuid", references: "Tenant.id", on_delete: "CASCADE" }
      name: { type: "string", validation: { min_length: 1, max_length: 100 } }
      description: { type: "text", optional: true }
      created_by: { type: "uuid", references: "User.id", on_delete: "SET_NULL", optional: true }
      created_at: { type: "datetime", default: "now()" }

frontend:
  target: "react"
  generate_ui_for: ["Project"]
  framework_config:
    react:
      state_management: "react-query"
      styling: "tailwind"
```

## 🔍 Validation and Schema

### YAML Schema Validation

Rust-form validates your configuration against a JSON schema. Common validation errors:

```yaml
# ❌ Invalid - missing required fields
project:
  description: "Missing name field"

# ❌ Invalid - unknown field type
models:
  User:
    fields:
      id: { type: "invalid_type" }

# ❌ Invalid - invalid validation rule
models:
  User:
    fields:
      email: 
        type: "string"
        validation:
          min_length: -1  # Must be positive

# ✅ Valid
project:
  name: "my_app"
models:
  User:
    fields:
      id: { type: "uuid", primary_key: true }
      email: { type: "string", validation: { format: "email" } }
```

### Best Practices

1. **Use descriptive names** for projects and models
2. **Always define primary keys** for models
3. **Add validation rules** for user input fields
4. **Use appropriate field types** for your data
5. **Document complex JSON schemas** with comments
6. **Test configurations** with small examples first

This comprehensive reference should help you create powerful, type-safe applications with Rust-form!