# Task: Project Structure Setup (inf-project-setup)

## Overview

Initialize the foundational Cargo workspace structure for Rustফর্ম with three main crates: `rustform-cli`, `rustform-codegen`, and `rustform-core`. This task establishes the project architecture and dependency management foundation.

## Requirements

### Functional Requirements
- Create a Cargo workspace with three distinct crates
- Define clear separation of concerns between crates
- Setup initial dependency structure
- Provide basic project documentation

### Technical Requirements  
- Use Rust 2021 edition
- Follow Cargo workspace best practices
- Include necessary dependencies for each crate's purpose
- Setup proper crate visibility and exports

## Implementation Notes

### Workspace Structure
```
rust-form/
├── Cargo.toml (workspace)
├── README.md
├── .gitignore  
├── rustform-cli/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
├── rustform-codegen/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── rustform-core/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### Crate Responsibilities
- **rustform-cli**: Command-line interface, user interaction, orchestration
- **rustform-codegen**: Template engine, code generation logic
- **rustform-core**: Shared types, configuration parsing, utilities

### Key Dependencies
- CLI: clap, miette, tracing
- Codegen: tera, serde_json, include_dir
- Core: serde, serde_yaml, sqlx, axum (re-exports)

## Acceptance Criteria

- [ ] Cargo workspace builds successfully with `cargo build`
- [ ] All three crates compile without errors
- [ ] Basic crate structure is in place with proper exports
- [ ] Dependencies are organized appropriately by crate
- [ ] README.md provides clear project overview
- [ ] .gitignore excludes Rust and platform-specific files

## Testing Plan

1. **Build Verification**: Run `cargo build` in workspace root
2. **Crate Independence**: Verify each crate builds individually  
3. **Dependency Resolution**: Ensure no circular dependencies
4. **Basic Functionality**: Each crate exports basic functionality

## Implementation Steps

1. Create workspace Cargo.toml with member crates
2. Initialize each crate with `cargo new --lib` (except CLI)
3. Define initial dependencies in each crate's Cargo.toml
4. Create basic module structure and exports
5. Add README.md with project description and structure
6. Setup .gitignore with Rust patterns
7. Verify everything builds and compiles

## Related Documentation

- [Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Rust Project Structure Best Practices](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- docs/kickoff/Rust-form - Architecture Overview.md

## Success Metrics

- Zero compilation errors in workspace
- Clear crate boundaries and responsibilities  
- Proper dependency organization
- Comprehensive documentation coverage