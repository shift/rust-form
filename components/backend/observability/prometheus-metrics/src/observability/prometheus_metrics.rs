use axum::{
    extract::{Request, State},
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::{IntoResponse, Response},
};
use prometheus::{Encoder, TextEncoder, Registry};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{
    config::MetricsConfig,
    error::MetricsError,
};

/// Metrics endpoint handler with authentication and rate limiting
#[derive(Clone)]
pub struct PrometheusMetricsHandler {
    registry: Arc<Registry>,
    config: MetricsConfig,
    rate_limiter: Arc<RateLimiter>,
    token_validator: Arc<TokenValidator>,
}

impl PrometheusMetricsHandler {
    pub fn new(
        registry: Arc<Registry>,
        config: MetricsConfig,
    ) -> Self {
        let rate_limiter = Arc::new(RateLimiter::new(
            config.rate_limit_requests_per_minute,
            config.rate_limit_burst_size,
        ));
        
        let token_validator = Arc::new(TokenValidator::new(config.clone()));

        Self {
            registry,
            config,
            rate_limiter,
            token_validator,
        }
    }

    /// Handle metrics endpoint request
    pub async fn handle_metrics(
        State(handler): State<PrometheusMetricsHandler>,
    ) -> Result<impl IntoResponse, StatusCode> {
        // Collect and encode metrics
        let encoder = TextEncoder::new();
        let metric_families = handler.registry.gather();

        match encoder.encode_to_string(&metric_families) {
            Ok(metrics_output) => {
                info!(
                    metrics_count = metric_families.len(),
                    output_size = metrics_output.len(),
                    "Metrics successfully collected and encoded"
                );

                Ok((
                    StatusCode::OK,
                    [("content-type", "text/plain; version=0.0.4")],
                    metrics_output,
                ))
            }
            Err(e) => {
                error!(error = %e, "Failed to encode metrics");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Authentication and rate limiting middleware
    pub async fn auth_middleware(
        State(handler): State<PrometheusMetricsHandler>,
        headers: HeaderMap,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let client_ip = extract_client_ip(&request);

        // Check rate limiting first
        if !handler.rate_limiter.check_rate_limit(&client_ip).await {
            warn!(
                client_ip = %client_ip,
                "Rate limit exceeded for metrics endpoint"
            );
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        // Skip authentication if disabled
        if !handler.config.authentication_enabled {
            return Ok(next.run(request).await);
        }

        // Extract and validate authentication token
        let auth_result = match handler.config.auth_method.as_str() {
            "bearer_token" => handler.validate_bearer_token(&headers).await,
            "api_key" => handler.validate_api_key(&headers).await,
            "jwt" => handler.validate_jwt(&headers).await,
            _ => {
                error!("Unknown authentication method: {}", handler.config.auth_method);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        match auth_result {
            Ok(token_info) => {
                info!(
                    client_ip = %client_ip,
                    token_id = %token_info.token_id,
                    scopes = ?token_info.scopes,
                    "Successful metrics authentication"
                );

                // Check if token has required scopes
                if !token_info.scopes.contains("metrics:read") {
                    warn!(
                        token_id = %token_info.token_id,
                        scopes = ?token_info.scopes,
                        "Token lacks required scope for metrics access"
                    );
                    return Err(StatusCode::FORBIDDEN);
                }

                Ok(next.run(request).await)
            }
            Err(e) => {
                warn!(
                    client_ip = %client_ip,
                    error = %e,
                    "Authentication failed for metrics endpoint"
                );
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }

    /// Validate bearer token
    async fn validate_bearer_token(&self, headers: &HeaderMap) -> Result<TokenInfo, MetricsError> {
        let auth_header = headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(MetricsError::MissingAuthHeader)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(MetricsError::InvalidAuthFormat);
        }

        let token = &auth_header[7..];
        self.token_validator.validate_token(token).await
    }

    /// Validate API key
    async fn validate_api_key(&self, headers: &HeaderMap) -> Result<TokenInfo, MetricsError> {
        let api_key = headers
            .get("x-api-key")
            .and_then(|h| h.to_str().ok())
            .ok_or(MetricsError::MissingApiKey)?;

        self.token_validator.validate_api_key(api_key).await
    }

    /// Validate JWT token
    async fn validate_jwt(&self, headers: &HeaderMap) -> Result<TokenInfo, MetricsError> {
        let auth_header = headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(MetricsError::MissingAuthHeader)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(MetricsError::InvalidAuthFormat);
        }

        let jwt = &auth_header[7..];
        self.token_validator.validate_jwt(jwt).await
    }
}

/// Token validation service
#[derive(Clone)]
pub struct TokenValidator {
    config: MetricsConfig,
    valid_tokens: Arc<RwLock<HashMap<String, TokenInfo>>>,
}

impl TokenValidator {
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            valid_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Validate bearer token
    pub async fn validate_token(&self, token: &str) -> Result<TokenInfo, MetricsError> {
        // First check environment variable token
        if let Ok(env_token) = std::env::var(&self.config.token_env_var) {
            if token == env_token {
                return Ok(TokenInfo {
                    token_id: "env_token".to_string(),
                    scopes: vec!["metrics:read".to_string()],
                    expires_at: None, // Environment tokens don't expire
                    created_at: Utc::now(),
                });
            }
        }

        // Check stored tokens
        let tokens = self.valid_tokens.read().await;
        if let Some(token_info) = tokens.get(token) {
            // Check if token has expired
            if let Some(expires_at) = token_info.expires_at {
                if expires_at < Utc::now() {
                    return Err(MetricsError::TokenExpired);
                }
            }
            return Ok(token_info.clone());
        }

        Err(MetricsError::InvalidToken)
    }

    /// Validate API key
    pub async fn validate_api_key(&self, api_key: &str) -> Result<TokenInfo, MetricsError> {
        // API keys can be implemented similarly to bearer tokens
        self.validate_token(api_key).await
    }

    /// Validate JWT token
    pub async fn validate_jwt(&self, jwt: &str) -> Result<TokenInfo, MetricsError> {
        use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

        // Get JWT secret from environment
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| MetricsError::JwtSecretMissing)?;

        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<JwtClaims>(jwt, &decoding_key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;
                
                // Check if token has expired
                if claims.exp < Utc::now().timestamp() as usize {
                    return Err(MetricsError::TokenExpired);
                }

                // Check if token has required scopes
                if !claims.scopes.contains(&"metrics:read".to_string()) {
                    return Err(MetricsError::InsufficientScope);
                }

                Ok(TokenInfo {
                    token_id: claims.sub,
                    scopes: claims.scopes,
                    expires_at: Some(DateTime::from_timestamp(claims.exp as i64, 0).unwrap().and_utc()),
                    created_at: DateTime::from_timestamp(claims.iat as i64, 0).unwrap().and_utc(),
                })
            }
            Err(e) => {
                error!(error = %e, "JWT validation failed");
                Err(MetricsError::InvalidJwt)
            }
        }
    }

    /// Generate a new bearer token
    pub async fn generate_token(
        &self,
        scopes: Vec<String>,
        expires_in: Option<Duration>,
    ) -> Result<String, MetricsError> {
        let token = generate_secure_token();
        let expires_at = expires_in.map(|duration| Utc::now() + duration);

        let token_info = TokenInfo {
            token_id: uuid::Uuid::new_v4().to_string(),
            scopes,
            expires_at,
            created_at: Utc::now(),
        };

        let mut tokens = self.valid_tokens.write().await;
        tokens.insert(token.clone(), token_info);

        Ok(token)
    }

    /// Revoke a token
    pub async fn revoke_token(&self, token: &str) -> Result<(), MetricsError> {
        let mut tokens = self.valid_tokens.write().await;
        tokens.remove(token);
        Ok(())
    }

    /// List all active tokens (for management purposes)
    pub async fn list_tokens(&self) -> Vec<TokenInfo> {
        let tokens = self.valid_tokens.read().await;
        tokens.values().cloned().collect()
    }
}

/// Rate limiter for metrics endpoint
#[derive(Clone)]
pub struct RateLimiter {
    requests_per_minute: u32,
    burst_size: u32,
    client_requests: Arc<RwLock<HashMap<String, ClientRateLimit>>>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32, burst_size: u32) -> Self {
        Self {
            requests_per_minute,
            burst_size,
            client_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if client is within rate limits
    pub async fn check_rate_limit(&self, client_ip: &str) -> bool {
        let mut clients = self.client_requests.write().await;
        let now = Instant::now();

        let client_limit = clients
            .entry(client_ip.to_string())
            .or_insert_with(|| ClientRateLimit::new(now));

        client_limit.check_and_update(
            now,
            self.requests_per_minute,
            self.burst_size,
        )
    }

    /// Cleanup expired rate limit entries
    pub async fn cleanup_expired(&self) {
        let mut clients = self.client_requests.write().await;
        let now = Instant::now();
        let cleanup_threshold = Duration::from_secs(300); // 5 minutes

        clients.retain(|_, limit| {
            now.duration_since(limit.last_request) < cleanup_threshold
        });
    }
}

/// Per-client rate limiting state
#[derive(Debug)]
struct ClientRateLimit {
    requests_this_minute: u32,
    burst_tokens: u32,
    minute_start: Instant,
    last_request: Instant,
}

impl ClientRateLimit {
    fn new(now: Instant) -> Self {
        Self {
            requests_this_minute: 0,
            burst_tokens: 0,
            minute_start: now,
            last_request: now,
        }
    }

    fn check_and_update(
        &mut self,
        now: Instant,
        requests_per_minute: u32,
        burst_size: u32,
    ) -> bool {
        self.last_request = now;

        // Reset minute counter if needed
        if now.duration_since(self.minute_start) >= Duration::from_secs(60) {
            self.requests_this_minute = 0;
            self.minute_start = now;
            self.burst_tokens = burst_size;
        }

        // Check burst limit first
        if self.burst_tokens > 0 {
            self.burst_tokens -= 1;
            self.requests_this_minute += 1;
            return true;
        }

        // Check per-minute limit
        if self.requests_this_minute < requests_per_minute {
            self.requests_this_minute += 1;
            true
        } else {
            false
        }
    }
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_id: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    scopes: Vec<String>,
    exp: usize,
    iat: usize,
}

/// Generate a cryptographically secure token
fn generate_secure_token() -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(uuid::Uuid::new_v4().as_bytes());
    hasher.update(Utc::now().timestamp().to_be_bytes());
    
    let hash = hasher.finalize();
    base64::encode_config(hash, base64::URL_SAFE_NO_PAD)
}

/// Extract client IP from request
fn extract_client_ip(request: &Request) -> String {
    // Check for forwarded headers first
    if let Some(forwarded_for) = request.headers().get("x-forwarded-for") {
        if let Ok(header_str) = forwarded_for.to_str() {
            if let Some(first_ip) = header_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    if let Some(real_ip) = request.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Fallback to connection remote address
    "unknown".to_string()
}

/// Metrics authentication and authorization errors
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    #[error("Missing authorization header")]
    MissingAuthHeader,
    
    #[error("Invalid authorization format")]
    InvalidAuthFormat,
    
    #[error("Missing API key")]
    MissingApiKey,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token has expired")]
    TokenExpired,
    
    #[error("JWT secret not configured")]
    JwtSecretMissing,
    
    #[error("Invalid JWT token")]
    InvalidJwt,
    
    #[error("Insufficient scope for operation")]
    InsufficientScope,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(60, 10);
        let client_ip = "127.0.0.1";

        // Should allow burst requests
        for _ in 0..10 {
            assert!(rate_limiter.check_rate_limit(client_ip).await);
        }

        // Should deny additional requests
        assert!(!rate_limiter.check_rate_limit(client_ip).await);
    }

    #[tokio::test]
    async fn test_token_generation_and_validation() {
        let config = MetricsConfig::default();
        let validator = TokenValidator::new(config);

        let token = validator
            .generate_token(
                vec!["metrics:read".to_string()],
                Some(Duration::from_secs(3600)),
            )
            .await
            .unwrap();

        let token_info = validator.validate_token(&token).await.unwrap();
        assert!(token_info.scopes.contains(&"metrics:read".to_string()));
    }

    #[tokio::test]
    async fn test_token_expiration() {
        let config = MetricsConfig::default();
        let validator = TokenValidator::new(config);

        let token = validator
            .generate_token(
                vec!["metrics:read".to_string()],
                Some(Duration::from_millis(100)),
            )
            .await
            .unwrap();

        // Token should be valid initially
        assert!(validator.validate_token(&token).await.is_ok());

        // Wait for expiration
        sleep(Duration::from_millis(200)).await;

        // Token should be expired now
        assert!(matches!(
            validator.validate_token(&token).await,
            Err(MetricsError::TokenExpired)
        ));
    }
}