## Contributing to Rustফর্ম

Thank you for your interest in contributing to Rustফর্ম! This document provides guidelines and information for contributors.

### 🚀 Quick Start

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/rust-form.git
   cd rust-form
   ```

2. **Set up development environment**
   ```bash
   # Using Nix (recommended)
   nix develop
   
   # Or install Rust manually
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Run tests**
   ```bash
   cargo test --workspace
   ```

### 🏗️ Development Workflow

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   cargo test --workspace
   cargo clippy --all-targets --all-features
   cargo fmt --all
   ```

4. **Commit and push**
   ```bash
   git commit -m "feat: add your feature description"
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request**

### 📋 Development Guidelines

#### Code Style
- Use `cargo fmt` to format your code
- Run `cargo clippy` and address all warnings
- Follow Rust naming conventions
- Write clear, self-documenting code

#### Testing
- Add unit tests for new functions
- Add integration tests for new features
- Ensure generated code compiles and runs
- Test with example configurations

#### Documentation
- Update README.md if adding user-facing features
- Add rustdoc comments for public APIs
- Update architecture docs for significant changes

### 🎯 Areas for Contribution

#### 🐛 Bug Fixes
- Check the [issues](https://github.com/your-org/rust-form/issues) for bugs
- Look for issues labeled `good first issue`

#### 🆕 New Features
- Database support (PostgreSQL, MySQL)
- Additional field types and validations
- Middleware implementations
- Template improvements

#### 📚 Documentation
- API documentation
- Usage examples
- Tutorial content
- Architecture guides

#### 🧪 Testing
- Increase test coverage
- Add integration tests
- Performance benchmarks

### 📝 Pull Request Process

1. **Before submitting:**
   - Ensure your PR has a clear description
   - Link to related issues
   - Include tests for new functionality
   - Update documentation if needed

2. **PR Requirements:**
   - All CI checks must pass
   - Code coverage should not decrease
   - Generated code must compile and run
   - Follow the PR template

3. **Review process:**
   - Maintainers will review your PR
   - Address feedback promptly
   - Be open to suggestions and changes

### 🏷️ Commit Message Format

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

### 🐛 Reporting Bugs

1. **Check existing issues** first
2. **Use the bug report template**
3. **Provide minimal reproduction case**
4. **Include environment details**

### 💡 Suggesting Features

1. **Check existing feature requests**
2. **Use the feature request template**
3. **Explain the use case clearly**
4. **Consider implementation complexity**

### 🔄 Development Environment

#### With Nix (Recommended)
```bash
nix develop
# All tools and dependencies are available
```

#### Manual Setup
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

### 🧪 Testing Strategy

#### Unit Tests
```bash
cargo test --lib
```

#### Integration Tests
```bash
cargo test --test integration
```

#### Generated Code Testing
```bash
# Test code generation
cargo run -- generate examples/todo.yml --output test-output
cd test-output
cargo build
cargo test
```

### 📊 Project Structure

```
rust-form/
├── rustform-cli/           # CLI interface
├── rustform-codegen/       # Code generation engine
├── rustform-core/          # Shared types and utilities
├── templates/              # Code generation templates
├── examples/               # Example configurations
├── docs/                   # Documentation
└── ai/                     # AI task planning (development)
```

### ❓ Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs and request features
- **Documentation**: Check docs/ directory
- **Examples**: See examples/ directory

### 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Rustফর্ম! 🦀✨