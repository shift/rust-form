# **Project Roadmap**

This document outlines the planned development stages for Rustফর্ম.

### **Phase 1: Minimum Viable Product (MVP)**

The goal of the MVP is to prove the core concept: generating a functional, CRUD-based web service from a YAML file.

* \[ \] **CLI:** Basic rustform generate command.  
* \[ \] **Configuration:** Support for project\_name, database (SQLite only), and api.models.  
* \[ \] **Code Generation:**  
  * Generate struct definitions for models with serde and sqlx::FromRow.  
  * Generate full RESTful CRUD handlers for a single model (/todos example).  
  * Generate main.rs with Axum router and SQLite connection pool.  
* \[ \] **Database Support:** SQLite only.  
* \[ \] **Middleware:** Basic logging.

### **Phase 2: Core Features & Extensibility**

This phase focuses on making Rustফর্ম useful for a wider range of simple applications.

* \[ \] **Database Support:** Add support for PostgreSQL.  
* \[ \] **Model Relationships:** Define basic relationships (e.g., belongs\_to, has\_many) in the YAML and generate appropriate handlers.  
* \[ \] **Richer Field Types:** Add support for more specific types like uuid, json, and enums.  
* \[ \] **Validation:** Add basic validation rules to model fields (e.g., min\_length, max\_length, email).  
* \[ \] **Custom Logic Injection:** Implement a "lambda" system to allow users to inject custom Rust code for specific handler logic. This is critical for moving beyond basic CRUD.  
* \[ \] **Middleware:** Add support for CORS configuration.

### **Phase 3: Production Readiness & Advanced Features**

This phase aims to make Rustফর্ম a viable choice for production services.

* \[ \] **Authentication:** Add declarative authentication strategies (e.g., JWT, API Key).  
* \[ \] **Configuration Management:** Better handling of secrets and environment-specific configuration.  
* \[ \] **Testing:** Auto-generate basic integration tests for the generated endpoints.  
* \[ \] **Background Jobs:** Define simple background jobs or tasks in the YAML.  
* \[ \] **Deployment:** Add helpers or documentation for deploying the binary (e.g., generating a Dockerfile).

### **Future Ideas (Post v1.0)**

* \[ \] GraphQL API generation.  
* \[ \] WebSockets support.  
* \[ \] Plugin system for community-contributed features.  
* \[ \] A rustform watch command for live regeneration during development.