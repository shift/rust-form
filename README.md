# Rustফর্ম

**Declarative, Type-Safe Web Backends in Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

> **⚠️ Work in Progress**: This project is under active development. APIs may change.

## Overview

Rustফর্ম is a command-line tool that generates high-performance, memory-safe web backends from simple YAML configuration files. Inspired by [ESPHome](https://esphome.io/)'s declarative philosophy and built on Rust's robust ecosystem.

**Motto:** Define the *what*, not the *how*.

## Quick Start

```yaml
# config.yml
project_name: todo_api
version: "0.1.0"

database:
  type: sqlite
  url_env: DATABASE_URL

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
        completed:
          type: boolean
          default: false

  endpoints:
    - path: /todos
      model: Todo
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true

middleware:
  - logger: true
  - cors:
      allow_origin: "*"
```

```bash
# Generate your backend
rustform generate config.yml

# Build and run
cd todo_api
cargo build --release
./target/release/todo_api
```

## Features

- **🚀 High Performance**: Built on Axum and Tokio
- **🔒 Type Safety**: Compile-time checked database queries with SQLx
- **📝 Declarative**: Focus on business logic, not boilerplate
- **🔧 Extensible**: Plugin system for custom logic (planned)
- **🌐 Production Ready**: Memory-safe, async-first architecture

## Development Status

This project is in **MVP development phase**. See our [Project Roadmap](docs/kickoff/Rust-form%20Project%20Roadmap.md) for planned features.

### Current Progress
- [ ] CLI Foundation
- [ ] YAML Configuration Parsing  
- [ ] Code Generation Engine
- [ ] Database Integration (SQLite)
- [ ] CRUD API Generation
- [ ] Basic Middleware Support

## Development

### Prerequisites

- [Nix](https://nixos.org/) (recommended)
- Rust 1.70+ (if not using Nix)

### Setup

```bash
# Clone the repository
git clone https://github.com/your-org/rust-form.git
cd rust-form

# Enter development environment (Nix)
nix develop

# Or install dependencies manually
# See docs/kickoff/Contributing\ to\ Rust-form.md
```

### Contributing

We welcome contributions! Please see [Contributing Guide](docs/kickoff/Contribitbuting%20to%20Rust-form.md) for details.

## Architecture

Rustফর্ম consists of three main components:

- **rustform-cli**: Command-line interface and orchestration
- **rustform-codegen**: Template engine and code generation  
- **rustform-core**: Shared types, configuration, and utilities

See [Architecture Overview](docs/kickoff/Rust-form%20-%20Architecture%20Overview.md) for detailed design.

## License

Licensed under the [MIT License](LICENSE).

## Roadmap

- **Phase 1 (MVP)**: Basic CRUD generation with SQLite
- **Phase 2**: PostgreSQL support, relationships, validation
- **Phase 3**: Authentication, testing, deployment helpers

See [detailed roadmap](docs/kickoff/Rust-form%20Project%20Roadmap.md) for more information.