use opentelemetry::{
    global,
    trace::{Tracer, TracerProvider},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Sampler},
    Resource,
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::time::Duration;
use anyhow::Result;

use crate::config::ObservabilityConfig;

/// OpenTelemetry setup and configuration
pub struct OpenTelemetrySetup {
    config: ObservabilityConfig,
}

impl OpenTelemetrySetup {
    pub fn new(config: ObservabilityConfig) -> Self {
        Self { config }
    }

    /// Initialize OpenTelemetry with configured exporters
    pub async fn initialize(&self) -> Result<()> {
        // Create resource with service information
        let resource = self.create_resource();

        // Initialize tracer if tracing is enabled
        if self.config.tracing_enabled {
            self.setup_tracing(resource.clone()).await?;
        }

        // Initialize metrics if metrics are enabled
        if self.config.metrics_enabled {
            self.setup_metrics(resource).await?;
        }

        // Setup tracing subscriber with OpenTelemetry layer
        self.setup_subscriber()?;

        tracing::info!(
            service.name = %self.config.service_name,
            service.version = %self.config.service_version,
            "OpenTelemetry initialized successfully"
        );

        Ok(())
    }

    /// Create OpenTelemetry resource with service metadata
    fn create_resource(&self) -> Resource {
        let mut resource_attributes = vec![
            KeyValue::new("service.name", self.config.service_name.clone()),
            KeyValue::new("service.version", self.config.service_version.clone()),
            KeyValue::new("telemetry.sdk.name", "opentelemetry"),
            KeyValue::new("telemetry.sdk.language", "rust"),
            KeyValue::new("telemetry.sdk.version", env!("CARGO_PKG_VERSION")),
        ];

        // Add environment-specific attributes
        if let Ok(env) = std::env::var("ENVIRONMENT") {
            resource_attributes.push(KeyValue::new("deployment.environment", env));
        }

        if let Ok(namespace) = std::env::var("K8S_NAMESPACE") {
            resource_attributes.push(KeyValue::new("k8s.namespace.name", namespace));
        }

        if let Ok(pod_name) = std::env::var("K8S_POD_NAME") {
            resource_attributes.push(KeyValue::new("k8s.pod.name", pod_name));
        }

        if let Ok(container_id) = std::env::var("CONTAINER_ID") {
            resource_attributes.push(KeyValue::new("container.id", container_id));
        }

        // Add custom resource attributes from configuration
        for (key, value) in &self.config.resource_attributes {
            resource_attributes.push(KeyValue::new(key.clone(), value.clone()));
        }

        Resource::new(resource_attributes)
    }

    /// Setup distributed tracing with configured exporters
    async fn setup_tracing(&self, resource: Resource) -> Result<()> {
        let mut pipeline = opentelemetry_otlp::new_pipeline().tracing();

        // Configure OTLP exporter
        if !self.config.otlp_endpoint.is_empty() {
            pipeline = pipeline.with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(&self.config.otlp_endpoint)
                    .with_timeout(Duration::from_secs(30))
            );
        }

        // Configure trace settings
        let trace_config = trace::config()
            .with_sampler(self.create_sampler())
            .with_id_generator(RandomIdGenerator::default())
            .with_max_events_per_span(self.config.max_events_per_span as u32)
            .with_max_attributes_per_span(self.config.max_attributes_per_span as u32)
            .with_resource(resource);

        // Install the tracer
        let tracer = pipeline
            .with_trace_config(trace_config)
            .install_batch(opentelemetry_sdk::runtime::Tokio)?;

        // Set as global tracer
        global::set_tracer_provider(tracer);

        Ok(())
    }

    /// Create sampler based on configuration
    fn create_sampler(&self) -> Sampler {
        if self.config.sample_rate >= 1.0 {
            Sampler::AlwaysOn
        } else if self.config.sample_rate <= 0.0 {
            Sampler::AlwaysOff
        } else {
            Sampler::TraceIdRatioBased(self.config.sample_rate)
        }
    }

    /// Setup metrics collection
    async fn setup_metrics(&self, resource: Resource) -> Result<()> {
        // This will be implemented when metrics exporters are available
        tracing::info!("Metrics setup placeholder - will be implemented with Prometheus integration");
        Ok(())
    }

    /// Setup tracing subscriber with OpenTelemetry layer
    fn setup_subscriber(&self) -> Result<()> {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug,sqlx=info", self.config.service_name).into()
            });

        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().with_target(false));

        // Add OpenTelemetry layer if tracing is enabled
        if self.config.tracing_enabled {
            let tracer = global::tracer(&self.config.service_name);
            let subscriber = subscriber.with(OpenTelemetryLayer::new(tracer));
            subscriber.init();
        } else {
            subscriber.init();
        }

        Ok(())
    }

    /// Graceful shutdown of OpenTelemetry
    pub async fn shutdown(&self) -> Result<()> {
        if self.config.tracing_enabled {
            global::shutdown_tracer_provider();
        }

        tracing::info!("OpenTelemetry shutdown completed");
        Ok(())
    }
}

/// Create a custom span with business context
#[tracing::instrument(
    name = "business_operation",
    skip_all,
    fields(
        operation.name = %operation_name,
        operation.id = %operation_id,
        user.id = tracing::field::Empty,
        error.message = tracing::field::Empty,
    )
)]
pub async fn trace_business_operation<F, T>(
    operation_name: &str,
    operation_id: &str,
    user_id: Option<&str>,
    operation: F,
) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    // Record user ID if provided
    if let Some(uid) = user_id {
        tracing::Span::current().record("user.id", uid);
    }

    // Record operation start
    tracing::info!(
        operation.status = "started",
        "Business operation started"
    );

    // Execute the operation
    match operation.await {
        Ok(result) => {
            tracing::info!(
                operation.status = "completed",
                "Business operation completed successfully"
            );
            Ok(result)
        }
        Err(error) => {
            tracing::Span::current().record("error.message", error.to_string());
            tracing::error!(
                operation.status = "failed",
                error = %error,
                "Business operation failed"
            );
            Err(error)
        }
    }
}

/// Add custom attributes to the current span
pub fn add_span_attributes(attributes: Vec<(&str, &str)>) {
    let span = tracing::Span::current();
    for (key, value) in attributes {
        span.record(key, value);
    }
}

/// Create a custom event in the current span
pub fn record_span_event(name: &str, attributes: Vec<(&str, &str)>) {
    tracing::info!(
        event.name = name,
        ?attributes,
        "Custom span event"
    );
}

/// Macro for creating instrumented database operations
#[macro_export]
macro_rules! trace_database_operation {
    ($db_operation:expr, $table:expr, $operation_type:expr) => {
        tracing::info_span!(
            "database_operation",
            db.operation = $operation_type,
            db.table = $table,
            db.duration_ms = tracing::field::Empty,
        )
        .in_scope(|| async {
            let start = std::time::Instant::now();
            let result = $db_operation.await;
            let duration = start.elapsed();
            
            tracing::Span::current().record("db.duration_ms", duration.as_millis());
            
            match &result {
                Ok(_) => tracing::info!("Database operation completed successfully"),
                Err(error) => tracing::error!(error = %error, "Database operation failed"),
            }
            
            result
        })
    };
}

/// Macro for creating instrumented HTTP client calls
#[macro_export]
macro_rules! trace_http_client {
    ($http_call:expr, $method:expr, $url:expr) => {
        tracing::info_span!(
            "http_client_request",
            http.method = $method,
            http.url = $url,
            http.status_code = tracing::field::Empty,
            http.duration_ms = tracing::field::Empty,
        )
        .in_scope(|| async {
            let start = std::time::Instant::now();
            let result = $http_call.await;
            let duration = start.elapsed();
            
            tracing::Span::current().record("http.duration_ms", duration.as_millis());
            
            match &result {
                Ok(response) => {
                    tracing::Span::current().record("http.status_code", response.status().as_u16());
                    tracing::info!("HTTP client request completed");
                }
                Err(error) => {
                    tracing::error!(error = %error, "HTTP client request failed");
                }
            }
            
            result
        })
    };
}

/// Custom error extension for tracing
pub trait TracingErrorExt {
    fn with_span_error(self) -> Self;
}

impl<E: std::fmt::Display> TracingErrorExt for Result<(), E> {
    fn with_span_error(self) -> Self {
        match &self {
            Ok(_) => {}
            Err(error) => {
                tracing::Span::current().record("error.message", error.to_string());
                tracing::error!(error = %error, "Operation failed");
            }
        }
        self
    }
}

/// Utility for creating correlation IDs
pub fn generate_correlation_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Extract trace context for propagation
pub fn extract_trace_context() -> Option<String> {
    use opentelemetry::trace::TraceContextExt;
    use opentelemetry::Context;
    
    let context = Context::current();
    let span_context = context.span().span_context();
    
    if span_context.is_valid() {
        Some(format!(
            "{:032x}{:016x}",
            span_context.trace_id(),
            span_context.span_id()
        ))
    } else {
        None
    }
}

/// Test utilities for OpenTelemetry in tests
#[cfg(test)]
pub mod test_utils {
    use super::*;
    use tracing_subscriber::fmt::TestWriter;

    pub fn setup_test_tracing() {
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_writer(TestWriter::default()));
        
        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    pub async fn with_test_span<F, T>(name: &str, f: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        tracing::info_span!(name).in_scope(f).await
    }
}