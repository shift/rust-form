{
  description = "A development environment for the Rustফর্ম project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" "llvm-tools-preview" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain with sources for rust-analyzer
            rustToolchain

            # Database tooling
            pkgs.sqlite
            pkgs.sqlx-cli

            # Development tools
            pkgs.cargo-watch
            pkgs.cargo-edit
            pkgs.cargo-audit
            pkgs.cargo-deny
            pkgs.pkg-config
            pkgs.openssl

            # Testing tools
            pkgs.gnumake

            # Git and development utilities
            pkgs.git
            pkgs.gh
          ];

          shellHook = ''
            export DATABASE_URL="sqlite:rustform_dev.db"
            export RUST_LOG="debug"
            
            # Install cargo tools if not already installed
            # if ! command -v cargo-tarpaulin &> /dev/null; then
            #   echo "📦 Installing cargo-tarpaulin for coverage..."
            #   cargo install cargo-tarpaulin
            # fi
            
            # if ! command -v cargo-watch &> /dev/null; then
            #   echo "📦 Installing cargo-watch..."
            #   cargo install cargo-watch
            # fi
            
            echo "✅ Rustফর্ম dev shell activated."
            echo "   Database URL: $DATABASE_URL"
            echo ""
            echo "🧪 Testing Commands:"
            echo "   make test              - Run all tests"
            echo "   make test-unit         - Run unit tests only"
            echo "   make test-integration  - Run integration tests"
            echo "   make test-e2e          - Run end-to-end tests"
            echo "   make coverage          - Generate coverage report"
            echo "   make coverage-html     - Generate HTML coverage report"
            echo "   make coverage-open     - Open coverage in browser"
            echo ""
            echo "🏗️  Build Commands:"
            echo "   nix build              - Build the project"
            echo "   nix flake check        - Run all checks"
            echo "   make lint              - Run clippy lints"
            echo "   make fmt               - Format code"
            echo ""
            echo "🗃️  Database Commands:"
            echo "   sqlx database setup    - Initialize the database"
            echo "   sqlx migrate run       - Run pending migrations"
            echo ""
            echo "⚡ Development Commands:"
            echo "   cargo watch -x check   - Continuous compilation"
            echo "   make test-watch        - Continuous testing"
            echo "   make help              - Show all available commands"
          '';
        };

        # Build outputs for CI
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rustform";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl pkgs.sqlite ];
        };

        # Checks for CI
        checks = {
          # Clippy linting check
          rustform-clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-clippy";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config rustToolchain ];
            buildInputs = [ pkgs.openssl pkgs.sqlite ];
            buildPhase = ''
              cargo clippy --workspace --all-targets --all-features -- -D warnings
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

          # Formatting check
          rustform-fmt = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-fmt";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ rustToolchain ];
            buildPhase = ''
              cargo fmt --all -- --check
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

          # Unit tests check
          rustform-test-unit = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-test-unit";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config rustToolchain ];
            buildInputs = [ pkgs.openssl pkgs.sqlite ];
            buildPhase = ''
              export DATABASE_URL="sqlite::memory:"
              cargo test --workspace --lib --all-features
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

          # Integration tests check
          rustform-test-integration = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-test-integration";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config rustToolchain ];
            buildInputs = [ pkgs.openssl pkgs.sqlite ];
            buildPhase = ''
              export DATABASE_URL="sqlite::memory:"
              cargo test --workspace --test integration_tests --all-features
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

          # Documentation build check
          rustform-docs = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-docs";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config rustToolchain ];
            buildInputs = [ pkgs.openssl pkgs.sqlite ];
            buildPhase = ''
              cargo doc --workspace --all-features --no-deps
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

          # Security audit check
          rustform-audit = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-audit";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ rustToolchain pkgs.cargo-audit pkgs.cargo-deny ];
            buildPhase = ''
              cargo audit
              cargo deny check
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };
        };

        # Development apps for easy access
        apps = {
          # Run all tests
          test = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-test" ''
              export DATABASE_URL="sqlite::memory:"
              exec ${rustToolchain}/bin/cargo test --workspace --all-features "$@"
            '';
          };

          # Run unit tests only
          test-unit = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-test-unit" ''
              export DATABASE_URL="sqlite::memory:"
              exec ${rustToolchain}/bin/cargo test --workspace --lib --all-features "$@"
            '';
          };

          # Run integration tests
          test-integration = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-test-integration" ''
              export DATABASE_URL="sqlite::memory:"
              exec ${rustToolchain}/bin/cargo test --workspace --test integration_tests --all-features "$@"
            '';
          };

          # Run e2e tests
          test-e2e = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-test-e2e" ''
              export DATABASE_URL="sqlite::memory:"
              exec ${rustToolchain}/bin/cargo test --workspace --test e2e_tests --all-features "$@"
            '';
          };

          # Generate coverage report
          coverage = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-coverage" ''
              if ! command -v cargo-tarpaulin &> /dev/null; then
                echo "Installing cargo-tarpaulin..."
                ${rustToolchain}/bin/cargo install cargo-tarpaulin
              fi
              export DATABASE_URL="sqlite::memory:"
              exec ${rustToolchain}/bin/cargo tarpaulin --config tarpaulin.toml "$@"
            '';
          };

          # Lint with clippy
          lint = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-lint" ''
              exec ${rustToolchain}/bin/cargo clippy --workspace --all-targets --all-features -- -D warnings "$@"
            '';
          };

          # Format code
          fmt = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-fmt" ''
              exec ${rustToolchain}/bin/cargo fmt --all "$@"
            '';
          };

          # Build docs
          docs = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rustform-docs" ''
              exec ${rustToolchain}/bin/cargo doc --workspace --all-features --no-deps --open "$@"
            '';
          };
        };
      });
}