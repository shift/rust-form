# Component Testing Standards

This document defines the testing standards and requirements for rust-form components to ensure quality, reliability, and compatibility.

## Overview

All rust-form components must meet specific testing standards to be considered production-ready. The `rustform component test` command validates components against these standards and provides quality metrics.

## Testing Phases

### Phase 1: Manifest Validation
- **Requirement**: All components MUST have a valid `rustform-component.yml` manifest
- **Validation**: Schema validation, required fields, semantic version format
- **Criteria**: Manifest passes all validation rules

### Phase 2: Compatibility Check
- **Requirement**: Components MUST be compatible with the target rust-form version
- **Validation**: API version compatibility, feature requirements, experimental API usage
- **Criteria**: Component compatibility status is `Compatible` or `CompatibleExperimental`

### Phase 3: Unit Tests
- **Requirement**: Components SHOULD include comprehensive unit tests
- **Validation**: Test discovery, execution, and results parsing
- **Test File Patterns**:
  - Rust: `*_test.rs`, `test_*.rs` in `tests/` or `src/`
  - JavaScript/TypeScript: `*.test.js`, `*.test.ts`
- **Criteria**: All unit tests pass, minimum 80% test coverage recommended

### Phase 4: Integration Testing
- **Requirement**: Components MUST work within a generated rust-form application
- **Validation**: Test application generation, compilation verification
- **Criteria**: Generated test application compiles without errors

### Phase 5: Quality Assessment
- **Requirement**: Components SHOULD meet quality standards for production use
- **Validation**: Documentation, examples, test coverage, functionality completeness
- **Criteria**: Quality score ≥ 70/100 for production readiness

## Quality Metrics

### Scoring System (0-100 points)

#### Manifest Completeness (25 points max)
- Description provided: +10 points
- Author information: +5 points
- License specified: +5 points
- Repository URL: +5 points

#### Documentation (20 points max)
- README.md exists: +20 points
- Additional documentation: +bonus

#### Examples (15 points max)
- Example directory/files exist: +15 points
- Working examples: +bonus

#### Testing (25 points max)
- Unit tests present: +25 points
- High test coverage: +bonus

#### Functionality (15 points max)
- Templates provided: +2 points each
- Assets provided: +1 point each
- Hooks provided: +3 points each

### Quality Grades
- **A+ (90-100)**: Excellent - Production ready
- **A (80-89)**: Very Good - Ready with minor improvements
- **B (70-79)**: Good - Acceptable for production
- **C (60-69)**: Fair - Needs improvement before production
- **D (50-59)**: Poor - Significant issues to address
- **F (<50)**: Needs Improvement - Not ready for use

## Component Requirements

### Mandatory Requirements
1. **Valid Manifest**: Must pass schema validation
2. **Compatibility**: Must be compatible with target rust-form version
3. **Compilation**: Generated test application must compile

### Recommended Requirements
1. **Documentation**: README.md with usage instructions
2. **Tests**: Unit tests with ≥80% coverage
3. **Examples**: Working usage examples
4. **Metadata**: Author, license, repository information

### Best Practices
1. **Semantic Versioning**: Follow semver for component versions
2. **API Stability**: Avoid breaking changes in patch versions
3. **Error Handling**: Graceful error handling and meaningful messages
4. **Performance**: Efficient template rendering and minimal resource usage
5. **Security**: Input validation and secure defaults

## Testing Commands

### Basic Testing
```bash
rustform component test <component-name>
```

### Test Options
```bash
# Generate test application for validation
rustform component test <component-name> --generate-test-app

# Run unit tests only
rustform component test <component-name> --unit-tests-only

# Skip compatibility check
rustform component test <component-name> --skip-compatibility

# Specify components directory
rustform component test <component-name> --directory ./my-components
```

### Test Output
The test command provides:
- Phase-by-phase validation results
- Test execution summary
- Quality assessment with score and grade
- Improvement suggestions for low-scoring components

## Continuous Integration

### CI Pipeline Integration
Components should integrate testing into their CI pipelines:

```yaml
# .github/workflows/test.yml
name: Component Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install rust-form
        run: cargo install rustform-cli
      - name: Test component
        run: rustform component test . --generate-test-app
```

### Quality Gates
- All tests must pass
- Quality score ≥ 70 for merging
- No compatibility issues with supported rust-form versions

## Component Structure

### Recommended Directory Structure
```
my-component/
├── rustform-component.yml     # Component manifest
├── README.md                  # Documentation
├── templates/                 # Template files
│   ├── model.rs.tera
│   └── handler.rs.tera
├── assets/                    # Static assets
│   └── styles.css
├── examples/                  # Usage examples
│   └── basic-usage.yml
├── tests/                     # Unit tests
│   ├── template_tests.rs
│   └── integration_tests.rs
└── src/                       # Source code (if applicable)
    └── lib.rs
```

### Test Files Location
- Rust tests: `tests/` directory or `src/` with `#[cfg(test)]`
- JavaScript tests: `tests/` or alongside source files
- Integration tests: `tests/integration/`

## Error Handling

### Test Failures
When tests fail, the component test command will:
1. Report specific failure reasons
2. Provide debugging information
3. Suggest remediation steps
4. Exit with non-zero status code

### Common Issues
1. **Manifest Validation**: Schema errors, missing required fields
2. **Compatibility**: Version mismatches, missing features
3. **Unit Tests**: Test failures, missing test files
4. **Integration**: Compilation errors in generated application
5. **Quality**: Missing documentation, low test coverage

## Compliance

### Production Readiness
For components to be considered production-ready:
- Quality score ≥ 70/100
- All mandatory requirements met
- No failing tests
- Compatible with supported rust-form versions

### Security Requirements
- Input validation in templates
- No hardcoded secrets or credentials
- Secure defaults for all configurations
- XSS prevention in web templates

## Future Enhancements

### Planned Features
1. **Performance Testing**: Benchmark template rendering
2. **Security Scanning**: Automated vulnerability detection
3. **Coverage Reporting**: Detailed test coverage metrics
4. **Integration with Registry**: Automated testing for published components
5. **Multi-Version Testing**: Test against multiple rust-form versions

This testing framework ensures that rust-form components meet high standards for quality, reliability, and compatibility, providing users with confidence in component stability and functionality.