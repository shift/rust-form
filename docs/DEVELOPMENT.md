# Development Environment Setup

## Prerequisites

This project uses [Nix](https://nixos.org/) for reproducible development environments. You have two options:

### Option 1: Using Nix (Recommended)

1. **Install Nix** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
   ```

2. **Enter the development shell**:
   ```bash
   nix develop
   ```

3. **Or use direnv for automatic loading** (recommended):
   ```bash
   # Install direnv if not available
   # On NixOS: already available
   # On other systems: follow https://direnv.net/docs/installation.html
   
   # Allow direnv to load the environment
   direnv allow
   
   # Environment will auto-load when you cd into the project
   ```

### Option 2: Manual Setup

If you prefer not to use Nix, install these dependencies manually:

1. **Rust toolchain** (1.70+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup component add clippy rustfmt rust-src
   ```

2. **SQLite and development libraries**:
   ```bash
   # Ubuntu/Debian
   sudo apt install sqlite3 libsqlite3-dev pkg-config libssl-dev
   
   # Fedora/RHEL
   sudo dnf install sqlite-devel openssl-devel pkg-config
   
   # macOS
   brew install sqlite openssl pkg-config
   ```

3. **SQLx CLI**:
   ```bash
   cargo install sqlx-cli --no-default-features --features sqlite
   ```

4. **Development tools**:
   ```bash
   cargo install cargo-watch cargo-edit cargo-audit
   ```

## Development Workflow

### Environment Variables

The development environment sets these variables:
- `DATABASE_URL="sqlite:rustform_dev.db"` - SQLite database for development
- `RUST_LOG="debug"` - Enable debug logging

### Available Commands

```bash
# Build the workspace
cargo build

# Run tests
cargo test --workspace

# Check code without building
cargo check

# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features

# Watch for changes and rebuild
cargo watch -x check

# Database operations
sqlx database create
sqlx database drop
sqlx migrate run
```

### Project Structure

```
rust-form/
├── rustform-cli/           # CLI interface crate
├── rustform-codegen/       # Code generation engine crate  
├── rustform-core/          # Shared types and utilities crate
├── templates/              # Code generation templates
├── examples/               # Example configurations
├── docs/                   # Documentation
└── ai/                     # AI task planning (development)
```

### Development Tools Included

**Nix Development Shell includes**:
- Rust stable toolchain with clippy, rustfmt, rust-src
- SQLite database and CLI tools
- SQLx CLI for database migrations
- cargo-watch for continuous compilation
- cargo-edit for dependency management
- cargo-audit for security auditing
- pkg-config and OpenSSL for native dependencies
- Git and GitHub CLI for development workflow

### First Time Setup

1. **Clone and enter environment**:
   ```bash
   git clone <repository-url>
   cd rust-form
   nix develop  # or direnv allow
   ```

2. **Initialize database**:
   ```bash
   sqlx database create
   ```

3. **Build and test**:
   ```bash
   cargo build
   cargo test --workspace
   ```

4. **Start development**:
   ```bash
   cargo watch -x "check --workspace"
   ```

## Troubleshooting

### Nix Issues

- **Flakes not enabled**: Add `experimental-features = nix-command flakes` to `/etc/nix/nix.conf`
- **Permission issues**: Ensure you're in the `nix-users` group
- **Slow downloads**: Consider using a Nix binary cache

### Build Issues

- **SQLite errors**: Ensure SQLite development libraries are installed
- **OpenSSL errors**: Install OpenSSL development packages
- **Linking errors**: Install pkg-config

### Database Issues

- **Connection failed**: Check `DATABASE_URL` environment variable
- **Migration errors**: Ensure database exists with `sqlx database create`