# Rust-form Observability Framework

## Overview

The Rust-form Observability Framework provides comprehensive monitoring, tracing, and metrics collection using OpenTelemetry and Prometheus. All observability features are configured through YAML and automatically integrated into generated applications.

## Architecture

### Current State Analysis

**✅ Currently Available:**
- Basic tracing with `tracing` crate
- HTTP request tracing via `tower-http::trace::TraceLayer`
- Environment-based log level configuration
- Structured logging to stdout

**❌ Missing Components (To Be Implemented):**
- OpenTelemetry integration
- Prometheus metrics endpoint
- Distributed tracing
- Custom business metrics
- Metrics authentication
- Horizontal scaling monitoring
- Performance monitoring
- Error rate tracking

### Observability Components Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Observability Framework                  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │  OpenTelemetry  │  │   Prometheus    │  │   Custom     │ │
│  │     Tracing     │  │    Metrics      │  │   Metrics    │ │
│  │                 │  │                 │  │              │ │
│  │ • Spans         │  │ • HTTP Metrics  │  │ • Business   │ │
│  │ • Traces        │  │ • System Stats  │  │ • Domain     │ │
│  │ • Baggage       │  │ • Database      │  │ • Custom     │ │
│  │ • Context       │  │ • Auth Metrics  │  │              │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │  Exporters      │  │  Authentication │  │  Horizontal  │ │
│  │                 │  │                 │  │   Scaling    │ │
│  │ • OTLP          │  │ • Token Auth    │  │              │ │
│  │ • Jaeger        │  │ • Bearer Token  │  │ • Instance   │ │
│  │ • Zipkin        │  │ • JWT Support   │  │   Discovery  │ │
│  │ • Console       │  │ • Rate Limiting │  │ • Load Bal.  │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Configuration Schema

### Complete Observability Configuration

```yaml
# rustform.yml - Observability configuration
name: "my-observed-app"

# Observability framework configuration
observability:
  # OpenTelemetry configuration
  opentelemetry:
    enabled: true
    service_name: "my-observed-app"
    service_version: "1.0.0"
    
    # Tracing configuration
    tracing:
      enabled: true
      sample_rate: 1.0  # 0.0 to 1.0 (1.0 = trace everything)
      max_events_per_span: 128
      max_attributes_per_span: 128
      
      # Exporters
      exporters:
        - type: "otlp"
          endpoint: "http://jaeger:14268/api/traces"
          headers:
            authorization: "Bearer ${JAEGER_TOKEN}"
        - type: "console"
          enabled_in_dev: true
    
    # Metrics configuration  
    metrics:
      enabled: true
      export_interval: "10s"
      
      # Built-in metrics
      http_metrics: true
      database_metrics: true
      system_metrics: true
      custom_metrics: true
      
      # Exporters
      exporters:
        - type: "prometheus"
          endpoint: "/metrics"
          authentication:
            enabled: true
            method: "bearer_token"
            token_header: "Authorization"
        - type: "otlp"
          endpoint: "http://otel-collector:4317"

    # Resource attributes
    resource:
      attributes:
        service.name: "my-observed-app"
        service.version: "1.0.0"
        deployment.environment: "${ENVIRONMENT}"
        k8s.namespace.name: "${K8S_NAMESPACE}"
        k8s.pod.name: "${K8S_POD_NAME}"

  # Prometheus configuration
  prometheus:
    enabled: true
    endpoint: "/metrics"
    
    # Authentication for metrics endpoint
    authentication:
      enabled: true
      method: "bearer_token"  # bearer_token, api_key, jwt
      token_env: "METRICS_TOKEN"
      required_scopes: ["metrics:read"]
      
      # Rate limiting
      rate_limit:
        requests_per_minute: 60
        burst_size: 10
    
    # Metrics configuration
    metrics:
      # HTTP metrics
      http:
        enabled: true
        track_user_agents: false
        track_ip_addresses: false  # Privacy consideration
        histogram_buckets: [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
      
      # Database metrics
      database:
        enabled: true
        track_queries: false  # Don't track actual SQL for privacy
        connection_pool: true
        query_duration: true
      
      # Business metrics
      business:
        enabled: true
        custom_counters: ["user_registrations", "orders_created", "payments_processed"]
        custom_gauges: ["active_users", "inventory_levels"]
        custom_histograms: ["order_values", "processing_times"]

  # Logging configuration
  logging:
    level: "info"  # trace, debug, info, warn, error
    format: "json"  # json, pretty
    
    # Structured logging fields
    include_spans: true
    include_targets: true
    include_thread_ids: false
    include_line_numbers: false  # Performance consideration
    
    # Log sampling (for high-traffic scenarios)
    sampling:
      enabled: false
      rate: 0.1  # Sample 10% of log events

  # Health checks and monitoring
  health:
    enabled: true
    endpoint: "/health"
    checks:
      - type: "database"
        timeout: "5s"
      - type: "external_service"
        name: "auth_service"
        url: "http://auth-service/health"
        timeout: "3s"
      - type: "custom"
        name: "business_logic"
        handler: "check_business_health"

# Horizontal scaling configuration
scaling:
  enabled: true
  strategy: "horizontal"
  
  # Load balancing
  load_balancer:
    algorithm: "round_robin"  # round_robin, least_connections, ip_hash
    health_check_interval: "30s"
    max_failures: 3
    
  # Service discovery
  service_discovery:
    method: "dns"  # dns, consul, kubernetes
    service_name: "my-observed-app"
    port: 3000
    
  # Instance management
  instances:
    min_instances: 2
    max_instances: 10
    target_cpu_utilization: 70
    target_memory_utilization: 80
    
    # Graceful shutdown
    shutdown_timeout: "30s"
    drain_timeout: "10s"

# Custom metrics definitions
custom_metrics:
  counters:
    - name: "user_registrations_total"
      description: "Total number of user registrations"
      labels: ["registration_method", "user_type"]
      
    - name: "orders_total"
      description: "Total number of orders processed"
      labels: ["status", "payment_method"]
  
  gauges:
    - name: "active_users"
      description: "Number of currently active users"
      labels: ["session_type"]
      
    - name: "inventory_items"
      description: "Current inventory levels"
      labels: ["product_category", "warehouse"]
  
  histograms:
    - name: "request_duration_seconds"
      description: "HTTP request duration in seconds"
      labels: ["method", "endpoint", "status"]
      buckets: [0.1, 0.5, 1.0, 2.0, 5.0]
      
    - name: "database_query_duration_seconds"
      description: "Database query execution time"
      labels: ["query_type", "table"]
      buckets: [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]

# Alert definitions (for integration with alerting systems)
alerts:
  - name: "high_error_rate"
    condition: "http_requests_total{status=~'5..'} / http_requests_total > 0.05"
    duration: "5m"
    severity: "warning"
    description: "HTTP error rate is above 5%"
    
  - name: "high_response_time"
    condition: "histogram_quantile(0.95, http_request_duration_seconds) > 2.0"
    duration: "10m"
    severity: "critical"
    description: "95th percentile response time is above 2 seconds"
    
  - name: "database_connection_exhaustion"
    condition: "database_connections_active / database_connections_max > 0.9"
    duration: "2m"
    severity: "critical"
    description: "Database connection pool is nearly exhausted"
```

## Components to Implement

### 1. OpenTelemetry Integration Component

**Component**: `observability/opentelemetry-integration`

```yaml
name: "opentelemetry-integration"
description: "Complete OpenTelemetry tracing and metrics integration"
version: "1.0.0"
category: "observability"
priority: "high"
complexity: "medium"

dependencies:
  rust:
    - "opentelemetry = \"0.21\""
    - "opentelemetry-otlp = \"0.14\""
    - "opentelemetry-jaeger = \"0.20\""
    - "opentelemetry_sdk = \"0.21\""
    - "opentelemetry-prometheus = \"0.14\""
    - "tracing-opentelemetry = \"0.22\""
    - "prometheus = \"0.13\""
    - "axum-prometheus = \"0.4\""

features:
  - "automatic_instrumentation"
  - "distributed_tracing"
  - "metrics_collection"
  - "custom_spans"
  - "baggage_propagation"
  - "resource_detection"
```

### 2. Prometheus Metrics Component

**Component**: `observability/prometheus-metrics`

```yaml
name: "prometheus-metrics"
description: "Prometheus metrics endpoint with authentication"
version: "1.0.0"
category: "observability"
priority: "high"
complexity: "medium"

dependencies:
  rust:
    - "prometheus = \"0.13\""
    - "axum-prometheus = \"0.4\""
    - "tower-http = { version = \"0.5\", features = [\"auth\"] }"
    - "jsonwebtoken = \"9.0\""

features:
  - "metrics_endpoint"
  - "token_authentication"
  - "rate_limiting"
  - "custom_metrics"
  - "histogram_configuration"
  - "gauge_tracking"
```

### 3. Metrics Authentication Component

**Component**: `observability/metrics-auth`

```yaml
name: "metrics-auth"
description: "Authentication and authorization for metrics endpoints"
version: "1.0.0"
category: "observability"
subcategory: "security"
priority: "medium"
complexity: "low"

features:
  - "bearer_token_auth"
  - "jwt_validation"
  - "scope_verification"
  - "rate_limiting"
  - "token_generation"
  - "token_rotation"
```

### 4. Horizontal Scaling Component

**Component**: `infrastructure/horizontal-scaling`

```yaml
name: "horizontal-scaling"
description: "Horizontal scaling support with service discovery"
version: "1.0.0"
category: "infrastructure"
priority: "medium"
complexity: "high"

dependencies:
  rust:
    - "consul = \"0.4\""
    - "tokio = { version = \"1.0\", features = [\"signal\"] }"
    - "sysinfo = \"0.29\""

features:
  - "service_discovery"
  - "health_checks"
  - "graceful_shutdown"
  - "load_balancing"
  - "instance_management"
  - "auto_scaling"
```

## Implementation Tasks

### Phase 1: OpenTelemetry Foundation
**Priority**: High  
**Timeline**: 1-2 weeks

1. **Create OpenTelemetry integration component**
   - Auto-instrument HTTP requests
   - Database query tracing
   - Custom span creation
   - Context propagation

2. **Implement tracing exporters**
   - OTLP exporter for standard collectors
   - Jaeger exporter for direct integration
   - Console exporter for development

3. **Add metrics collection**
   - HTTP request metrics
   - Database connection pool metrics
   - System resource metrics
   - Custom business metrics

### Phase 2: Prometheus Integration  
**Priority**: High  
**Timeline**: 1 week

1. **Create Prometheus metrics endpoint**
   - Secure `/metrics` endpoint
   - Token-based authentication
   - Rate limiting protection

2. **Implement metrics authentication**
   - Bearer token validation
   - JWT support with scopes
   - Token generation utilities
   - Token rotation capabilities

3. **Add comprehensive metrics**
   - HTTP response time histograms
   - Error rate counters
   - Active connection gauges
   - Custom business metrics

### Phase 3: Horizontal Scaling Support
**Priority**: Medium  
**Timeline**: 2-3 weeks

1. **Service discovery integration**
   - DNS-based discovery
   - Consul integration
   - Kubernetes service discovery
   - Health check propagation

2. **Load balancing support**
   - Instance registration
   - Health monitoring
   - Graceful shutdown handling
   - Connection draining

3. **Auto-scaling capabilities**
   - CPU/memory monitoring
   - Instance scaling logic
   - Configuration-driven scaling
   - Metrics-based decisions

### Phase 4: Advanced Features
**Priority**: Low  
**Timeline**: 1-2 weeks

1. **Enhanced monitoring**
   - Custom dashboards generation
   - Alert rule templates
   - SLA monitoring
   - Performance profiling

2. **Integration tooling**
   - Grafana dashboard exports
   - Alertmanager configuration
   - Kubernetes manifests
   - Docker Compose templates

## Token Management

### Metrics Token Generation

Create a utility for generating and managing metrics access tokens:

```rust
// Auto-generated token management
use rustform::observability::MetricsTokenManager;

// Generate a new metrics token
let token_manager = MetricsTokenManager::new("your-secret-key");
let token = token_manager.generate_token(
    vec!["metrics:read"], // scopes
    Duration::days(30),   // expiry
)?;

println!("Metrics token: {}", token);
```

### CLI Integration

```bash
# Generate metrics token
rustform metrics generate-token --scopes "metrics:read" --expires-in "30d"

# Rotate existing token  
rustform metrics rotate-token --token-id "token-123"

# List active tokens
rustform metrics list-tokens
```

## Generated Observability Code

### Auto-Generated Instrumentation

```rust
// Auto-generated in main.rs
use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use tracing_opentelemetry::OpenTelemetryLayer;
use prometheus::{Encoder, TextEncoder, Registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("{{ observability.opentelemetry.tracing.exporters.0.endpoint }}")
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_sampler(opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(
                    {{ observability.opentelemetry.tracing.sample_rate }}
                ))
                .with_resource(opentelemetry_sdk::Resource::new(vec![
                    KeyValue::new("service.name", "{{ observability.opentelemetry.service_name }}"),
                    KeyValue::new("service.version", "{{ observability.opentelemetry.service_version }}"),
                ]))
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    // Initialize Prometheus metrics
    let registry = Registry::new();
    let metrics = create_prometheus_metrics(&registry)?;
    
    // Setup tracing with OpenTelemetry
    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create app with observability
    let app = create_router_with_observability(state, metrics, registry);
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], {{ server.port }}));
    tracing::info!("Server starting with full observability on {}", addr);
    
    axum::serve(listener, app).await?;
    
    // Graceful shutdown
    opentelemetry::global::shutdown_tracer_provider();
    
    Ok(())
}
```

### Auto-Generated Metrics

```rust
// Auto-generated metrics collectors
use prometheus::{Counter, Histogram, Gauge, Registry};

#[derive(Clone)]
pub struct AppMetrics {
    pub http_requests_total: Counter,
    pub http_request_duration: Histogram,
    pub database_connections_active: Gauge,
    pub active_users: Gauge,
    // Custom business metrics
    {% for counter in custom_metrics.counters %}
    pub {{ counter.name | snake_case }}: Counter,
    {% endfor %}
    {% for gauge in custom_metrics.gauges %}
    pub {{ gauge.name | snake_case }}: Gauge,
    {% endfor %}
    {% for histogram in custom_metrics.histograms %}
    pub {{ histogram.name | snake_case }}: Histogram,
    {% endfor %}
}

impl AppMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let metrics = Self {
            http_requests_total: Counter::new("http_requests_total", "Total HTTP requests")?
                .with_label_values(&[]),
            http_request_duration: Histogram::with_opts(
                HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
                    .buckets({{ observability.prometheus.metrics.http.histogram_buckets | json }})
            )?,
            database_connections_active: Gauge::new("database_connections_active", "Active database connections")?,
            active_users: Gauge::new("active_users", "Currently active users")?,
            // Register custom metrics
            {% for counter in custom_metrics.counters %}
            {{ counter.name | snake_case }}: Counter::new("{{ counter.name }}", "{{ counter.description }}")?,
            {% endfor %}
            {% for gauge in custom_metrics.gauges %}
            {{ gauge.name | snake_case }}: Gauge::new("{{ gauge.name }}", "{{ gauge.description }}")?,
            {% endfor %}
            {% for histogram in custom_metrics.histograms %}
            {{ histogram.name | snake_case }}: Histogram::with_opts(
                HistogramOpts::new("{{ histogram.name }}", "{{ histogram.description }}")
                    .buckets({{ histogram.buckets | json }})
            )?,
            {% endfor %}
        };

        // Register all metrics
        registry.register(Box::new(metrics.http_requests_total.clone()))?;
        registry.register(Box::new(metrics.http_request_duration.clone()))?;
        registry.register(Box::new(metrics.database_connections_active.clone()))?;
        registry.register(Box::new(metrics.active_users.clone()))?;
        
        Ok(metrics)
    }
}
```

### Auto-Generated Metrics Endpoint

```rust
// Auto-generated secure metrics endpoint
use axum::{
    extract::{State, Request},
    http::{StatusCode, HeaderMap},
    response::Response,
    middleware::Next,
};
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_endpoint(
    State(registry): State<Arc<Registry>>,
) -> Result<Response<String>, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => Ok(Response::builder()
            .header("content-type", "text/plain; version=0.0.4")
            .body(metrics)
            .unwrap()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Authentication middleware for metrics
pub async fn metrics_auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    
    let auth_header = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = &auth_header[7..];
    
    // Validate token
    if !validate_metrics_token(token).await? {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Rate limiting check
    if !check_rate_limit(extract_client_ip(&request)).await? {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}
```

## Benefits

### Automatic Observability
- **Zero Configuration**: Full observability with simple YAML configuration
- **Industry Standards**: OpenTelemetry and Prometheus compliance
- **Production Ready**: Authentication, rate limiting, and security built-in
- **Compliance Friendly**: Audit trails and data privacy considerations

### Horizontal Scaling Support
- **Service Discovery**: Automatic instance registration and discovery
- **Health Monitoring**: Comprehensive health checks and monitoring
- **Graceful Operations**: Proper shutdown and connection draining
- **Load Balancing**: Built-in load balancing strategies

### Developer Experience
- **Rich Metrics**: Business and technical metrics out-of-the-box
- **Easy Integration**: Works with Grafana, Prometheus, Jaeger
- **Debugging Tools**: Distributed tracing for complex request flows
- **Performance Insights**: Detailed performance and resource metrics

This observability framework transforms monitoring from a complex operational burden into a simple configuration option, providing enterprise-grade observability that scales with your application.