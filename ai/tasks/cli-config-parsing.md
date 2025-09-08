# Task: YAML Configuration Parsing (cli-config-parsing)

## Overview

Design and implement comprehensive YAML configuration parsing with schema validation, error reporting, and type safety. This is the foundation that transforms user configuration into strongly-typed Rust data structures.

## Requirements

### Functional Requirements
- Parse YAML configuration files into typed Rust structures
- Provide comprehensive validation with helpful error messages
- Support the complete configuration schema (models, endpoints, database, middleware)
- Enable schema evolution and versioning

### Technical Requirements
- Use Serde for serialization/deserialization
- Implement custom validation logic with detailed error reporting
- Support YAML-specific features (anchors, references, multi-line strings)
- Maintain type safety throughout the parsing pipeline

## Implementation Notes

### Configuration Schema Structure
```yaml
project_name: string
version: string (semver)
database:
  type: "sqlite" | "postgres" | "mysql" 
  url_env: string
api:
  models:
    ModelName:
      table_name: string
      fields:
        field_name:
          type: "integer" | "string" | "boolean" | "datetime"
          primary_key?: boolean
          auto_increment?: boolean
          required?: boolean
          default?: any
  endpoints:
    - path: string
      model: string
      crud:
        create?: boolean
        read_all?: boolean
        read_one?: boolean
        update?: boolean
        delete?: boolean
middleware:
  - logger?: boolean
  - cors?:
      allow_origin: string
```

### Validation Strategy
- Field-level validation (required fields, type checking)
- Cross-field validation (model references, endpoint consistency)
- Business logic validation (valid table names, path formats)
- Configuration completeness validation

### Error Reporting
- Line and column numbers for YAML errors
- Contextual error messages with suggestions
- Multiple error reporting (don't stop at first error)
- Clear error categorization (syntax, validation, logic)

## Acceptance Criteria

- [ ] Valid configuration files parse successfully into typed structs
- [ ] Invalid configurations produce detailed, actionable error messages
- [ ] Schema validation catches common configuration mistakes
- [ ] Error messages include line numbers and context
- [ ] Configuration supports all required features for MVP
- [ ] Parsing performance is acceptable for typical config sizes

## Testing Plan

### Valid Configuration Testing
1. **Basic Config**: Simple Todo API configuration
2. **Complex Config**: Multi-model configuration with relationships
3. **Edge Cases**: Minimum and maximum configuration examples

### Invalid Configuration Testing
1. **Syntax Errors**: Invalid YAML syntax
2. **Type Errors**: Wrong field types
3. **Missing Fields**: Required fields omitted
4. **Logic Errors**: Invalid model references, path conflicts

### Error Message Quality
1. **Specificity**: Errors point to exact problem location
2. **Helpfulness**: Error messages suggest corrections
3. **Completeness**: Multiple errors reported in single pass

## Implementation Steps

1. Define configuration schema structs with Serde derives
2. Implement custom field validation logic
3. Create comprehensive error types with Miette integration
4. Setup YAML parsing with detailed error context
5. Implement cross-field validation rules
6. Create example configuration files
7. Write comprehensive test suite
8. Optimize error message quality and helpfulness

## Key Code Components

### Configuration Schema
```rust
#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    pub project_name: String,
    pub version: String,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    #[serde(default)]
    pub middleware: Vec<MiddlewareConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub table_name: String,
    pub fields: IndexMap<String, FieldConfig>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_yaml::Value>,
}
```

### Validation Implementation
```rust
impl Validate for Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        // Validate model references in endpoints
        // Check for duplicate table names
        // Verify endpoint path formats
        // Validate middleware configuration
    }
}
```

### Error Context
```rust
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ConfigError {
    #[error("YAML syntax error")]
    YamlSyntax(#[from] serde_yaml::Error),
    
    #[error("Configuration validation failed")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("Model '{model}' referenced in endpoint but not defined")]
    ModelNotFound { model: String },
}
```

## Related Documentation

- YAML specification and best practices
- Serde documentation for custom deserializers
- Validator crate for validation patterns
- Miette for error reporting patterns

## Success Metrics

- Configuration parsing is intuitive and forgiving
- Error messages guide users to quick resolution
- Schema supports all MVP requirements
- Validation catches errors early in development process
- Performance is suitable for interactive CLI usage