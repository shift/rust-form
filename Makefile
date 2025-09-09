# Test and Coverage Commands

.PHONY: test test-unit test-integration test-e2e coverage coverage-html coverage-open clean-coverage

# Default test command
test:
	cargo test --workspace --all-features

# Unit tests only
test-unit:
	cargo test --workspace --lib --all-features

# Integration tests only
test-integration:
	cargo test --workspace --test integration_tests --all-features

# End-to-end tests only  
test-e2e:
	cargo test --workspace --test e2e_tests --all-features

# Property tests (longer running)
test-property:
	cargo test --workspace --all-features property_tests

# Cucumber tests
test-cucumber:
	cargo test --workspace --test cucumber_tests --all-features

# Performance benchmarks
bench:
	cargo bench --workspace --all-features

# Coverage with tarpaulin (requires cargo-tarpaulin)
coverage:
	cargo tarpaulin --config tarpaulin.toml

# Coverage with HTML output
coverage-html:
	cargo tarpaulin --config tarpaulin.toml --out Html --output-dir coverage/html

# Coverage for CI (XML format)
coverage-ci:
	cargo tarpaulin --config tarpaulin.toml --out Xml --output-dir coverage

# Open coverage report in browser
coverage-open: coverage-html
	@if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open coverage/html/index.html; \
	elif command -v open >/dev/null 2>&1; then \
		open coverage/html/index.html; \
	else \
		echo "Coverage report generated at coverage/html/index.html"; \
	fi

# Clean coverage data
clean-coverage:
	rm -rf coverage/
	cargo clean

# Run all tests with coverage
test-all-coverage: clean-coverage
	cargo tarpaulin --config tarpaulin.toml --tests --benches

# Fast tests (exclude slow property tests)
test-fast:
	cargo test --workspace --all-features --exclude-ignored

# Test specific crate
test-core:
	cargo test -p rustform-core --all-features

test-codegen:
	cargo test -p rustform-codegen --all-features

test-cli:
	cargo test -p rustform-cli --all-features

# Clippy linting
lint:
	cargo clippy --workspace --all-features -- -D warnings

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run all quality checks
check-all: fmt-check lint test coverage

# Install development dependencies
install-deps:
	cargo install cargo-tarpaulin
	cargo install cargo-audit
	cargo install cargo-deny

# Security audit
audit:
	cargo audit
	cargo deny check

# Generate documentation
docs:
	cargo doc --workspace --all-features --no-deps --open

# Run tests in watch mode (requires cargo-watch)
test-watch:
	cargo watch -x "test --workspace --all-features"

# Clean everything
clean-all: clean-coverage
	cargo clean
	rm -rf target/

# Help command
help:
	@echo "Available commands:"
	@echo "  test              - Run all tests"
	@echo "  test-unit         - Run unit tests only"
	@echo "  test-integration  - Run integration tests only"
	@echo "  test-e2e          - Run end-to-end tests only"
	@echo "  coverage          - Generate coverage report"
	@echo "  coverage-html     - Generate HTML coverage report"
	@echo "  coverage-open     - Open coverage report in browser"
	@echo "  lint              - Run clippy lints"
	@echo "  fmt               - Format code"
	@echo "  check-all         - Run all quality checks"
	@echo "  install-deps      - Install development dependencies"
	@echo "  docs              - Generate and open documentation"
	@echo "  clean-all         - Clean all build artifacts"