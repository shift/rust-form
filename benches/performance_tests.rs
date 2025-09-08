use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rustform_core::config::{Config, validation::validate_config};
use rustform_codegen::Pipeline;
use tempfile::TempDir;
use std::path::PathBuf;

fn benchmark_config_parsing(c: &mut Criterion) {
    let configs = vec![
        ("small", include_str!("../examples/todo.yml")),
        ("medium", include_str!("../examples/ecommerce.yml")),
        ("large", include_str!("../examples/saas-analytics-platform.yml")),
    ];
    
    let mut group = c.benchmark_group("config_parsing");
    
    for (name, config_content) in configs {
        group.bench_with_input(BenchmarkId::new("parse", name), config_content, |b, content| {
            b.iter(|| {
                let config: Config = serde_yaml::from_str(black_box(content)).expect("Should parse");
                black_box(config)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("parse_and_validate", name), config_content, |b, content| {
            b.iter(|| {
                let config: Config = serde_yaml::from_str(black_box(content)).expect("Should parse");
                validate_config(&config).expect("Should validate");
                black_box(config)
            })
        });
    }
    
    group.finish();
}

fn benchmark_code_generation(c: &mut Criterion) {
    let configs = vec![
        ("todo", create_todo_config()),
        ("ecommerce", create_ecommerce_config()),
        ("complex", create_complex_config()),
    ];
    
    let mut group = c.benchmark_group("code_generation");
    group.sample_size(10); // Reduce sample size for expensive operations
    
    for (name, config) in configs {
        group.bench_with_input(BenchmarkId::new("full_generation", name), &config, |b, config| {
            b.iter(|| {
                let temp_dir = TempDir::new().expect("Failed to create temp dir");
                let output_dir = temp_dir.path().join("generated");
                
                let mut pipeline = Pipeline::new().expect("Should create pipeline");
                pipeline.generate(black_box(config), &output_dir).expect("Should generate");
                
                black_box(output_dir)
            })
        });
    }
    
    group.finish();
}

fn benchmark_template_rendering(c: &mut Criterion) {
    let engine = rustform_codegen::CodegenEngine::new().expect("Should create engine");
    
    let contexts = vec![
        ("simple", create_simple_context()),
        ("complex", create_complex_context()),
    ];
    
    let templates = vec![
        "main.rs.tera",
        "models.rs.tera", 
        "handlers.rs.tera",
        "Cargo.toml.tera",
    ];
    
    let mut group = c.benchmark_group("template_rendering");
    
    for template in templates {
        for (context_name, context) in &contexts {
            let benchmark_name = format!("{}_{}", template.replace(".tera", ""), context_name);
            
            group.bench_with_input(
                BenchmarkId::new("render", benchmark_name), 
                &(template, context), 
                |b, (template, context)| {
                    b.iter(|| {
                        engine.render_template(black_box(template), black_box(context))
                            .expect("Should render template")
                    })
                }
            );
        }
    }
    
    group.finish();
}

fn benchmark_component_compatibility(c: &mut Criterion) {
    let manifests = vec![
        ("auth", create_auth_component_manifest()),
        ("database", create_database_component_manifest()),
        ("observability", create_observability_component_manifest()),
    ];
    
    let versions = vec![
        "0.1.0", "0.1.5", "0.2.0", "1.0.0", "2.0.0"
    ];
    
    let mut group = c.benchmark_group("component_compatibility");
    
    for (component_name, manifest) in manifests {
        for version in &versions {
            let benchmark_name = format!("{}_{}", component_name, version.replace('.', "_"));
            
            group.bench_with_input(
                BenchmarkId::new("check_compatibility", benchmark_name),
                &(manifest.clone(), version),
                |b, (manifest, version)| {
                    b.iter(|| {
                        manifest.compatibility_status(black_box(version)).expect("Should check compatibility")
                    })
                }
            );
        }
    }
    
    group.finish();
}

fn create_todo_config() -> Config {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: benchmark_todo
version: "1.0.0"
database:
  type: sqlite
  url_env: DATABASE_URL
api:
  models:
    Todo:
      table_name: todos
      fields:
        id:
          type: integer
          primary_key: true
        title:
          type: string
          required: true
        completed:
          type: boolean
          default: false
  endpoints:
    - path: /todos
      model: Todo
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
middleware:
  - logger: true
"#;
    serde_yaml::from_str(yaml).expect("Should parse todo config")
}

fn create_ecommerce_config() -> Config {
    let yaml = include_str!("../examples/ecommerce.yml");
    let mut config: Config = serde_yaml::from_str(yaml).expect("Should parse ecommerce config");
    config.schema_version = "1.0.0".to_string();
    config.api_version = "0.1.0".to_string();
    config
}

fn create_complex_config() -> Config {
    let yaml = r#"
schema_version: "1.0.0"
api_version: "0.1.0"
project_name: benchmark_complex
version: "1.0.0"
database:
  type: postgres
  url_env: DATABASE_URL
  pool_size: 50
api:
  models:
    User:
      table_name: users
      fields:
        id:
          type: uuid
          primary_key: true
        email:
          type: string
          required: true
          unique: true
        profile:
          type: json
    Product:
      table_name: products
      fields:
        id:
          type: uuid
          primary_key: true
        name:
          type: string
          required: true
        price:
          type: decimal
          required: true
        user_id:
          type: uuid
          required: true
      relationships:
        user:
          type: many_to_one
          model: User
          foreign_key: user_id
  endpoints:
    - path: /users
      model: User
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
    - path: /products
      model: Product
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
middleware:
  - logger: true
  - cors:
      allow_origin: "*"
  - rate_limit:
      max_requests: 1000
      window_seconds: 3600
"#;
    serde_yaml::from_str(yaml).expect("Should parse complex config")
}

fn create_simple_context() -> rustform_codegen::CodegenContext {
    let mut context = rustform_codegen::CodegenContext::new();
    context.insert("project_name", "simple_benchmark");
    context.insert("version", "1.0.0");
    context
}

fn create_complex_context() -> rustform_codegen::CodegenContext {
    let config = create_complex_config();
    rustform_codegen::CodegenContext::from_config(&config).expect("Should create context")
}

fn create_auth_component_manifest() -> rustform_core::component::ComponentManifest {
    let yaml = r#"
name: jwt-auth
description: JWT authentication component
category: auth
priority: high
version: "1.0.0"
author: "Benchmark"
license: "MIT"
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
"#;
    serde_yaml::from_str(yaml).expect("Should parse auth manifest")
}

fn create_database_component_manifest() -> rustform_core::component::ComponentManifest {
    let yaml = r#"
name: postgres-pool
description: PostgreSQL connection pool
category: database
priority: high
version: "1.0.0"
author: "Benchmark"
license: "MIT"
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
"#;
    serde_yaml::from_str(yaml).expect("Should parse database manifest")
}

fn create_observability_component_manifest() -> rustform_core::component::ComponentManifest {
    let yaml = r#"
name: prometheus-metrics
description: Prometheus metrics collection
category: observability
priority: medium
version: "1.0.0"
author: "Benchmark"
license: "MIT"
api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
"#;
    serde_yaml::from_str(yaml).expect("Should parse observability manifest")
}

criterion_group!(
    benches,
    benchmark_config_parsing,
    benchmark_code_generation,
    benchmark_template_rendering,
    benchmark_component_compatibility
);

criterion_main!(benches);