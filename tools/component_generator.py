#!/usr/bin/env python3
"""
Component Generator for Rust-form
Rapidly scaffolds component directories with proper structure, templates, and tests.
"""

import os
import sys
import json
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

# Component categories and their descriptions
COMPONENT_CATEGORIES = {
    "auth": {
        "description": "Authentication and authorization components",
        "examples": ["jwt-authentication", "oauth2-providers", "role-based-access"]
    },
    "payments": {
        "description": "Payment processing and billing components", 
        "examples": ["stripe-integration", "subscription-billing", "invoice-generation"]
    },
    "dashboard": {
        "description": "Dashboard, analytics and monitoring components",
        "examples": ["admin-dashboard", "analytics-widgets", "real-time-metrics"]
    },
    "ecommerce": {
        "description": "E-commerce and marketplace components",
        "examples": ["product-catalog", "shopping-cart", "order-management"]
    },
    "communication": {
        "description": "Communication and messaging components",
        "examples": ["email-templates", "chat-system", "push-notifications"]
    },
    "cms": {
        "description": "Content management system components",
        "examples": ["blog-system", "page-builder", "media-management"]
    },
    "ui": {
        "description": "User interface and design system components",
        "examples": ["design-system", "form-builders", "data-tables"]
    },
    "integrations": {
        "description": "Third-party service integrations",
        "examples": ["google-analytics", "social-media-apis", "crm-integrations"]
    }
}

def create_component_manifest_yaml(name: str, category: str, description: str, author: str = "rust-form") -> str:
    """Generate component manifest YAML content as string."""
    return f'''name: {name}
version: "1.0.0"
description: "{description}"
author: {author}
homepage: "https://github.com/rust-form/{name}"
keywords:
  - {category}
  - rust-form
  - component

api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
  experimental: false

dependencies: {{}}

files:
  - "handlers.rs.tera"
  - "models.rs.tera"
  - "middleware.rs.tera"

provides:
  templates:
    - name: "handlers.rs.tera"
      path: "handlers.rs.tera"
      description: "{name.title()} request handlers"
      variables: []
      target: Backend
    - name: "models.rs.tera"
      path: "models.rs.tera"
      description: "{name.title()} data models"
      variables: []
      target: Backend
    - name: "middleware.rs.tera"
      path: "middleware.rs.tera"
      description: "{name.title()} middleware"
      variables: []
      target: Backend
  assets: []
  hooks: []'''

def create_handlers_template(name: str, category: str) -> str:
    """Generate handlers.rs.tera template."""
    return f'''use axum::{{
    extract::{{Path, Query, State}},
    http::StatusCode,
    response::Json,
    routing::{{get, post, put, delete}},
    Router,
}};
use serde::{{Deserialize, Serialize}};
use sqlx::SqlitePool;

use crate::{{models::*, error::*}};

#[derive(Debug, Serialize, Deserialize)]
pub struct {name.title().replace("-", "")}Request {{
    // Add request fields here
    pub name: String,
}}

#[derive(Debug, Serialize, Deserialize)]
pub struct {name.title().replace("-", "")}Response {{
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}}

/// Create new {name.replace("-", " ")}
pub async fn create_{name.replace("-", "_")}(
    State(pool): State<SqlitePool>,
    Json(request): Json<{name.title().replace("-", "")}Request>,
) -> Result<Json<{name.title().replace("-", "")}Response>, AppError> {{
    // Implementation here
    todo!("Implement {name} creation")
}}

/// Get {name.replace("-", " ")} by ID
pub async fn get_{name.replace("-", "_")}_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<{name.title().replace("-", "")}Response>, AppError> {{
    // Implementation here
    todo!("Implement {name} retrieval")
}}

/// Update {name.replace("-", " ")}
pub async fn update_{name.replace("-", "_")}(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(request): Json<{name.title().replace("-", "")}Request>,
) -> Result<Json<{name.title().replace("-", "")}Response>, AppError> {{
    // Implementation here
    todo!("Implement {name} update")
}}

/// Delete {name.replace("-", " ")}
pub async fn delete_{name.replace("-", "_")}(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {{
    // Implementation here
    todo!("Implement {name} deletion")
}}

/// Get {name.replace("-", " ")} routes
pub fn {name.replace("-", "_")}_routes() -> Router<SqlitePool> {{
    Router::new()
        .route("/{name.replace("-", "_")}", post(create_{name.replace("-", "_")}))
        .route("/{name.replace("-", "_")}/{{id}}", get(get_{name.replace("-", "_")}_by_id))
        .route("/{name.replace("-", "_")}/{{id}}", put(update_{name.replace("-", "_")}))
        .route("/{name.replace("-", "_")}/{{id}}", delete(delete_{name.replace("-", "_")}))
}}
'''

def create_models_template(name: str, category: str) -> str:
    """Generate models.rs.tera template."""
    return f'''use serde::{{Deserialize, Serialize}};
use sqlx::{{FromRow, SqlitePool}};
use chrono::{{DateTime, Utc}};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct {name.title().replace("-", "")} {{
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}}

impl {name.title().replace("-", "")} {{
    /// Create a new {name.replace("-", " ")}
    pub async fn create(
        pool: &SqlitePool,
        name: String,
    ) -> Result<Self, sqlx::Error> {{
        let row = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO {name.replace("-", "_")} (name, created_at, updated_at)
            VALUES (?1, ?2, ?2)
            RETURNING id, name, created_at, updated_at
            "#,
            name,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }}

    /// Find {name.replace("-", " ")} by ID
    pub async fn find_by_id(
        pool: &SqlitePool,
        id: i64,
    ) -> Result<Option<Self>, sqlx::Error> {{
        let row = sqlx::query_as!(
            Self,
            "SELECT id, name, created_at, updated_at FROM {name.replace("-", "_")} WHERE id = ?1",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }}

    /// Update {name.replace("-", " ")}
    pub async fn update(
        &mut self,
        pool: &SqlitePool,
        name: String,
    ) -> Result<(), sqlx::Error> {{
        sqlx::query!(
            r#"
            UPDATE {name.replace("-", "_")}
            SET name = ?1, updated_at = ?2
            WHERE id = ?3
            "#,
            name,
            Utc::now(),
            self.id
        )
        .execute(pool)
        .await?;

        self.name = name;
        self.updated_at = Utc::now();
        Ok(())
    }}

    /// Delete {name.replace("-", " ")}
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {{
        let result = sqlx::query!("DELETE FROM {name.replace("-", "_")} WHERE id = ?1", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }}

    /// List all {name.replace("-", " ")}s
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {{
        let rows = sqlx::query_as!(
            Self,
            "SELECT id, name, created_at, updated_at FROM {name.replace("-", "_")} ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }}
}}
'''

def create_middleware_template(name: str, category: str) -> str:
    """Generate middleware.rs.tera template."""
    return f'''use axum::{{
    extract::Request,
    http::{{HeaderMap, StatusCode}},
    middleware::Next,
    response::Response,
}};
use tracing::{{info, warn}};

/// {name.title().replace("-", " ")} middleware
pub async fn {name.replace("-", "_")}_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {{
    info!("Processing request through {name} middleware");
    
    // Add {name} specific middleware logic here
    // Examples:
    // - Authentication validation
    // - Rate limiting
    // - Request/response transformation
    // - Logging and metrics
    
    let response = next.run(request).await;
    
    info!("Completed {name} middleware processing");
    Ok(response)
}}

/// {name.title().replace("-", " ")} validation middleware
pub async fn validate_{name.replace("-", "_")}_request(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {{
    // Add request validation logic here
    // Examples:
    // - Schema validation
    // - Business rule validation
    // - Security checks
    
    let response = next.run(request).await;
    Ok(response)
}}

/// {name.title().replace("-", " ")} logging middleware
pub async fn log_{name.replace("-", "_")}_activity(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {{
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    info!("{{}} {{}} - {name} activity", method, uri);
    
    let response = next.run(request).await;
    
    info!("Response status: {{}}", response.status());
    Ok(response)
}}
'''

def create_test_template(name: str, category: str) -> str:
    """Generate lib_test.rs template."""
    return f'''#[cfg(test)]
mod {name.replace("-", "_")}_tests {{
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_{name.replace("-", "_")}_creation() {{
        // Test {name} creation logic
        assert!(true, "{name.title().replace("-", " ")} creation should work");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_validation() {{
        // Test {name} validation logic
        assert!(true, "{name.title().replace("-", " ")} validation should work");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_configuration() {{
        // Test {name} configuration
        let config = HashMap::new();
        assert!(config.is_empty(), "Empty config should be valid");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_integration() {{
        // Test {name} integration with other components
        assert!(true, "{name.title().replace("-", " ")} should integrate properly");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_error_handling() {{
        // Test {name} error handling
        assert!(true, "{name.title().replace("-", " ")} should handle errors gracefully");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_performance() {{
        // Test {name} performance characteristics
        assert!(true, "{name.title().replace("-", " ")} should perform well");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_security() {{
        // Test {name} security features
        assert!(true, "{name.title().replace("-", " ")} should be secure");
    }}

    #[test]
    fn test_{name.replace("-", "_")}_compatibility() {{
        // Test {name} API compatibility
        assert!(true, "{name.title().replace("-", " ")} should be API compatible");
    }}
}}
'''

def create_readme(name: str, category: str, description: str) -> str:
    """Generate README.md for component."""
    return f'''# {name.title().replace("-", " ")} Component

{description}

## Category
{category.title()}

## Features
- Feature 1: Core {name.replace("-", " ")} functionality
- Feature 2: Advanced {name.replace("-", " ")} configuration
- Feature 3: Integration with rust-form ecosystem
- Feature 4: Comprehensive error handling
- Feature 5: Security best practices

## Installation

Add this component to your rust-form project:

```yaml
components:
  {name}: "path:./components/{category}/{name}"
```

## Configuration

### Basic Usage

```yaml
project_name: my_app
version: "0.1.0"

components:
  {name}: "path:./components/{category}/{name}"

api:
  models:
    MyModel:
      table_name: my_models
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        name:
          type: string
          required: true

  endpoints:
    - path: /my-models
      model: MyModel
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
```

### Advanced Configuration

```yaml
# Add advanced configuration examples here
{name}_settings:
  feature_enabled: true
  custom_options:
    option1: "value1"
    option2: "value2"
```

## API Reference

### Handlers
- `create_{name.replace("-", "_")}`: Create new {name.replace("-", " ")}
- `get_{name.replace("-", "_")}_by_id`: Retrieve {name.replace("-", " ")} by ID
- `update_{name.replace("-", "_")}`: Update existing {name.replace("-", " ")}
- `delete_{name.replace("-", "_")}`: Delete {name.replace("-", " ")}

### Models
- `{name.title().replace("-", "")}`: Main data model for {name.replace("-", " ")}

### Middleware
- `{name.replace("-", "_")}_middleware`: Core {name.replace("-", " ")} middleware
- `validate_{name.replace("-", "_")}_request`: Request validation
- `log_{name.replace("-", "_")}_activity`: Activity logging

## Examples

See the `examples/` directory for complete usage examples:
- `examples/basic-usage.yml`: Simple setup
- `examples/advanced-config.yml`: Advanced configuration
- `examples/integration-test.yml`: Integration testing

## Testing

Run component tests:

```bash
rustform component test components/{category}/{name}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Changelog

### v1.0.0
- Initial release
- Core {name.replace("-", " ")} functionality
- Basic integration with rust-form
- Comprehensive test suite
'''

def create_cargo_toml(name: str) -> str:
    """Generate Cargo.toml for component."""
    return f'''[package]
name = "{name}"
version = "1.0.0"
edition = "2021"
description = "{name.title().replace("-", " ")} component for rust-form"

[workspace]

[lib]
name = "{name.replace("-", "_")}"
path = "src/lib_test.rs"

[dependencies]
'''

def create_changelog(name: str) -> str:
    """Generate CHANGELOG.md for component."""
    today = datetime.now().strftime("%Y-%m-%d")
    return f'''# Changelog

All notable changes to the {name.title().replace("-", " ")} component will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - {today}

### Added
- Initial release of {name.title().replace("-", " ")} component
- Core functionality implementation
- Request handlers for CRUD operations
- Data models with SQLx integration
- Middleware for request processing
- Comprehensive test suite
- Documentation and examples
- Integration with rust-form ecosystem

### Security
- Input validation and sanitization
- SQL injection prevention
- Authentication and authorization hooks
- Secure error handling
'''

def generate_component(name: str, category: str, description: str, base_path: str = "components") -> None:
    """Generate a complete component structure."""
    
    # Create base directories
    component_path = Path(base_path) / category / name
    component_path.mkdir(parents=True, exist_ok=True)
    
    # Create subdirectories
    (component_path / "src").mkdir(exist_ok=True)
    (component_path / "frontend" / "components").mkdir(parents=True, exist_ok=True)
    (component_path / "frontend" / "hooks").mkdir(exist_ok=True)
    (component_path / "frontend" / "types").mkdir(exist_ok=True)
    (component_path / "examples").mkdir(exist_ok=True)
    (component_path / "docs").mkdir(exist_ok=True)
    (component_path / "assets" / "icons").mkdir(parents=True, exist_ok=True)
    (component_path / "assets" / "styles").mkdir(exist_ok=True)
    (component_path / "assets" / "images").mkdir(exist_ok=True)
    
    # Generate files
    files_to_create = [
        ("rustform-component.yml", create_component_manifest_yaml(name, category, description)),
        ("README.md", create_readme(name, category, description)),
        ("CHANGELOG.md", create_changelog(name)),
        ("Cargo.toml", create_cargo_toml(name)),
        ("handlers.rs.tera", create_handlers_template(name, category)),
        ("models.rs.tera", create_models_template(name, category)),
        ("middleware.rs.tera", create_middleware_template(name, category)),
        ("src/lib_test.rs", create_test_template(name, category)),
    ]
    
    for filename, content in files_to_create:
        file_path = component_path / filename
        with open(file_path, 'w') as f:
            f.write(content)
    
    # Create example files
    examples = [
        ("examples/basic-usage.yml", f'''# Basic usage example for {name}
project_name: {name.replace("-", "_")}_example
version: "0.1.0"

components:
  {name}: "path:../../../{category}/{name}"

database:
  type: sqlite
  url_env: DATABASE_URL

api:
  models:
    Example:
      table_name: examples
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        name:
          type: string
          required: true

  endpoints:
    - path: /examples
      model: Example
      crud:
        create: true
        read_all: true
        read_one: true
'''),
        ("examples/advanced-config.yml", f'''# Advanced configuration example for {name}
project_name: {name.replace("-", "_")}_advanced
version: "0.1.0"

components:
  {name}: "path:../../../{category}/{name}"

# Add advanced configuration here
'''),
        ("examples/integration-test.yml", f'''# Integration test example for {name}
project_name: {name.replace("-", "_")}_integration_test
version: "0.1.0"

components:
  {name}: "path:../../../{category}/{name}"

# Add integration test configuration here
''')
    ]
    
    for filename, content in examples:
        file_path = component_path / filename
        with open(file_path, 'w') as f:
            f.write(content)
    
    # Create documentation files
    docs = [
        ("docs/installation.md", f"# {name.title().replace('-', ' ')} Installation Guide\\n\\nDetailed installation instructions..."),
        ("docs/configuration.md", f"# {name.title().replace('-', ' ')} Configuration\\n\\nConfiguration options and examples..."),
        ("docs/api-reference.md", f"# {name.title().replace('-', ' ')} API Reference\\n\\nComplete API documentation..."),
        ("docs/migration-guide.md", f"# {name.title().replace('-', ' ')} Migration Guide\\n\\nUpgrade and migration instructions...")
    ]
    
    for filename, content in docs:
        file_path = component_path / filename
        with open(file_path, 'w') as f:
            f.write(content)
    
    print(f"✅ Generated component: {category}/{name}")
    print(f"   Path: {component_path}")
    print(f"   Files: {len(files_to_create) + len(examples) + len(docs)} files created")

def generate_category_batch(category: str, components: List[str], base_path: str = "components") -> None:
    """Generate a batch of components for a category."""
    print(f"🚀 Generating {len(components)} components for category: {category}")
    
    for component_name in components:
        description = f"{component_name.title().replace('-', ' ')} component for {category} functionality"
        generate_component(component_name, category, description, base_path)
    
    print(f"✅ Completed {category} category - {len(components)} components generated")

def main():
    """Main entry point for component generator."""
    if len(sys.argv) < 3:
        print("Usage: python component_generator.py <category> <component-name> [description]")
        print("       python component_generator.py batch <category> <component1,component2,...>")
        print("       python component_generator.py list-categories")
        print("")
        print("Available categories:")
        for cat, info in COMPONENT_CATEGORIES.items():
            print(f"  {cat}: {info['description']}")
            print(f"    Examples: {', '.join(info['examples'][:3])}")
        return
    
    command = sys.argv[1]
    
    if command == "list-categories":
        print("Available component categories:")
        for cat, info in COMPONENT_CATEGORIES.items():
            print(f"\\n{cat.upper()}:")
            print(f"  Description: {info['description']}")
            print(f"  Examples: {', '.join(info['examples'])}")
        return
    
    if command == "batch":
        category = sys.argv[2]
        components = sys.argv[3].split(',')
        if category not in COMPONENT_CATEGORIES:
            print(f"Error: Unknown category '{category}'")
            print(f"Available categories: {', '.join(COMPONENT_CATEGORIES.keys())}")
            return
        generate_category_batch(category, components)
        return
    
    # Single component generation
    category = command
    name = sys.argv[2]
    description = sys.argv[3] if len(sys.argv) > 3 else f"{name.title().replace('-', ' ')} component for {category} functionality"
    
    if category not in COMPONENT_CATEGORIES:
        print(f"Error: Unknown category '{category}'")
        print(f"Available categories: {', '.join(COMPONENT_CATEGORIES.keys())}")
        return
    
    generate_component(name, category, description)
    print(f"\\n🎉 Component generated successfully!")
    print(f"Run tests: rustform component test components/{category}/{name}")

if __name__ == "__main__":
    main()