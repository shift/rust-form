# Task: CLI Foundation with Clap (cli-foundation)

## Overview

Establish the core CLI structure using Clap for command-line argument parsing and Miette for beautiful error reporting. This provides the foundation for all user interactions with the Rustফর্ম tool.

## Requirements

### Functional Requirements
- Implement basic CLI with subcommands (generate, init, etc.)
- Provide helpful error messages and user guidance
- Support standard CLI patterns (--help, --version)
- Enable structured logging and diagnostics

### Technical Requirements
- Use Clap v4 with derive API for type safety
- Integrate Miette for error reporting with context
- Setup Tracing for structured logging
- Follow CLI best practices and conventions

## Implementation Notes

### Command Structure
```
rustform
├── generate <config.yml> [--output <dir>]
├── init [--name <project>] 
├── --version
└── --help
```

### Error Handling Strategy
- Use Miette for rich error diagnostics
- Provide helpful suggestions for common mistakes
- Include relevant context (file paths, line numbers)
- Maintain consistent error formatting

### Logging Configuration
- Use tracing for structured logging
- Support different log levels (debug, info, warn, error)
- Enable optional verbose output mode

## Acceptance Criteria

- [ ] `rustform --help` displays comprehensive usage information
- [ ] `rustform --version` shows correct version number
- [ ] Commands parse arguments correctly with type safety
- [ ] Error messages are user-friendly and actionable
- [ ] Miette integration provides rich error context
- [ ] Logging works properly with configurable levels

## Testing Plan

1. **Command Parsing**: Verify all commands accept correct arguments
2. **Help System**: Ensure help messages are comprehensive and clear
3. **Error Handling**: Test invalid arguments produce helpful errors
4. **Version Display**: Confirm version information is accurate
5. **Logging**: Verify log output at different levels

## Implementation Steps

1. Setup Clap with derive macros for CLI structure
2. Define command enums and argument structs
3. Implement Miette error types and conversions
4. Setup tracing subscriber configuration
5. Create command handler stubs
6. Add comprehensive help documentation
7. Implement version information display
8. Test all CLI interactions

## Key Code Components

### Main CLI Structure
```rust
#[derive(Parser)]
#[command(name = "rustform")]
#[command(about = "Declarative, Type-Safe Web Backends in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate(GenerateCommand),
    Init(InitCommand),
}
```

### Error Handling
```rust
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum CliError {
    #[error("Configuration file not found")]
    ConfigNotFound(#[from] std::io::Error),
    
    #[error("Invalid configuration")]
    InvalidConfig(#[from] ConfigError),
}
```

## Related Documentation

- [Clap Documentation](https://docs.rs/clap/)
- [Miette Error Reporting](https://docs.rs/miette/)
- [Tracing Documentation](https://docs.rs/tracing/)

## Success Metrics

- CLI provides intuitive user experience
- Error messages guide users to solutions
- Help system is comprehensive and useful
- All standard CLI conventions are followed