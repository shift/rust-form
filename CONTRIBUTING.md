# Contributing to Rust-form

We welcome contributions to Rust-form! This guide will help you get started.

## 🚀 Quick Start

```bash
git clone https://github.com/rust-form/rust-form.git
cd rust-form

# Use Nix for development environment (recommended)
nix develop

# Or install dependencies manually
cargo install --path rustform-cli
```

## 🎯 Ways to Contribute

### 🐛 Bug Reports
- Use [GitHub Issues](https://github.com/rust-form/rust-form/issues)
- Include minimal reproduction case
- Specify Rust-form version and platform

### ✨ Feature Requests
- Check [existing issues](https://github.com/rust-form/rust-form/issues)
- Describe use case and motivation
- Consider contributing implementation

### 🔧 Code Contributions

#### High-Impact Areas
1. **Frontend Frameworks** - Add Vue, Svelte, Angular support
2. **Template Components** - New component types and patterns
3. **Validation** - Advanced validation rules and integration
4. **Database Support** - Additional database adapters
5. **Documentation** - Examples, guides, and API docs

#### Development Process
1. **Fork** the repository
2. **Create feature branch** from `main`
3. **Write tests** for new functionality
4. **Update documentation** if needed
5. **Submit pull request** with clear description

### 📚 Documentation
- Improve existing guides
- Add examples and tutorials
- Write API documentation
- Create video tutorials

### 🎨 Templates
- Add new frontend framework support
- Create backend variants (auth, GraphQL, etc.)
- Develop component libraries
- Optimize generated code

## 🏗️ Architecture

```
rust-form/
├── rustform-cli/          # CLI interface
├── rustform-codegen/      # Template engine
├── rustform-core/         # Core types
└── components/            # Auto-discovered templates
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p rustform-core

# Integration tests
cargo test --test integration

# Generate test projects
rustform generate examples/todo.yml
cd todo_app && cargo check
```

## 📝 Code Style

- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Documentation**: Use `///` doc comments
- **Error Handling**: Use `miette` for user-facing errors

## 🎯 Priority Areas

Current development priorities (see [tasks.json](ai/tasks.json)):

1. **Testing Framework** - Comprehensive test suite
2. **Enhanced CRUD** - Advanced SQL generation
3. **Validation Integration** - Field validation rules
4. **Relationship Handling** - Foreign keys and joins
5. **Frontend Frameworks** - Vue and Svelte support

## 🏷️ Commit Message Format

We follow conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting, no code change
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
```
feat(cli): add init command for project scaffolding
fix(codegen): handle empty model definitions correctly
docs(readme): update installation instructions
```

## 🔄 Development Environment

### With Nix (Recommended)
```bash
nix develop
# All tools and dependencies are available
```

### Manual Setup
Requirements:
- Rust 1.70+
- SQLite development libraries
- pkg-config
- OpenSSL development libraries

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Setup database
export DATABASE_URL="sqlite:rustform_dev.db"
sqlx database setup
```

## 🧪 Testing Strategy

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test integration
```

### Generated Code Testing
```bash
# Test code generation
rustform generate examples/todo.yml --output test-output
cd test-output && cargo build && cargo test
```

## 🌟 Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Project documentation
- Community Discord

## 📞 Getting Help

- **Discord**: [Join our community](https://discord.gg/rust-form)
- **GitHub Discussions**: For design discussions
- **Issues**: For bugs and feature requests

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for making Rust-form better!** 🦀✨