# Templates Development Guide

Guide for creating and customizing Rust-form templates.

## 🎯 Overview

Rust-form uses an auto-discovery template system where templates are automatically loaded from directory structures.

## 📁 Template Structure

```
components/
├── frontend/
│   ├── react/              # Framework name = directory name
│   │   ├── meta.toml       # Framework metadata
│   │   ├── components/     # Component templates
│   │   │   ├── Form.tsx.tera
│   │   │   └── List.tsx.tera
│   │   └── config/
│   │       ├── package.json.tera
│   │       └── vite.config.ts.tera
│   └── vue/                # Add new frameworks here
└── backend/
    ├── basic/              # Backend variant name
    │   ├── meta.toml
    │   ├── main.rs.tera
    │   ├── models.rs.tera
    │   └── handlers.rs.tera
    └── auth/               # Add new variants here
```

## 🔧 Template Context

Templates receive rich context data:

```rust
pub struct TemplateContext {
    pub project: ProjectContext,
    pub models: Vec<ModelContext>,
    pub database: DatabaseContext,
    pub frontend: Option<FrontendContext>,
}
```

## ✨ Template Example

```rust
// models.rs.tera
{% for model in models %}
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "{{ frontend.typescript_output_dir }}/types/")]
pub struct {{ model.name | pascal_case }} {
    {% for field in model.fields %}
    pub {{ field.name }}: {{ field.rust_type }},
    {% endfor %}
}
{% endfor %}
```

## 🎨 Adding Frontend Frameworks

1. Create directory: `components/frontend/your-framework/`
2. Add `meta.toml` configuration
3. Create component templates
4. Framework automatically discovered

## 🚀 Contributing Templates

Templates are auto-discovered and can be contributed as:
- Core framework support
- Backend variants
- Component libraries
- Custom plugins

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.