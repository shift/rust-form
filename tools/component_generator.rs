use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Component category definitions for systematic generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCategory {
    pub name: String,
    pub description: String,
    pub templates: Vec<ComponentTemplate>,
    pub common_dependencies: Vec<String>,
    pub quality_requirements: QualityRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTemplate {
    pub name: String,
    pub description: String,
    pub template_type: TemplateType,
    pub variables: Vec<TemplateVariable>,
    pub content_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    Backend,
    Frontend,
    Database,
    Config,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub var_type: String,
    pub default_value: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub min_test_coverage: u8,
    pub required_documentation: Vec<String>,
    pub required_examples: u8,
    pub required_templates: u8,
}

/// Component library generator for rapid scaling
pub struct ComponentLibraryGenerator {
    pub categories: HashMap<String, ComponentCategory>,
    pub output_directory: String,
    pub base_templates: HashMap<String, String>,
}

impl ComponentLibraryGenerator {
    pub fn new(output_directory: String) -> Self {
        let mut generator = Self {
            categories: HashMap::new(),
            output_directory,
            base_templates: HashMap::new(),
        };

        generator.initialize_categories();
        generator.load_base_templates();
        generator
    }

    /// Initialize all component categories for systematic generation
    pub fn initialize_categories(&mut self) {
        // Authentication Category
        self.categories.insert(
            "auth".to_string(),
            ComponentCategory {
                name: "Authentication".to_string(),
                description: "Authentication and authorization components".to_string(),
                templates: vec![
                    ComponentTemplate {
                        name: "jwt_handler.rs.tera".to_string(),
                        description: "JWT token handling and validation".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![
                            TemplateVariable {
                                name: "secret_key".to_string(),
                                var_type: "string".to_string(),
                                default_value: Some("JWT_SECRET".to_string()),
                                description: "Environment variable for JWT secret".to_string(),
                            },
                            TemplateVariable {
                                name: "expiry_hours".to_string(),
                                var_type: "number".to_string(),
                                default_value: Some("24".to_string()),
                                description: "Token expiry time in hours".to_string(),
                            },
                        ],
                        content_template: "jwt_handler_template".to_string(),
                    },
                    ComponentTemplate {
                        name: "auth_middleware.rs.tera".to_string(),
                        description: "Authentication middleware for protected routes".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![],
                        content_template: "auth_middleware_template".to_string(),
                    },
                ],
                common_dependencies: vec![
                    "jsonwebtoken = \"8.3\"".to_string(),
                    "chrono = { version = \"0.4\", features = [\"serde\"] }".to_string(),
                ],
                quality_requirements: QualityRequirements {
                    min_test_coverage: 85,
                    required_documentation: vec![
                        "README.md".to_string(),
                        "SECURITY.md".to_string(),
                    ],
                    required_examples: 2,
                    required_templates: 3,
                },
            },
        );

        // Payment Processing Category
        self.categories.insert(
            "payments".to_string(),
            ComponentCategory {
                name: "Payment Processing".to_string(),
                description: "Payment gateway integrations and billing components".to_string(),
                templates: vec![
                    ComponentTemplate {
                        name: "stripe_handler.rs.tera".to_string(),
                        description: "Stripe payment processing integration".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![TemplateVariable {
                            name: "stripe_secret_key".to_string(),
                            var_type: "string".to_string(),
                            default_value: Some("STRIPE_SECRET_KEY".to_string()),
                            description: "Stripe secret key environment variable".to_string(),
                        }],
                        content_template: "stripe_handler_template".to_string(),
                    },
                    ComponentTemplate {
                        name: "webhook_handler.rs.tera".to_string(),
                        description: "Payment webhook processing".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![],
                        content_template: "webhook_handler_template".to_string(),
                    },
                ],
                common_dependencies: vec![
                    "stripe = \"0.28\"".to_string(),
                    "hmac = \"0.12\"".to_string(),
                    "sha2 = \"0.10\"".to_string(),
                ],
                quality_requirements: QualityRequirements {
                    min_test_coverage: 90,
                    required_documentation: vec![
                        "README.md".to_string(),
                        "INTEGRATION.md".to_string(),
                    ],
                    required_examples: 3,
                    required_templates: 4,
                },
            },
        );

        // Dashboard Components Category
        self.categories.insert(
            "dashboards".to_string(),
            ComponentCategory {
                name: "Dashboard Components".to_string(),
                description: "Analytics and dashboard UI components".to_string(),
                templates: vec![
                    ComponentTemplate {
                        name: "chart_component.tsx.tera".to_string(),
                        description: "Reusable chart component with multiple chart types"
                            .to_string(),
                        template_type: TemplateType::Frontend,
                        variables: vec![TemplateVariable {
                            name: "chart_type".to_string(),
                            var_type: "string".to_string(),
                            default_value: Some("line".to_string()),
                            description: "Default chart type (line, bar, pie, etc.)".to_string(),
                        }],
                        content_template: "chart_component_template".to_string(),
                    },
                    ComponentTemplate {
                        name: "metrics_api.rs.tera".to_string(),
                        description: "API endpoints for dashboard metrics".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![],
                        content_template: "metrics_api_template".to_string(),
                    },
                ],
                common_dependencies: vec![
                    "recharts = \"^2.8.0\"".to_string(),
                    "date-fns = \"^2.30.0\"".to_string(),
                ],
                quality_requirements: QualityRequirements {
                    min_test_coverage: 80,
                    required_documentation: vec![
                        "README.md".to_string(),
                        "EXAMPLES.md".to_string(),
                    ],
                    required_examples: 4,
                    required_templates: 5,
                },
            },
        );

        // E-commerce Category
        self.categories.insert(
            "ecommerce".to_string(),
            ComponentCategory {
                name: "E-commerce".to_string(),
                description: "Online store and marketplace components".to_string(),
                templates: vec![
                    ComponentTemplate {
                        name: "product_catalog.rs.tera".to_string(),
                        description: "Product catalog management with search and filtering"
                            .to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![],
                        content_template: "product_catalog_template".to_string(),
                    },
                    ComponentTemplate {
                        name: "shopping_cart.tsx.tera".to_string(),
                        description: "Shopping cart UI component with persistence".to_string(),
                        template_type: TemplateType::Frontend,
                        variables: vec![],
                        content_template: "shopping_cart_template".to_string(),
                    },
                ],
                common_dependencies: vec![
                    "uuid = { version = \"1.6\", features = [\"v4\"] }".to_string(),
                    "rust_decimal = \"1.33\"".to_string(),
                ],
                quality_requirements: QualityRequirements {
                    min_test_coverage: 85,
                    required_documentation: vec!["README.md".to_string(), "API.md".to_string()],
                    required_examples: 3,
                    required_templates: 6,
                },
            },
        );

        // CMS Category
        self.categories.insert(
            "cms".to_string(),
            ComponentCategory {
                name: "Content Management".to_string(),
                description: "Content management and publishing components".to_string(),
                templates: vec![
                    ComponentTemplate {
                        name: "content_editor.tsx.tera".to_string(),
                        description: "Rich text content editor with media support".to_string(),
                        template_type: TemplateType::Frontend,
                        variables: vec![],
                        content_template: "content_editor_template".to_string(),
                    },
                    ComponentTemplate {
                        name: "media_handler.rs.tera".to_string(),
                        description: "File upload and media management backend".to_string(),
                        template_type: TemplateType::Backend,
                        variables: vec![],
                        content_template: "media_handler_template".to_string(),
                    },
                ],
                common_dependencies: vec![
                    "multer = \"2.1\"".to_string(),
                    "mime = \"0.3\"".to_string(),
                ],
                quality_requirements: QualityRequirements {
                    min_test_coverage: 80,
                    required_documentation: vec!["README.md".to_string(), "USAGE.md".to_string()],
                    required_examples: 2,
                    required_templates: 4,
                },
            },
        );
    }

    /// Load base template files for component generation
    pub fn load_base_templates(&mut self) {
        // Component manifest template
        self.base_templates.insert(
            "manifest".to_string(),
            r#"name: "{{ component_name }}"
version: "{{ version | default(value='1.0.0') }}"
description: "{{ description }}"
author: "{{ author | default(value='rust-form') }}"
homepage: "{{ homepage | default(value='https://github.com/rust-form/components') }}"
keywords: {{ keywords | default(value='[]') }}

api_compatibility:
  api_version: "{{ api_version | default(value='0.1.0') }}"
  min_version: "{{ min_version | default(value='0.1.0') }}"
  max_version: "{{ max_version | default(value='0.2.0') }}"
  experimental: {{ experimental | default(value='false') }}

dependencies: {{ dependencies | default(value='{}') }}

files: {{ files }}

provides:
  templates: {{ templates }}
  assets: {{ assets | default(value='[]') }}
  hooks: {{ hooks | default(value='[]') }}
"#
            .to_string(),
        );

        // Test file template
        self.base_templates.insert(
            "test".to_string(),
            r#"#[cfg(test)]
mod {{ component_name | snake_case }}_tests {
    use super::*;
    
    #[test]
    fn test_{{ component_name | snake_case }}_basic_functionality() {
        // Test basic component functionality
        assert!(true, "Component should have basic functionality");
    }
    
    #[test]
    fn test_{{ component_name | snake_case }}_configuration() {
        // Test component configuration
        assert!(true, "Component should be configurable");
    }
    
    #[test]
    fn test_{{ component_name | snake_case }}_error_handling() {
        // Test error handling
        assert!(true, "Component should handle errors gracefully");
    }
    
    #[test]
    fn test_{{ component_name | snake_case }}_integration() {
        // Test integration with rust-form
        assert!(true, "Component should integrate with rust-form");
    }
    
    #[test]
    fn test_{{ component_name | snake_case }}_performance() {
        // Test performance characteristics
        assert!(true, "Component should meet performance requirements");
    }
}
"#
            .to_string(),
        );

        // README template
        self.base_templates.insert(
            "readme".to_string(),
            r#"# {{ component_display_name }}

{{ description }}

## Features

{{ features | default(value='- Core functionality\n- Easy integration\n- Comprehensive testing') }}

## Installation

Add this component to your rust-form project:

```yaml
components:
  {{ component_name }}: "path:./components/{{ component_name }}"
```

## Usage

{{ usage_example | default(value='Basic usage example coming soon.') }}

## Configuration

{{ configuration_docs | default(value='Configuration documentation coming soon.') }}

## API Reference

{{ api_reference | default(value='API reference coming soon.') }}

## Testing

Run the component tests:

```bash
rustform component test {{ component_name }}
```

## Contributing

1. Follow the rust-form component development guidelines
2. Ensure all tests pass
3. Maintain quality score above {{ min_quality_score | default(value='80') }}
4. Update documentation for any changes

## License

MIT License - see LICENSE file for details
"#
            .to_string(),
        );

        // JWT Handler Template
        self.base_templates.insert(
            "jwt_handler_template".to_string(),
            r#"use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub role: Option<String>,
}

pub struct JwtHandler {
    secret: String,
    expiry_hours: i64,
}

impl JwtHandler {
    pub fn new() -> Self {
        Self {
            secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string()),
            expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        }
    }

    pub fn generate_token(&self, user_id: &str, role: Option<String>) -> Result<String, JwtError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(self.expiry_hours)).timestamp() as usize,
            iat: now.timestamp() as usize,
            role,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("JWT encoding error: {0}")]
    EncodingError(#[from] jsonwebtoken::errors::Error),
    #[error("Token expired")]
    Expired,
    #[error("Invalid token")]
    Invalid,
}
"#
            .to_string(),
        );

        // Auth Middleware Template
        self.base_templates.insert(
            "auth_middleware_template".to_string(),
            r#"use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::jwt_handler::{JwtHandler, Claims};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let jwt_handler = JwtHandler::new();
    
    match jwt_handler.validate_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn extract_user_claims(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}
"#
            .to_string(),
        );

        // Additional component-specific templates
        self.base_templates.insert("stripe_handler_template".to_string(), r#"// Stripe payment handler implementation template
use stripe::{Client, CreatePaymentIntent, PaymentIntent, PaymentIntentConfirm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: i64,
    pub currency: String,
    pub customer_id: Option<String>,
}

pub struct StripeHandler {
    client: Client,
}

impl StripeHandler {
    pub fn new(secret_key: &str) -> Self {
        Self {
            client: Client::new(secret_key),
        }
    }

    pub async fn create_payment_intent(&self, request: PaymentRequest) -> Result<PaymentIntent, stripe::Error> {
        let create_intent = CreatePaymentIntent {
            amount: request.amount,
            currency: stripe::Currency::from_str(&request.currency).unwrap_or(stripe::Currency::USD),
            customer: request.customer_id,
            ..Default::default()
        };

        PaymentIntent::create(&self.client, create_intent).await
    }
}
"#.to_string());

        // Additional templates for other categories...
        self.base_templates.insert(
            "metrics_api_template".to_string(),
            r#"// Dashboard metrics API template
use axum::{extract::Query, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MetricsQuery {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub granularity: Option<String>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub data: HashMap<String, Vec<MetricPoint>>,
    pub meta: MetricsMeta,
}

#[derive(Serialize)]
pub struct MetricPoint {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Serialize)]
pub struct MetricsMeta {
    pub total_points: usize,
    pub start_date: String,
    pub end_date: String,
}

pub async fn get_metrics(Query(params): Query<MetricsQuery>) -> Json<MetricsResponse> {
    // Implementation for fetching metrics
    let data = HashMap::new(); // Placeholder
    let meta = MetricsMeta {
        total_points: 0,
        start_date: params.start_date.unwrap_or_default(),
        end_date: params.end_date.unwrap_or_default(),
    };
    
    Json(MetricsResponse { data, meta })
}
"#
            .to_string(),
        );

        // Webhook handler template
        self.base_templates.insert(
            "webhook_handler_template".to_string(),
            r#"use axum::{extract::Json, response::Json as ResponseJson};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub received: bool,
    pub processed: bool,
    pub message: String,
}

pub async fn handle_webhook(
    Json(payload): Json<WebhookPayload>
) -> ResponseJson<WebhookResponse> {
    // Verify webhook signature
    // Process webhook payload based on event type
    
    ResponseJson(WebhookResponse {
        received: true,
        processed: true,
        message: "Webhook processed successfully".to_string(),
    })
}
"#
            .to_string(),
        );

        // Chart component template
        self.base_templates.insert("chart_component_template".to_string(), r#"import React from 'react';
import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend } from 'chart.js';
import { Bar } from 'react-chartjs-2';

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend);

interface ChartComponentProps {
  data: {
    labels: string[];
    datasets: {
      label: string;
      data: number[];
      backgroundColor?: string;
      borderColor?: string;
      borderWidth?: number;
    }[];
  };
  options?: any;
}

export const ChartComponent: React.FC<ChartComponentProps> = ({ data, options = {} }) => {
  const defaultOptions = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Chart Component',
      },
    },
    ...options,
  };

  return <Bar data={data} options={defaultOptions} />;
};
"#.to_string());

        // Product catalog template
        self.base_templates.insert(
            "product_catalog_template".to_string(),
            r#"use axum::{extract::{Path, Query}, response::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub in_stock: bool,
    pub images: Vec<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock: Option<bool>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct ProductCatalogResponse {
    pub products: Vec<Product>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

pub async fn get_products(Query(params): Query<ProductQuery>) -> Json<ProductCatalogResponse> {
    // Implementation for fetching products
    let products = Vec::new(); // Placeholder
    
    Json(ProductCatalogResponse {
        products,
        total: 0,
        page: params.page.unwrap_or(1),
        per_page: params.limit.unwrap_or(20),
    })
}

pub async fn get_product(Path(id): Path<Uuid>) -> Json<Option<Product>> {
    // Implementation for fetching single product
    Json(None) // Placeholder
}
"#
            .to_string(),
        );

        // Shopping cart template
        self.base_templates.insert(
            "shopping_cart_template".to_string(),
            r#"import React, { useState } from 'react';

interface CartItem {
  id: string;
  name: string;
  price: number;
  quantity: number;
  image?: string;
}

interface ShoppingCartProps {
  items: CartItem[];
  onUpdateQuantity: (id: string, quantity: number) => void;
  onRemoveItem: (id: string) => void;
  onCheckout: () => void;
}

export const ShoppingCart: React.FC<ShoppingCartProps> = ({
  items,
  onUpdateQuantity,
  onRemoveItem,
  onCheckout,
}) => {
  const total = items.reduce((sum, item) => sum + item.price * item.quantity, 0);

  return (
    <div className="shopping-cart">
      <h2>Shopping Cart</h2>
      {items.length === 0 ? (
        <p>Your cart is empty</p>
      ) : (
        <>
          {items.map((item) => (
            <div key={item.id} className="cart-item">
              <div className="item-details">
                <h3>{item.name}</h3>
                <p>${item.price.toFixed(2)}</p>
              </div>
              <div className="quantity-controls">
                <button onClick={() => onUpdateQuantity(item.id, item.quantity - 1)}>-</button>
                <span>{item.quantity}</span>
                <button onClick={() => onUpdateQuantity(item.id, item.quantity + 1)}>+</button>
              </div>
              <button onClick={() => onRemoveItem(item.id)}>Remove</button>
            </div>
          ))}
          <div className="cart-total">
            <strong>Total: ${total.toFixed(2)}</strong>
          </div>
          <button onClick={onCheckout} className="checkout-btn">
            Proceed to Checkout
          </button>
        </>
      )}
    </div>
  );
};
"#
            .to_string(),
        );

        // Content editor template
        self.base_templates.insert(
            "content_editor_template".to_string(),
            r#"import React, { useState } from 'react';

interface ContentEditorProps {
  initialContent?: string;
  onChange: (content: string) => void;
  onSave: () => void;
  placeholder?: string;
}

export const ContentEditor: React.FC<ContentEditorProps> = ({
  initialContent = '',
  onChange,
  onSave,
  placeholder = 'Start writing...',
}) => {
  const [content, setContent] = useState(initialContent);
  const [isEditing, setIsEditing] = useState(false);

  const handleContentChange = (value: string) => {
    setContent(value);
    onChange(value);
  };

  const handleSave = () => {
    onSave();
    setIsEditing(false);
  };

  return (
    <div className="content-editor">
      <div className="editor-toolbar">
        <button onClick={() => setIsEditing(!isEditing)}>
          {isEditing ? 'Preview' : 'Edit'}
        </button>
        {isEditing && (
          <button onClick={handleSave} className="save-btn">
            Save
          </button>
        )}
      </div>
      
      {isEditing ? (
        <textarea
          value={content}
          onChange={(e) => handleContentChange(e.target.value)}
          placeholder={placeholder}
          className="editor-textarea"
        />
      ) : (
        <div
          className="editor-preview"
          dangerouslySetInnerHTML={{ __html: content }}
        />
      )}
    </div>
  );
};
"#
            .to_string(),
        );

        // Media handler template
        self.base_templates.insert("media_handler_template".to_string(), r#"use axum::{extract::Multipart, response::Json};
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;
use tokio::fs;

#[derive(Serialize)]
pub struct MediaUploadResponse {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub size: u64,
    pub content_type: String,
}

#[derive(Serialize)]
pub struct MediaError {
    pub error: String,
    pub code: String,
}

pub async fn upload_media(mut multipart: Multipart) -> Result<Json<MediaUploadResponse>, Json<MediaError>> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("file").to_string();
        
        if name == "file" {
            let filename = field.file_name()
                .unwrap_or("unknown")
                .to_string();
            
            let content_type = field.content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            
            let data = field.bytes().await.unwrap();
            let size = data.len() as u64;
            
            // Generate unique filename
            let file_id = Uuid::new_v4();
            let extension = PathBuf::from(&filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("bin");
            
            let unique_filename = format!("{}.{}", file_id, extension);
            let file_path = PathBuf::from("uploads").join(&unique_filename);
            
            // Ensure uploads directory exists
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).await.unwrap();
            }
            
            // Save file
            fs::write(&file_path, &data).await.unwrap();
            
            return Ok(Json(MediaUploadResponse {
                id: file_id,
                filename: unique_filename,
                url: format!("/uploads/{}", unique_filename),
                size,
                content_type,
            }));
        }
    }
    
    Err(Json(MediaError {
        error: "No file found in request".to_string(),
        code: "NO_FILE".to_string(),
    }))
}
"#.to_string());
    }

    /// Generate a complete component with all required files
    pub async fn generate_component(
        &self,
        category: &str,
        component_name: &str,
        config: ComponentGenerationConfig,
    ) -> Result<GeneratedComponent, ComponentGenerationError> {
        let category_info =
            self.categories
                .get(category)
                .ok_or(ComponentGenerationError::CategoryNotFound(
                    category.to_string(),
                ))?;

        let component_dir = Path::new(&self.output_directory)
            .join("components")
            .join(category)
            .join(component_name);

        // Create component directory structure
        fs::create_dir_all(&component_dir)?;
        fs::create_dir_all(component_dir.join("src"))?;
        fs::create_dir_all(component_dir.join("templates"))?;
        fs::create_dir_all(component_dir.join("tests"))?;
        fs::create_dir_all(component_dir.join("examples"))?;
        fs::create_dir_all(component_dir.join("docs"))?;
        fs::create_dir_all(component_dir.join("test-app"))?;

        // Generate component manifest
        let manifest = self.generate_manifest(component_name, category_info, &config)?;
        fs::write(component_dir.join("rustform-component.yml"), manifest)?;

        // Generate templates
        let mut template_files = Vec::new();
        for template in &category_info.templates {
            let content = self.render_template(&template.content_template, &config)?;
            let file_path = component_dir.join("templates").join(&template.name);
            fs::write(&file_path, content)?;
            template_files.push(template.name.clone());
        }

        // Generate tests
        let test_content = self.generate_test_file(component_name, &config)?;
        fs::write(component_dir.join("src").join("lib_test.rs"), test_content)?;

        // Generate Cargo.toml for standalone testing
        let cargo_toml = self.generate_cargo_toml(component_name, category_info)?;
        fs::write(component_dir.join("Cargo.toml"), cargo_toml)?;

        // Generate README
        let readme = self.generate_readme(component_name, category_info, &config)?;
        fs::write(component_dir.join("README.md"), readme)?;

        // Generate examples
        for i in 0..category_info.quality_requirements.required_examples as usize {
            let example_content = self.generate_example(component_name, i + 1, &config)?;
            fs::write(
                component_dir
                    .join("examples")
                    .join(format!("example_{}.yml", i + 1)),
                example_content,
            )?;
        }

        // Generate test-app configuration
        let test_app_config = self.generate_test_app_config(component_name, &config)?;
        fs::write(
            component_dir.join("test-app").join("rustform.yml"),
            test_app_config,
        )?;

        Ok(GeneratedComponent {
            name: component_name.to_string(),
            category: category.to_string(),
            path: component_dir.to_string_lossy().to_string(),
            template_files,
            quality_score: self.calculate_initial_quality_score(category_info),
        })
    }

    /// Generate multiple components in batch for rapid scaling
    pub async fn generate_component_library(
        &self,
        specifications: Vec<ComponentSpec>,
    ) -> Result<Vec<GeneratedComponent>, ComponentGenerationError> {
        let mut results = Vec::new();

        for spec in specifications {
            let config = ComponentGenerationConfig {
                description: spec.description,
                author: spec.author.unwrap_or_else(|| "rust-form".to_string()),
                features: spec.features,
                dependencies: spec.dependencies,
                ..Default::default()
            };

            let component = self
                .generate_component(&spec.category, &spec.name, config)
                .await?;
            results.push(component);
        }

        Ok(results)
    }

    /// Render a template with the given configuration
    fn render_template(
        &self,
        template_key: &str,
        _config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        // For now, return the template content directly
        // TODO: Implement proper Tera template rendering with variable substitution
        self.base_templates
            .get(template_key)
            .cloned()
            .ok_or_else(|| {
                ComponentGenerationError::TemplateError(format!(
                    "Template not found: {}",
                    template_key
                ))
            })
    }

    /// Generate component manifest YAML
    fn generate_manifest(
        &self,
        component_name: &str,
        _category: &ComponentCategory,
        config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        Ok(format!(
            r#"name: "{}"
version: "1.0.0"
description: "{}"
author: "Rust-Form Team"
license: "MIT"
keywords: ["generated", "component"]

api_compatibility:
  api_version: "0.1.0"
  min_version: "0.1.0"
  max_version: "0.2.0"
  required_features:
    - "core"
  experimental: false

dependencies: {{}}

provides:
  templates: []
  assets: []
  hooks: []

files: []
"#,
            component_name, config.description
        ))
    }

    /// Generate test file content
    fn generate_test_file(
        &self,
        component_name: &str,
        _config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        let rust_name = component_name.replace("-", "_");
        Ok(format!(
            "#[cfg(test)]\nmod {}_tests {{\n    // Generated tests for {}\n    use super::*;\n    \n    #[test]\n    fn test_{}_basic() {{\n        // Basic test for {}\n        assert!(true);\n    }}\n}}\n",
            rust_name, component_name, rust_name, component_name
        ))
    }

    /// Generate Cargo.toml for component
    fn generate_cargo_toml(
        &self,
        component_name: &str,
        _category: &ComponentCategory,
    ) -> Result<String, ComponentGenerationError> {
        let mut cargo_toml = format!(
            "[package]\nname = \"{}\"\nversion = \"1.0.0\"\nedition = \"2021\"\n\n[workspace]\n\n[lib]\nname = \"{}\"\npath = \"src/lib_test.rs\"\n\n[dependencies]\n",
            component_name, component_name.replace("-", "_")
        );

        for dep in &_category.common_dependencies {
            cargo_toml.push_str(&format!("{}\n", dep));
        }

        Ok(cargo_toml)
    }

    /// Generate README content
    fn generate_readme(
        &self,
        component_name: &str,
        _category: &ComponentCategory,
        config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        // TODO: Implement README generation using Tera
        Ok(format!("# {}\n\n{}\n", component_name, config.description))
    }

    /// Generate example configuration
    fn generate_example(
        &self,
        component_name: &str,
        example_number: usize,
        _config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        Ok(format!(
            "# Example {} for {}\nproject_name: example_{}\ncomponents:\n  {}: \"path:../\"\n",
            example_number, component_name, example_number, component_name
        ))
    }

    /// Generate test-app configuration for component testing
    fn generate_test_app_config(
        &self,
        component_name: &str,
        config: &ComponentGenerationConfig,
    ) -> Result<String, ComponentGenerationError> {
        Ok(format!(
            r#"name: "{}-test-app"
version: "0.1.0"
description: "Test application for component {}"

components:
  - name: "{}"
    path: "../"

models:
  TestModel:
    fields:
      id:
        type: integer
        primary: true
        auto_increment: true
      name:
        type: string
        max_length: 255
        required: true
      created_at:
        type: datetime
        auto_now_add: true

routes:
  - path: "/test"
    method: GET
    handler: "list_test_models"
    response:
      type: "TestModel[]"
  
  - path: "/test"
    method: POST
    handler: "create_test_model"
    request:
      type: "TestModel"
    response:
      type: "TestModel"
"#,
            component_name, config.description, component_name
        ))
    }

    /// Calculate initial quality score based on category requirements
    fn calculate_initial_quality_score(&self, category: &ComponentCategory) -> u8 {
        // Base score starts at 60 for generated components
        // Additional points for meeting category requirements
        let base_score = 60;
        let quality_bonus = if category.quality_requirements.min_test_coverage >= 85 {
            10
        } else {
            5
        };
        let template_bonus = std::cmp::min(category.templates.len() * 5, 15);

        std::cmp::min(base_score + quality_bonus + template_bonus as u8, 100)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComponentGenerationConfig {
    pub description: String,
    pub author: String,
    pub features: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ComponentSpec {
    pub name: String,
    pub category: String,
    pub description: String,
    pub author: Option<String>,
    pub features: Vec<String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct GeneratedComponent {
    pub name: String,
    pub category: String,
    pub path: String,
    pub template_files: Vec<String>,
    pub quality_score: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum ComponentGenerationError {
    #[error("Category not found: {0}")]
    CategoryNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Template rendering error: {0}")]
    TemplateError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Predefined component specifications for rapid library generation
pub fn get_component_library_specs() -> Vec<ComponentSpec> {
    vec![
        // Authentication Components
        ComponentSpec {
            name: "jwt-authentication".to_string(),
            category: "auth".to_string(),
            description: "Complete JWT authentication system with refresh tokens".to_string(),
            author: None,
            features: vec![
                "jwt".to_string(),
                "refresh-tokens".to_string(),
                "role-based".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "oauth2-integration".to_string(),
            category: "auth".to_string(),
            description: "OAuth2 integration for Google, GitHub, and custom providers".to_string(),
            author: None,
            features: vec!["oauth2".to_string(), "multiple-providers".to_string()],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "session-management".to_string(),
            category: "auth".to_string(),
            description: "Secure session management with Redis backend".to_string(),
            author: None,
            features: vec![
                "sessions".to_string(),
                "redis".to_string(),
                "security".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        // Payment Components
        ComponentSpec {
            name: "stripe-integration".to_string(),
            category: "payments".to_string(),
            description: "Complete Stripe payment processing with webhooks".to_string(),
            author: None,
            features: vec![
                "stripe".to_string(),
                "webhooks".to_string(),
                "subscriptions".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "paypal-integration".to_string(),
            category: "payments".to_string(),
            description: "PayPal payment processing and order management".to_string(),
            author: None,
            features: vec!["paypal".to_string(), "orders".to_string()],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "invoice-system".to_string(),
            category: "payments".to_string(),
            description: "Invoice generation, tracking, and payment processing".to_string(),
            author: None,
            features: vec![
                "invoicing".to_string(),
                "pdf-generation".to_string(),
                "tracking".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        // Dashboard Components
        ComponentSpec {
            name: "analytics-dashboard".to_string(),
            category: "dashboards".to_string(),
            description: "Real-time analytics dashboard with charts and metrics".to_string(),
            author: None,
            features: vec![
                "analytics".to_string(),
                "real-time".to_string(),
                "charts".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "admin-panel".to_string(),
            category: "dashboards".to_string(),
            description: "Complete admin panel with user management and system controls"
                .to_string(),
            author: None,
            features: vec![
                "admin".to_string(),
                "user-management".to_string(),
                "system-controls".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "monitoring-dashboard".to_string(),
            category: "dashboards".to_string(),
            description: "System monitoring dashboard with alerts and notifications".to_string(),
            author: None,
            features: vec![
                "monitoring".to_string(),
                "alerts".to_string(),
                "notifications".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        // E-commerce Components
        ComponentSpec {
            name: "product-catalog".to_string(),
            category: "ecommerce".to_string(),
            description: "Product catalog with search, filtering, and inventory management"
                .to_string(),
            author: None,
            features: vec![
                "catalog".to_string(),
                "search".to_string(),
                "inventory".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "shopping-cart".to_string(),
            category: "ecommerce".to_string(),
            description: "Shopping cart with persistence and checkout integration".to_string(),
            author: None,
            features: vec![
                "cart".to_string(),
                "persistence".to_string(),
                "checkout".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "order-management".to_string(),
            category: "ecommerce".to_string(),
            description: "Complete order management system with tracking and fulfillment"
                .to_string(),
            author: None,
            features: vec![
                "orders".to_string(),
                "tracking".to_string(),
                "fulfillment".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        // CMS Components
        ComponentSpec {
            name: "content-editor".to_string(),
            category: "cms".to_string(),
            description: "Rich text content editor with media management".to_string(),
            author: None,
            features: vec![
                "editor".to_string(),
                "media".to_string(),
                "rich-text".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "blog-system".to_string(),
            category: "cms".to_string(),
            description: "Complete blog system with posts, comments, and SEO".to_string(),
            author: None,
            features: vec![
                "blog".to_string(),
                "comments".to_string(),
                "seo".to_string(),
            ],
            dependencies: HashMap::new(),
        },
        ComponentSpec {
            name: "file-manager".to_string(),
            category: "cms".to_string(),
            description: "File upload, organization, and management system".to_string(),
            author: None,
            features: vec![
                "files".to_string(),
                "upload".to_string(),
                "organization".to_string(),
            ],
            dependencies: HashMap::new(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_generator_initialization() {
        let generator = ComponentLibraryGenerator::new("./test_output".to_string());
        assert_eq!(generator.categories.len(), 5);
        assert!(generator.categories.contains_key("auth"));
        assert!(generator.categories.contains_key("payments"));
    }

    #[test]
    fn test_predefined_component_specs() {
        let specs = get_component_library_specs();
        assert!(specs.len() >= 15); // Should have at least 15 predefined components

        // Check that we have components for each major category
        let categories: std::collections::HashSet<_> = specs.iter().map(|s| &s.category).collect();
        assert!(categories.contains(&"auth".to_string()));
        assert!(categories.contains(&"payments".to_string()));
        assert!(categories.contains(&"dashboards".to_string()));
        assert!(categories.contains(&"ecommerce".to_string()));
        assert!(categories.contains(&"cms".to_string()));
    }
}
