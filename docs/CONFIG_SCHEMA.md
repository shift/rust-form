# Rust-form Configuration Schema Documentation

## Overview
Rust-form uses a declarative YAML configuration to generate complete web backends and frontends. This document details the complete schema for configuration files.

## Root Configuration

```yaml
project_name: string              # Required: Project name (valid Rust crate name)
version: string                   # Required: Project version (semver format)
database: DatabaseConfig          # Required: Database configuration
api: ApiConfig                    # Required: API models and endpoints
server: ServerConfig              # Optional: Server configuration
middleware: MiddlewareConfig[]     # Optional: Middleware stack
frontend: FrontendConfig          # Optional: Frontend generation
components: ComponentMap          # Optional: Component dependencies
```

## Database Configuration

```yaml
database:
  type: sqlite | postgres | mysql  # Required: Database type
  url_env: string                  # Required: Environment variable for DB URL
  pool_size: number               # Optional: Connection pool size
  timeout: number                 # Optional: Connection timeout (seconds)
```

## Server Configuration

```yaml
server:
  host: string                    # Optional: Default "127.0.0.1"
  port: number                    # Optional: Default 3000
```

## API Configuration

### Models

```yaml
api:
  models:
    ModelName:                    # Model name (PascalCase)
      table_name: string          # Required: Database table name
      fields:                     # Required: Model fields
        field_name:
          type: FieldType         # Required: Field data type
          primary_key: boolean    # Optional: Is primary key
          auto_increment: boolean # Optional: Auto increment
          required: boolean       # Optional: Is required/not null
          unique: boolean         # Optional: Unique constraint
          nullable: boolean       # Optional: Allow null values
          default: any            # Optional: Default value
          auto_now: boolean       # Optional: Update on save
          auto_now_add: boolean   # Optional: Set on creation
          max_length: number      # Optional: Max string length
          min_length: number      # Optional: Min string length
          min_value: number       # Optional: Min numeric value
          max_value: number       # Optional: Max numeric value
          regex: string           # Optional: Validation regex
      relationships:              # Optional: Model relationships
        relationship_name:
          type: RelationshipType  # one_to_one, one_to_many, etc.
          model: string           # Target model name
          foreign_key: string     # Optional: Foreign key field
          on_delete: Action       # cascade, restrict, set_null, etc.
          on_update: Action       # cascade, restrict, set_null, etc.
      indexes:                    # Optional: Database indexes
        - name: string            # Index name
          fields: string[]        # Fields to index
          unique: boolean         # Optional: Unique index
          type: IndexType         # btree, hash, gin, gist
```

### Field Types

- `integer` - 32/64-bit integers
- `string` - Variable length text
- `boolean` - True/false values  
- `datetime` - Timestamp with timezone
- `date` - Date only
- `time` - Time only
- `uuid` - UUID v4 identifiers
- `json` - JSON data
- `text` - Large text fields
- `float` - Single precision floating point
- `double` - Double precision floating point
- `decimal` - Fixed precision decimal
- `binary` - Binary data

### Endpoints

```yaml
api:
  endpoints:
    - path: string              # Required: API path (e.g., "/users")
      model: string             # Required: Associated model name
      crud:                     # Required: CRUD operations
        create: boolean         # Enable POST endpoint
        read_all: boolean       # Enable GET collection endpoint  
        read_one: boolean       # Enable GET single item endpoint
        update: boolean         # Enable PUT endpoint
        delete: boolean         # Enable DELETE endpoint
        patch: boolean          # Enable PATCH endpoint
      auth:                     # Optional: Authentication
        type: AuthType          # bearer, basic, api_key, jwt
        required: boolean       # Is auth required
      pagination:               # Optional: Pagination config
        default_page_size: number # Default page size
        max_page_size: number   # Maximum allowed page size
      filters:                  # Optional: Query filters
        - field: string         # Field to filter on
          type: FilterType      # Filter operation type
```

### Filter Types

- `exact` - Exact match
- `contains` - String contains
- `starts_with` - String starts with
- `ends_with` - String ends with
- `greater_than` - Numeric/date comparison
- `less_than` - Numeric/date comparison
- `greater_than_or_equal` - Numeric/date comparison
- `less_than_or_equal` - Numeric/date comparison
- `in` - Value in list
- `between` - Value between range

## Middleware Configuration

```yaml
middleware:
  - logger: boolean             # Enable request logging
  
  - cors:                       # CORS configuration
      allow_origin: string      # Allowed origins (* for all)
      allow_methods: string[]   # HTTP methods
      allow_headers: string[]   # HTTP headers
      allow_credentials: boolean # Allow credentials
      max_age: number           # Preflight cache time
  
  - rate_limit:                 # Rate limiting
      max_requests: number      # Max requests per window
      window_seconds: number    # Time window
      burst: number             # Burst allowance
  
  - compression:                # Response compression
      gzip: boolean             # Enable gzip
      brotli: boolean           # Enable brotli
      level: number             # Compression level
  
  - security:                   # Security headers
      helmet: boolean           # Enable helmet.js equivalent
      content_security_policy: string # CSP header
      x_frame_options: string   # X-Frame-Options header
```

## Frontend Configuration

```yaml
frontend:
  target: string                # react, vue, svelte
  typescript_output_dir: string # Output directory for TS files
  generate_ui_for: string[]     # Models to generate UI for
  auto_generate_types: boolean  # Auto-generate TypeScript types
  api_base_url: string          # API base URL for client
  
  components:                   # UI component configuration
    ModelName:
      generate: string[]        # ["form", "list", "card", "detail"]
      form_fields: string[]     # Fields to include in forms
      list_columns: string[]    # Columns for list views
      features: string[]        # ["create", "edit", "delete", "search", "filter"]
      pagination: boolean       # Enable pagination
      search_fields: string[]   # Searchable fields
      relationships:            # Relationship display config
        rel_name:
          display_field: string # Field to display for relations
  
  framework_config:            # Framework-specific settings
    react:
      version: string           # React version
      typescript: boolean       # Use TypeScript
      state_management: string  # "react-query", "redux", "zustand"
      styling: string           # "tailwind", "styled-components", "emotion"
      forms: string             # "react-hook-form", "formik"
      routing: string           # "react-router", "next-router"
      build_tool: string        # "vite", "webpack", "parcel"
    vue:
      version: string           # Vue version
      typescript: boolean       # Use TypeScript
      state_management: string  # "pinia", "vuex"
      styling: string           # "tailwind", "vuetify"
    svelte:
      kit: boolean              # Use SvelteKit
      typescript: boolean       # Use TypeScript
      styling: string           # "tailwind", "bulma"
  
  export:                       # Optional: Export as npm package
    package_name: string        # Package name
    version: string             # Package version
    components: string[]        # Components to export
```

## Component Dependencies

```yaml
components:
  component_name: string        # Component URI (github:org/repo@version, path:./local)
```

Component URIs support multiple schemes:
- `github:org/repo@version` - GitHub repositories
- `gitlab:org/repo@version` - GitLab repositories  
- `path:./local/path` - Local filesystem
- `file:///absolute/path` - Absolute filesystem path
- `registry:package@version` - Rust-form component registry

## Example Complete Configuration

```yaml
project_name: todo_api
version: "0.1.0"

database:
  type: sqlite
  url_env: DATABASE_URL

server:
  host: "127.0.0.1"
  port: 3000

api:
  models:
    Todo:
      table_name: todos
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        title:
          type: string
          required: true
          max_length: 200
        completed:
          type: boolean
          default: false
      indexes:
        - name: idx_todos_completed
          fields: [completed]

  endpoints:
    - path: /todos
      model: Todo
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true

frontend:
  target: react
  typescript_output_dir: "../frontend/src/generated"
  generate_ui_for: ["Todo"]
  auto_generate_types: true
  
  components:
    Todo:
      generate: ["form", "list"]
      form_fields: ["title", "completed"]
      list_columns: ["title", "completed", "created_at"]

middleware:
  - logger: true
  - cors:
      allow_origin: "*"
      allow_methods: ["GET", "POST", "PUT", "DELETE"]

components:
  ui-kit: "github:rust-form/ui-components@^2.0.0"
```

## Validation Rules

- `project_name` must be a valid Rust crate name (lowercase, hyphens allowed)
- `version` must follow semantic versioning
- Model names must be PascalCase
- Field names must be snake_case
- At least one model and endpoint is required
- Primary key fields are automatically required
- Auto-increment fields must be integer type
- Foreign key fields must reference existing models
- Endpoint paths must start with "/"
- Component URIs must be valid according to their scheme

## Generated Files

Based on configuration, Rust-form generates:

### Backend Files
- `src/main.rs` - Application entry point
- `src/models.rs` - Database models
- `src/handlers.rs` - API handlers
- `src/database.rs` - Database connection
- `src/error.rs` - Error handling
- `Cargo.toml` - Rust dependencies
- `migrations/` - Database migrations
- `.env.example` - Environment template

### Frontend Files (if configured)
- `types.ts` - TypeScript type definitions
- `api-client.ts` - API client code
- `hooks.ts` - React Query hooks (React)
- `components/` - UI components
- `package.json` - Frontend dependencies

This schema provides a complete declarative interface for generating modern web applications with Rust backends and optional frontend frameworks.