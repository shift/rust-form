# **Rustফর্ম (Rust-form)**

**\[WIP\] Declarative, Type-Safe Web Backends in Rust**

**Motto:** Define the *what*, not the *how*.

Rustফর্ম is a command-line tool that generates a high-performance, memory-safe web backend from a simple YAML configuration file. Inspired by the declarative philosophy of [ESPHome](https://esphome.io/) and the robustness of the Rust ecosystem, it aims to eliminate boilerplate and accelerate development without sacrificing performance.

### **Core Concept**

You define your entire backend—database models, API endpoints, middleware, and configuration—in a single config.yml. Rustফর্ম takes this definition and generates a complete, compilable Rust project using best-in-class libraries like Axum, SQLx, and Serde.

The result is a single, statically-compiled binary that is incredibly fast, efficient, and secure by default.

**Workflow:**

1. **Define:** Describe your backend in config.yml.  
2. **Generate:** Run rustform generate.  
3. **Compile:** Run cargo build \--release.  
4. **Deploy:** Ship your native binary.

### **Example: A Simple config.yml**

project\_name: todo\_api  
version: "0.1.0"

database:  
  type: sqlite  
  url\_env: DATABASE\_URL \# Read from this environment variable

api:  
  models:  
    Todo:  
      table\_name: todos  
      fields:  
        id:  
          type: integer  
          primary\_key: true  
          auto\_increment: true  
        title:  
          type: string  
          required: true  
        completed:  
          type: boolean  
          default: false  
        created\_at:  
          type: datetime  
          auto\_now\_add: true

  endpoints:  
    \- path: /todos  
      model: Todo  
      \# Automatically generate full RESTful CRUD handlers  
      crud:  
        create: true  
        read\_all: true  
        read\_one: true  
        update: true  
        delete: true

middleware:  
  \- logger: true  
  \- cors:  
      allow\_origin: "\*"

### **Key Features**

* **Declarative:** Focus on your application's logic, not the setup.  
* **Type-Safe:** Generates strongly-typed Rust code that catches errors at compile-time.  
* **High-Performance:** Built on the [Axum](https://github.com/tokio-rs/axum) web framework and [Tokio](https://tokio.rs/) async runtime.  
* **Async Database:** Uses [SQLx](https://github.com/launchbadge/sqlx) for fully asynchronous, compile-time checked database queries.  
* **Extensible:** Planned support for custom logic injection ("lambdas") for when you need to go beyond the configuration.  
* **Zero-Cost Abstraction:** The generated code is as performant as if you wrote it by hand.