# Horizontal Scaling Architecture for Rust-form

## Overview

Rust-form applications support horizontal scaling at the application layer through automatic service discovery, load balancing, and health monitoring. The scaling system is configured through YAML and provides production-ready scaling capabilities for high-availability deployments.

## Architecture Components

### 1. Service Discovery

**DNS-Based Discovery**
- Automatic instance registration with DNS
- Health check integration
- Graceful service deregistration

**Consul Integration**
- Dynamic service discovery
- Health monitoring and alerting
- Key-value configuration sharing

**Kubernetes Native**
- Service mesh integration
- Pod lifecycle management
- Automatic scaling with HPA

### 2. Load Balancing Strategies

**Round Robin** (Default)
- Equal distribution across healthy instances
- Simple and effective for most workloads

**Least Connections**
- Routes to instance with fewest active connections
- Optimal for variable request processing times

**IP Hash**
- Consistent routing based on client IP
- Useful for session affinity requirements

**Weighted Round Robin**
- Distributes load based on instance capacity
- Supports heterogeneous instance types

### 3. Health Monitoring

**Application Health Checks**
- Database connectivity validation
- External service dependency checks
- Custom business logic health validation

**System Health Metrics**
- CPU and memory utilization
- Network connectivity
- Disk space and I/O

**Graceful Degradation**
- Circuit breaker pattern implementation
- Retry logic with exponential backoff
- Fallback service responses

## Configuration

### Complete Horizontal Scaling Configuration

```yaml
# rustform.yml - Horizontal scaling configuration
name: "scalable-app"

# Horizontal scaling configuration
scaling:
  enabled: true
  strategy: "horizontal"
  
  # Service discovery configuration
  service_discovery:
    method: "kubernetes"  # dns, consul, kubernetes, static
    service_name: "scalable-app"
    namespace: "production"
    port: 3000
    
    # Health check configuration
    health_check:
      endpoint: "/health"
      interval: "30s"
      timeout: "5s"
      healthy_threshold: 2
      unhealthy_threshold: 3
      
    # Kubernetes specific
    kubernetes:
      service_type: "ClusterIP"
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "3000"
        prometheus.io/path: "/metrics"
    
    # Consul specific
    consul:
      address: "consul.service.consul:8500"
      datacenter: "dc1"
      token: "${CONSUL_TOKEN}"
      
  # Load balancing configuration
  load_balancer:
    algorithm: "round_robin"  # round_robin, least_connections, ip_hash, weighted_round_robin
    session_affinity: false
    health_check_interval: "10s"
    max_failures: 3
    failure_timeout: "60s"
    
    # Circuit breaker configuration
    circuit_breaker:
      enabled: true
      failure_threshold: 5
      recovery_timeout: "30s"
      half_open_max_calls: 3
      
  # Instance management
  instances:
    min_instances: 2
    max_instances: 20
    target_cpu_utilization: 70
    target_memory_utilization: 80
    scale_up_cooldown: "300s"
    scale_down_cooldown: "300s"
    
    # Instance lifecycle
    startup:
      initial_delay: "30s"
      readiness_probe: "/ready"
      liveness_probe: "/health"
      
    shutdown:
      graceful_timeout: "30s"
      drain_timeout: "15s"
      force_timeout: "60s"

# Enhanced health checks
health:
  enabled: true
  endpoint: "/health"
  
  # Health check definitions
  checks:
    - name: "database"
      type: "database"
      timeout: "5s"
      critical: true
      
    - name: "redis_cache"
      type: "external_service"
      url: "redis://redis:6379"
      timeout: "3s"
      critical: false
      
    - name: "auth_service"
      type: "http"
      url: "http://auth-service/health"
      timeout: "3s"
      critical: true
      
    - name: "business_logic"
      type: "custom"
      handler: "check_business_health"
      timeout: "10s"
      critical: true

# Observability for scaling
observability:
  # Scaling-specific metrics
  scaling_metrics:
    enabled: true
    metrics:
      - "instance_count"
      - "cpu_utilization_percentage"
      - "memory_utilization_percentage"
      - "active_connections"
      - "request_queue_depth"
      - "response_time_p95"
      
  # Auto-scaling triggers
  auto_scaling:
    enabled: true
    metrics:
      - metric: "cpu_utilization_percentage"
        threshold: 70
        action: "scale_up"
        cooldown: "300s"
        
      - metric: "cpu_utilization_percentage"
        threshold: 20
        action: "scale_down"
        cooldown: "600s"
        
      - metric: "response_time_p95"
        threshold: 2000  # 2 seconds
        action: "scale_up"
        cooldown: "180s"
        
      - metric: "active_connections"
        threshold: 1000
        action: "scale_up"
        cooldown: "120s"

# Environment-specific scaling
environments:
  development:
    scaling:
      instances:
        min_instances: 1
        max_instances: 2
        
  staging:
    scaling:
      instances:
        min_instances: 1
        max_instances: 5
        
  production:
    scaling:
      instances:
        min_instances: 3
        max_instances: 50
```

## Generated Infrastructure Code

### Auto-Generated Service Discovery

```rust
// Auto-generated service discovery implementation
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval, sleep};
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

use crate::config::ScalingConfig;

#[derive(Clone)]
pub struct ServiceDiscovery {
    config: ScalingConfig,
    discovery_client: Arc<dyn DiscoveryClient>,
    health_checker: Arc<HealthChecker>,
}

impl ServiceDiscovery {
    pub async fn new(config: ScalingConfig) -> Result<Self, ScalingError> {
        let discovery_client = create_discovery_client(&config).await?;
        let health_checker = Arc::new(HealthChecker::new(config.clone()));

        Ok(Self {
            config,
            discovery_client,
            health_checker,
        })
    }

    /// Register this instance with service discovery
    pub async fn register_instance(&self) -> Result<(), ScalingError> {
        let instance = ServiceInstance {
            id: generate_instance_id(),
            name: self.config.service_name.clone(),
            address: get_local_ip().await?,
            port: self.config.port,
            health_check_url: format!("http://{}:{}{}", 
                get_local_ip().await?, 
                self.config.port,
                self.config.health_check.endpoint
            ),
            metadata: create_instance_metadata(),
            registered_at: chrono::Utc::now(),
        };

        self.discovery_client.register(&instance).await?;
        
        info!(
            instance_id = %instance.id,
            service_name = %instance.name,
            address = %instance.address,
            port = instance.port,
            "Service instance registered successfully"
        );

        // Start health check loop
        self.start_health_checks().await;

        Ok(())
    }

    /// Discover other service instances
    pub async fn discover_instances(&self) -> Result<Vec<ServiceInstance>, ScalingError> {
        let instances = self.discovery_client
            .discover(&self.config.service_name)
            .await?;

        debug!(
            service_name = %self.config.service_name,
            instance_count = instances.len(),
            "Discovered service instances"
        );

        Ok(instances)
    }

    /// Start periodic health checks
    async fn start_health_checks(&self) {
        let health_checker = self.health_checker.clone();
        let discovery_client = self.discovery_client.clone();
        let check_interval = self.config.health_check.interval;

        tokio::spawn(async move {
            let mut interval = interval(check_interval);
            
            loop {
                interval.tick().await;
                
                match health_checker.check_health().await {
                    Ok(health_status) => {
                        if let Err(e) = discovery_client.update_health(health_status).await {
                            warn!(error = %e, "Failed to update health status");
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "Health check failed");
                    }
                }
            }
        });
    }

    /// Graceful deregistration
    pub async fn deregister_instance(&self) -> Result<(), ScalingError> {
        info!("Starting graceful service deregistration");

        // Mark instance as draining
        self.discovery_client.mark_draining().await?;

        // Wait for drain timeout to allow existing connections to complete
        let drain_timeout = self.config.shutdown.drain_timeout;
        info!(timeout_seconds = drain_timeout.as_secs(), "Draining connections");
        sleep(drain_timeout).await;

        // Deregister from service discovery
        self.discovery_client.deregister().await?;

        info!("Service instance deregistered successfully");
        Ok(())
    }
}

/// Health checker for service instances
#[derive(Clone)]
pub struct HealthChecker {
    config: ScalingConfig,
    checks: Vec<Box<dyn HealthCheck>>,
}

impl HealthChecker {
    pub fn new(config: ScalingConfig) -> Self {
        let mut checks: Vec<Box<dyn HealthCheck>> = vec![];

        // Add configured health checks
        for check_config in &config.health.checks {
            match check_config.check_type.as_str() {
                "database" => checks.push(Box::new(DatabaseHealthCheck::new(check_config.clone()))),
                "external_service" => checks.push(Box::new(ExternalServiceHealthCheck::new(check_config.clone()))),
                "http" => checks.push(Box::new(HttpHealthCheck::new(check_config.clone()))),
                "custom" => checks.push(Box::new(CustomHealthCheck::new(check_config.clone()))),
                _ => warn!("Unknown health check type: {}", check_config.check_type),
            }
        }

        Self { config, checks }
    }

    /// Perform all configured health checks
    pub async fn check_health(&self) -> Result<HealthStatus, ScalingError> {
        let mut health_status = HealthStatus {
            overall_status: ServiceStatus::Healthy,
            checks: vec![],
            timestamp: chrono::Utc::now(),
        };

        for check in &self.checks {
            let check_result = check.perform_check().await;
            
            match &check_result.status {
                CheckStatus::Healthy => {
                    debug!(check_name = %check_result.name, "Health check passed");
                }
                CheckStatus::Unhealthy(reason) => {
                    warn!(
                        check_name = %check_result.name,
                        reason = %reason,
                        "Health check failed"
                    );
                    
                    if check_result.critical {
                        health_status.overall_status = ServiceStatus::Unhealthy;
                    }
                }
                CheckStatus::Warning(reason) => {
                    warn!(
                        check_name = %check_result.name,
                        reason = %reason,
                        "Health check warning"
                    );
                }
            }

            health_status.checks.push(check_result);
        }

        Ok(health_status)
    }
}

/// Load balancer implementation
#[derive(Clone)]
pub struct LoadBalancer {
    algorithm: LoadBalancingAlgorithm,
    instances: Arc<tokio::sync::RwLock<Vec<ServiceInstance>>>,
    circuit_breaker: Arc<CircuitBreaker>,
    discovery: Arc<ServiceDiscovery>,
}

impl LoadBalancer {
    pub fn new(
        algorithm: LoadBalancingAlgorithm,
        discovery: Arc<ServiceDiscovery>,
        circuit_breaker_config: CircuitBreakerConfig,
    ) -> Self {
        Self {
            algorithm,
            instances: Arc::new(tokio::sync::RwLock::new(vec![])),
            circuit_breaker: Arc::new(CircuitBreaker::new(circuit_breaker_config)),
            discovery,
        }
    }

    /// Select next instance based on load balancing algorithm
    pub async fn select_instance(&self) -> Option<ServiceInstance> {
        let instances = self.instances.read().await;
        
        if instances.is_empty() {
            warn!("No healthy instances available");
            return None;
        }

        match self.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                self.round_robin_select(&instances).await
            }
            LoadBalancingAlgorithm::LeastConnections => {
                self.least_connections_select(&instances).await
            }
            LoadBalancingAlgorithm::IpHash => {
                self.ip_hash_select(&instances).await
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                self.weighted_round_robin_select(&instances).await
            }
        }
    }

    /// Update instance list from service discovery
    pub async fn refresh_instances(&self) -> Result<(), ScalingError> {
        let discovered_instances = self.discovery.discover_instances().await?;
        let healthy_instances: Vec<ServiceInstance> = discovered_instances
            .into_iter()
            .filter(|instance| instance.is_healthy())
            .collect();

        let mut instances = self.instances.write().await;
        *instances = healthy_instances;

        debug!(
            instance_count = instances.len(),
            "Refreshed load balancer instance list"
        );

        Ok(())
    }

    /// Round robin instance selection
    async fn round_robin_select(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        // Implementation would track current index and rotate
        static ROUND_ROBIN_INDEX: std::sync::atomic::AtomicUsize = 
            std::sync::atomic::AtomicUsize::new(0);
            
        let index = ROUND_ROBIN_INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % instances.len();
        instances.get(index).cloned()
    }

    /// Least connections instance selection
    async fn least_connections_select(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        // Find instance with least active connections
        instances.iter()
            .min_by_key(|instance| instance.active_connections)
            .cloned()
    }
}

/// Auto-scaling manager
#[derive(Clone)]
pub struct AutoScaler {
    config: AutoScalingConfig,
    metrics_collector: Arc<MetricsCollector>,
    scaling_actions: Arc<tokio::sync::Mutex<Vec<ScalingAction>>>,
}

impl AutoScaler {
    pub fn new(config: AutoScalingConfig) -> Self {
        Self {
            config,
            metrics_collector: Arc::new(MetricsCollector::new()),
            scaling_actions: Arc::new(tokio::sync::Mutex::new(vec![])),
        }
    }

    /// Start auto-scaling monitoring loop
    pub async fn start_monitoring(&self) {
        let metrics_collector = self.metrics_collector.clone();
        let scaling_actions = self.scaling_actions.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Check every 30 seconds
            
            loop {
                interval.tick().await;
                
                // Collect current metrics
                let current_metrics = metrics_collector.collect_metrics().await;
                
                // Evaluate scaling rules
                for rule in &config.scaling_rules {
                    if let Some(action) = evaluate_scaling_rule(rule, &current_metrics).await {
                        let mut actions = scaling_actions.lock().await;
                        
                        // Check cooldown period
                        if should_apply_scaling_action(&action, &actions) {
                            actions.push(action.clone());
                            
                            info!(
                                action = ?action.action_type,
                                metric = %action.metric_name,
                                threshold = action.threshold,
                                current_value = action.current_value,
                                "Applying auto-scaling action"
                            );
                            
                            apply_scaling_action(&action).await;
                        }
                    }
                }
                
                // Cleanup old scaling actions
                cleanup_old_scaling_actions(&mut scaling_actions.lock().await);
            }
        });
    }
}
```

### Auto-Generated Kubernetes Manifests

```yaml
# Auto-generated Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ app_name }}
  namespace: {{ scaling.service_discovery.namespace }}
  labels:
    app: {{ app_name }}
    version: "{{ app_version }}"
spec:
  replicas: {{ scaling.instances.min_instances }}
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: {{ app_name }}
  template:
    metadata:
      labels:
        app: {{ app_name }}
        version: "{{ app_version }}"
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "{{ server.port }}"
        prometheus.io/path: "/metrics"
    spec:
      containers:
      - name: {{ app_name }}
        image: {{ container_image }}
        ports:
        - containerPort: {{ server.port }}
          name: http
        env:
        - name: ENVIRONMENT
          value: "{{ environment }}"
        - name: K8S_NAMESPACE
          valueFrom:
            fieldRef:
              fieldPath: metadata.namespace
        - name: K8S_POD_NAME
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        
        # Health checks
        livenessProbe:
          httpGet:
            path: {{ health.endpoint }}
            port: http
          initialDelaySeconds: {{ scaling.instances.startup.initial_delay | duration_seconds }}
          periodSeconds: 30
          timeoutSeconds: 5
          failureThreshold: 3
          
        readinessProbe:
          httpGet:
            path: {{ scaling.instances.startup.readiness_probe }}
            port: http
          initialDelaySeconds: 10
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        
        # Resource limits
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        
        # Graceful shutdown
        lifecycle:
          preStop:
            exec:
              command: ["/bin/sh", "-c", "sleep {{ scaling.instances.shutdown.drain_timeout | duration_seconds }}"]
      
      # Graceful termination
      terminationGracePeriodSeconds: {{ scaling.instances.shutdown.graceful_timeout | duration_seconds }}

---
apiVersion: v1
kind: Service
metadata:
  name: {{ app_name }}
  namespace: {{ scaling.service_discovery.namespace }}
  labels:
    app: {{ app_name }}
  {% if scaling.service_discovery.kubernetes.annotations %}
  annotations:
    {% for key, value in scaling.service_discovery.kubernetes.annotations %}
    {{ key }}: "{{ value }}"
    {% endfor %}
  {% endif %}
spec:
  type: {{ scaling.service_discovery.kubernetes.service_type }}
  ports:
  - port: {{ server.port }}
    targetPort: http
    protocol: TCP
    name: http
  selector:
    app: {{ app_name }}

---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: {{ app_name }}-hpa
  namespace: {{ scaling.service_discovery.namespace }}
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: {{ app_name }}
  minReplicas: {{ scaling.instances.min_instances }}
  maxReplicas: {{ scaling.instances.max_instances }}
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: {{ scaling.instances.target_cpu_utilization }}
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: {{ scaling.instances.target_memory_utilization }}
  behavior:
    scaleUp:
      stabilizationWindowSeconds: {{ scaling.instances.scale_up_cooldown | duration_seconds }}
      policies:
      - type: Percent
        value: 100
        periodSeconds: 15
    scaleDown:
      stabilizationWindowSeconds: {{ scaling.instances.scale_down_cooldown | duration_seconds }}
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
```

### Auto-Generated Docker Compose (Development)

```yaml
# Auto-generated docker-compose.yml for development scaling
version: '3.8'

services:
  app:
    build: .
    ports:
      - "{{ server.port }}:{{ server.port }}"
    environment:
      - DATABASE_URL={{ database.url }}
      - ENVIRONMENT=development
      - METRICS_TOKEN=${METRICS_TOKEN}
    volumes:
      - .:/app
    depends_on:
      - postgres
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:{{ server.port }}{{ health.endpoint }}"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    deploy:
      replicas: {{ scaling.instances.min_instances }}
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
        window: 120s

  # Load balancer (for multi-instance development)
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - app
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost/health"]
      interval: 30s
      timeout: 5s
      retries: 3

  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: {{ database.name }}
      POSTGRES_USER: {{ database.user }}
      POSTGRES_PASSWORD: {{ database.password }}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U {{ database.user }}"]
      interval: 30s
      timeout: 10s
      retries: 5

  redis:
    image: redis:7-alpine
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 3s
      retries: 5

volumes:
  postgres_data:
```

## CLI Integration

### Scaling Management Commands

```bash
# Generate scaling token
rustform scaling generate-token --scopes "scaling:read,scaling:write" --expires-in "7d"

# Check scaling status
rustform scaling status

# Manual scaling operations
rustform scaling scale-up --instances 5
rustform scaling scale-down --instances 2

# Health check management
rustform scaling health-check --name "database" --enable
rustform scaling health-check --name "external_api" --disable

# View scaling metrics
rustform scaling metrics --since "1h"

# Test load balancing
rustform scaling test-lb --requests 100 --concurrent 10
```

## Benefits

### Production-Ready Scaling
- **Zero-Configuration**: Automatic scaling with simple YAML configuration
- **Cloud Native**: Kubernetes and container orchestration ready
- **High Availability**: Multi-instance deployment with health monitoring
- **Graceful Operations**: Proper startup, shutdown, and deployment procedures

### Observability Integration
- **Scaling Metrics**: Automatic metrics for scaling decisions
- **Health Monitoring**: Comprehensive health checks and alerts
- **Performance Tracking**: Request rates, response times, and resource usage
- **Debug Tools**: Distributed tracing across scaled instances

### Developer Experience
- **Local Development**: Docker Compose support for multi-instance testing
- **Easy Configuration**: YAML-driven scaling configuration
- **Testing Tools**: Load balancing and scaling simulation
- **Monitoring Integration**: Grafana dashboards and Prometheus alerts

This horizontal scaling framework enables Rust-form applications to scale from single instances to large distributed deployments with minimal configuration and maximum reliability.