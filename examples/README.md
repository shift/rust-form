# Rust-form Component System Examples

This directory contains examples demonstrating the Rust-form component system.

## Overview

The component system allows you to:
- Create reusable template components
- Share components across projects
- Version and manage component dependencies
- Create complex UIs from simple building blocks

## Example Structure

```
examples/
├── components/
│   └── ui-kit/                 # Example component package
│       ├── rustform.yml        # Component manifest
│       ├── button.tera         # Button component template
│       ├── input.tera          # Input field component template
│       ├── form.tera           # Form wrapper component template
│       ├── card.tera           # Card container component template
│       └── README.md           # Component documentation
├── custom-templates/
│   └── user-form.html.tera     # Custom template using components
├── component-demo.yml          # Demo project configuration
└── README.md                   # This file
```

## UI Kit Component

The `ui-kit` component demonstrates:
- **Component manifest** (`rustform.yml`) with metadata
- **Template files** with styled HTML/CSS
- **Parameter system** for customization
- **Documentation** and usage examples

### Component Manifest

```yaml
name: "ui-kit"
version: "1.0.0"
description: "Common UI components for Rust-form applications"
author: "Rust-form Community"
license: "MIT"

templates:
  - "button.tera"
  - "input.tera" 
  - "form.tera"
  - "card.tera"

dependencies: {}

metadata:
  tags: ["ui", "components", "frontend"]
  homepage: "https://github.com/rust-form/ui-kit"
```

### Template Usage

Components are used in templates with the `component_template` function:

```tera
{% component_template component="ui-kit" template="button.tera" %}
  text="Save Changes"
  variant="primary"
  type="submit"
{% endcomponent_template %}
```

## Demo Project Configuration

The `component-demo.yml` shows how to:
- **Declare component dependencies** in the `components` section
- **Reference local components** using `path:` URIs
- **Generate projects** that use components

```yaml
components:
  ui-kit: "path:./examples/components/ui-kit"

models:
  user:
    # ... model definition
```

## Running the Example

1. **Generate the demo project:**
   ```bash
   cargo run -- generate examples/component-demo.yml --output ./demo-output
   ```

2. **Check the generated lockfile:**
   ```bash
   cat demo-output/rustform.lock
   ```

3. **View component templates** in the generated project files.

## Component URI Schemes

The component system supports various URI schemes:

```yaml
components:
  # Local components
  ui-kit: "path:./components/ui-kit"
  
  # GitHub repositories
  bootstrap: "github:rust-form/bootstrap-components@^2.0.0"
  
  # GitLab repositories  
  material: "gitlab:company/material-ui@latest"
  
  # Component registry (future)
  icons: "registry:fontawesome/icons@^5.0.0"
```

## Component Development

To create your own component:

1. **Create component directory:**
   ```bash
   mkdir my-component
   cd my-component
   ```

2. **Create manifest** (`rustform.yml`):
   ```yaml
   name: "my-component"
   version: "1.0.0"
   description: "My custom component"
   
   templates:
     - "template.tera"
   ```

3. **Create template files** with Tera syntax

4. **Test locally** by referencing with `path:` URI

5. **Publish** to Git repository for sharing

## Advanced Features

- **Version constraints**: `ui-kit: "^1.0.0"` for semantic versioning
- **Dependency resolution**: Components can depend on other components
- **Integrity verification**: SHA checksums ensure component authenticity
- **Caching**: Downloaded components are cached for performance
- **Lockfiles**: Reproducible builds with exact component versions

## Best Practices

1. **Version your components** using semantic versioning
2. **Document parameters** in component README files
3. **Keep components focused** on single responsibilities
4. **Test components** in multiple contexts
5. **Use meaningful names** for component and template files
6. **Include styling** within component templates for portability

This component system provides a powerful foundation for building reusable, maintainable web applications with Rust-form!