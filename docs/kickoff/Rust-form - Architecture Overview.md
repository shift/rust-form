# **Architecture Overview**

This document outlines the proposed technical architecture for Rustফর্ম.

### **Guiding Principles**

1. **Minimize Magic:** The generated code should be idiomatic, readable, and easy to understand. A developer should be able to "eject" and work with the generated code directly if needed.  
2. **Leverage the Ecosystem:** Use battle-tested, popular crates to provide the core functionality. This avoids reinventing the wheel and benefits from the security and performance work of the wider Rust community.  
3. **Compile-Time Confidence:** Push as many checks as possible to compile-time (e.g., SQL query validation via sqlx::query\!, type safety via Rust's type system).

### **Core Components**

The project is split into three main parts: the CLI, the Code Generator, and a small Runtime Crate.

**1\. CLI (rustform-cli)**

* **Responsibility:** User interaction, parsing the YAML configuration, and orchestrating the code generation process.  
* **Tech Stack:**  
  * clap: For parsing command-line arguments (generate, init, etc.).  
  * serde\_yaml: For parsing the config.yml file into strongly-typed Rust structs.  
  * miette: For beautiful, diagnostic error reporting.

**2\. Code Generator (rustform-codegen)**

* **Responsibility:** Contains the logic for transforming the parsed YAML configuration into Rust source code files.  
* **Tech Stack:**  
  * tera: A powerful templating engine similar to Jinja2. We will have templates for main.rs, models.rs, handlers.rs, errors.rs, etc.  
  * **Logic:** The generator will iterate through the parsed config:  
    * For each model, it generates a struct with Serde and SQLx derives.  
    * For each endpoint with crud: true, it generates the corresponding REST handlers (create, read, update, delete).  
    * It constructs the main.rs file, assembling the Axum router, middleware, database connection pool, and server startup logic.

**3\. Runtime Crate (rustform-core)**

* **Responsibility:** A small, optional crate that the generated code can depend on. It provides common utilities, error types, and helper functions that would be redundant to generate every time.  
* **Potential Contents:**  
  * A standardized ApiError enum that can be converted into HTTP responses.  
  * Common middleware implementations or wrappers.  
  * Helper functions for establishing the database connection.

### **Tech Stack Choices**

* **Web Framework:** **Axum**. Chosen for its deep integration with the Tokio ecosystem, clean and ergonomic API (especially with extractors), and backing by the Tokio team.  
* **Database:** **SQLx**. Chosen for its compile-time checked queries, async-first design, and support for major SQL databases (Postgres, SQLite, MySQL).  
* **Serialization:** **Serde**. The de-facto standard for serialization and deserialization in Rust.  
* **Async Runtime:** **Tokio**. The most mature and widely-used async runtime in the Rust ecosystem.

### **Directory Structure of Generated Project**

.  
├── .env.example  
├── Cargo.toml  
├── config.yml      \<-- The source of truth  
├── sqlx-data.json  \<-- For compile-time query validation  
└── src/  
    ├── main.rs     \<-- Server setup, router, main function  
    ├── models.rs   \<-- Generated structs for database models  
    ├── handlers.rs \<-- Generated request handlers (CRUD)  
    └── error.rs    \<-- Standardized API error handling  
