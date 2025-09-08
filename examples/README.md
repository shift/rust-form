# Examples and Tutorials

This directory contains comprehensive examples and step-by-step tutorials for Rust-form.

## 📚 Quick Start Examples

### [todo.yml](../todo.yml)
Simple todo application with CRUD operations.
```bash
rustform generate examples/todo.yml
```

### [blog.yml](../blog.yml)  
Multi-model blog with users, categories, and posts.
```bash
rustform generate examples/blog.yml
```

### [ecommerce.yml](../ecommerce.yml)
Basic e-commerce platform with products and orders.
```bash
rustform generate examples/ecommerce.yml
```

## 🎯 Tutorials

### [tutorials/](./tutorials/)
Step-by-step guides for building real applications:

1. **[Getting Started](./tutorials/01-getting-started.md)** - Your first Rust-form app
2. **[Blog Platform](./tutorials/02-blog-platform.md)** - Multi-model relationships
3. **[E-commerce API](./tutorials/03-ecommerce-api.md)** - Complex business logic
4. **[Frontend Integration](./tutorials/04-frontend-integration.md)** - React/Vue/Svelte
5. **[Production Deployment](./tutorials/05-production-deployment.md)** - Going live

## 🔮 Advanced Examples

### [future_configs/](../tests/future_configs/)
Advanced configurations demonstrating future features:

- **[ecommerce-advanced.yml](../tests/future_configs/ecommerce-advanced.yml)** - Full e-commerce platform
- **[social-media.yml](../tests/future_configs/social-media.yml)** - Social networking platform  
- **[cms-platform.yml](../tests/future_configs/cms-platform.yml)** - Enterprise CMS
- **[auth-service.yml](../tests/future_configs/auth-service.yml)** - Authentication microservice
- **[inventory-management.yml](../tests/future_configs/inventory-management.yml)** - Inventory system

## 🚀 Usage

Each example can be generated and run:

```bash
# Generate any example
rustform generate examples/[example-name].yml

# Run the generated backend
cd [project-name]/backend
cargo run

# Run the generated frontend (if configured)
cd [project-name]/frontend
npm install && npm run dev
```

## 🤝 Contributing Examples

To contribute a new example:

1. Create a new `.yml` configuration file
2. Test generation and compilation
3. Add documentation explaining the use case
4. Submit a pull request

Examples should demonstrate:
- Real-world use cases
- Best practices
- Different features and patterns
- Progressive complexity

Happy coding with Rust-form! 🦀✨