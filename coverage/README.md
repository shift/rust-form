# Test Coverage Configuration for Rust-form

This directory contains test coverage configuration and reporting tools.

## Coverage Tools

### 1. cargo-tarpaulin (Recommended)

Tarpaulin is a code coverage tool specifically designed for Rust.

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage/html

# Run tests with XML output for CI
cargo tarpaulin --out Xml --output-dir coverage
```

### 2. llvm-cov (Alternative)

For more detailed coverage analysis using LLVM.

```bash
# Install llvm tools
rustup component add llvm-tools-preview

# Run tests with coverage
cargo test --features coverage
RUSTFLAGS="-C instrument-coverage" cargo test --tests
```

## Coverage Targets

The project aims for the following coverage targets:

- **Overall Coverage**: > 80%
- **Core Library (`rustform-core`)**: > 85%
- **Code Generation (`rustform-codegen`)**: > 80%
- **CLI Interface (`rustform-cli`)**: > 75%

## Running Coverage Reports

### Local Development

```bash
# Quick coverage check
make coverage

# Detailed HTML report
make coverage-html

# Open coverage report in browser
make coverage-open
```

### CI/CD Pipeline

Coverage is automatically generated and reported in the CI pipeline:

1. Tests run with coverage collection
2. Coverage reports are uploaded to Codecov
3. PR comments show coverage changes
4. Coverage badges are updated

## Coverage Configuration

### Exclusions

The following code is excluded from coverage requirements:

- Generated code (in `generated/` directories)
- Test code (files ending in `_test.rs` or in `tests/` modules)
- Example code (in `examples/` directory)
- Benchmark code (in `benches/` directory)
- Error types (display implementations)
- Debug implementations

### Included Coverage

Coverage includes:

- All library code in `src/` directories
- Integration between modules
- Error handling paths
- Configuration parsing and validation
- Template rendering
- Component system

## Coverage Reports

### HTML Reports

Detailed line-by-line coverage is available in HTML format:

```
coverage/html/index.html
```

### Summary Reports

Text summaries are generated for quick review:

```
coverage/summary.txt
```

### CI Integration

Coverage data is automatically:

- Collected during test runs
- Uploaded to coverage services
- Compared against previous builds
- Reported in pull requests

## Improving Coverage

To improve coverage:

1. **Identify uncovered code**: Use HTML reports to find untested lines
2. **Add unit tests**: Focus on business logic and edge cases
3. **Add integration tests**: Test component interactions
4. **Add property tests**: Use fuzzing for input validation
5. **Test error paths**: Ensure error handling is covered

### Coverage Goals by Module

- `config/`: Configuration parsing and validation
- `component/`: Component system and manifest handling
- `codegen/`: Template engine and code generation
- `cli/`: Command-line interface and argument parsing
- `database/`: Database integration and migrations
- `error/`: Error types and handling

## Maintenance

Coverage configuration should be updated when:

- New modules are added
- Coverage targets change
- New testing tools are adopted
- CI/CD pipeline changes

## Troubleshooting

### Common Issues

1. **Low coverage on new code**: Add tests before merging
2. **Flaky coverage**: May indicate non-deterministic tests
3. **Missing coverage in CI**: Check environment setup
4. **Excluded code counted**: Review exclusion patterns

### Debug Coverage

```bash
# Verbose coverage output
cargo tarpaulin --verbose

# Include ignored tests
cargo tarpaulin --ignored

# Run specific test patterns
cargo tarpaulin --test integration_tests
```