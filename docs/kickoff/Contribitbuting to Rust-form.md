# **Contributing to Rustফর্ম**

We welcome contributions of all kinds\! Whether it's reporting a bug, proposing a feature, or writing code, your help is appreciated.

### **Development Environment with Nix**

This project uses [Nix](https://nixos.org/) to provide a reproducible development environment. If you are on NixOS (like many of us\!) or have Nix installed, getting set up is as simple as running one command.

**1\. Enter the Shell:**

Navigate to the project's root directory and run:

nix develop

This command will drop you into a shell with Rust, sqlx-cli, cargo-watch, and all other necessary dependencies available in the path.

**2\. Setup the Database (First Time):**

The shell automatically exports a DATABASE\_URL. Initialize the database using sqlx-cli:

\# This will create the database file and run migrations if they exist  
sqlx database setup

**Flake Definition (flake.nix)**

Here is the Nix flake that defines the development environment. You don't need to do anything with it other than run nix develop.

\# flake.nix  
{  
  description \= "A development environment for the Rustফর্ম project";

  inputs \= {  
    nixpkgs.url \= "github:NixOS/nixpkgs/nixos-unstable";  
    flake-utils.url \= "github:numtide/flake-utils";  
    rust-overlay.url \= "github:oxalica/rust-overlay";  
  };

  outputs \= { self, nixpkgs, flake-utils, rust-overlay }:  
    flake-utils.lib.eachDefaultSystem (system:  
      let  
        overlays \= \[ (import rust-overlay) \];  
        pkgs \= import nixpkgs {  
          inherit system overlays;  
        };  
        rustToolchain \= pkgs.rust-bin.stable.latest.default.override {  
          extensions \= \[ "rust-src" \];  
        };  
      in  
      {  
        devShells.default \= pkgs.mkShell {  
          buildInputs \= \[  
            \# Rust toolchain with sources for rust-analyzer  
            rustToolchain

            \# Database tooling  
            pkgs.sqlite  
            pkgs.sqlx-cli

            \# Handy dev tools  
            pkgs.cargo-watch  
            pkgs.pkg-config  
            pkgs.openssl  
          \];

          shellHook \= ''  
            export DATABASE\_URL="sqlite:rustform\_dev.db"  
            echo "✅ Rustফর্ম dev shell activated."  
            echo "Run 'sqlx database setup' to initialize the database."  
          '';  
        };  
      });  
}

### **General Workflow**

1. **Fork the repository.**  
2. **Create a new branch:** git checkout \-b feature/my-new-feature.  
3. **Make your changes.** Please adhere to rustfmt and clippy guidelines.  
4. **Commit your changes:** git commit \-m 'feat: Add some amazing feature'.  
5. **Push to the branch:** git push origin feature/my-new-feature.  
6. **Submit a pull request.**