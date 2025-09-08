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
          extensions = [ "rust-src" "clippy" "rustfmt" ];
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
            pkgs.pkg-config
            pkgs.openssl

            # Git and development utilities
            pkgs.git
            pkgs.gh
          ];

          shellHook = ''
            export DATABASE_URL="sqlite:rustform_dev.db"
            export RUST_LOG="debug"
            echo "✅ Rustফর্ম dev shell activated."
            echo "   Database URL: $DATABASE_URL"
            echo "   Run 'sqlx database setup' to initialize the database."
            echo "   Run 'cargo watch -x check' for continuous compilation."
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
          rustform-clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "rustform-clippy";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config rustToolchain ];
            buildInputs = [ pkgs.openssl pkgs.sqlite ];
            buildPhase = ''
              cargo clippy --all-targets --all-features -- -D warnings
              touch $out
            '';
            installPhase = "mkdir -p $out";
          };

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
        };
      });
}