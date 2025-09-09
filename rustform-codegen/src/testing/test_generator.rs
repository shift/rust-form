use rustform_core::component::ComponentManifest as ComponentConfig;
use std::collections::HashMap;
use tera::{Context, Tera};

/// Test generation system for components
pub struct TestGenerator {
    tera: Tera,
}

impl TestGenerator {
    /// Create a new test generator
    pub fn new() -> Result<Self, tera::Error> {
        let mut tera = Tera::new("rustform-codegen/templates/testing/**/*")?;

        // Register custom filters for test generation
        tera.register_filter("test_name", test_name_filter);
        tera.register_filter("mock_name", mock_name_filter);
        tera.register_filter("rust_type", rust_type_filter);

        Ok(Self { tera })
    }

    /// Generate all tests for a component
    pub fn generate_tests(
        &self,
        config: &ComponentConfig,
    ) -> Result<HashMap<String, String>, TestGenerationError> {
        let mut generated_tests = HashMap::new();

        // Generate unit tests
        if let Some(tests) = &config.tests {
            if let Some(unit_tests) = &tests.unit {
                for test_name in unit_tests {
                    let test_content = self.generate_unit_test(config, test_name)?;
                    generated_tests.insert(
                        format!("tests/unit_{}.rs", test_name.to_lowercase()),
                        test_content,
                    );
                }
            }

            // Generate integration tests
            if let Some(integration_tests) = &tests.integration {
                for test_name in integration_tests {
                    let test_content = self.generate_integration_test(config, test_name)?;
                    generated_tests.insert(
                        format!("tests/{}.rs", test_name.to_lowercase()),
                        test_content,
                    );
                }
            }

            // Generate performance tests
            if let Some(performance_tests) = &tests.performance {
                for test_name in performance_tests {
                    let test_content = self.generate_performance_test(config, test_name)?;
                    generated_tests.insert(
                        format!("benches/{}.rs", test_name.to_lowercase()),
                        test_content,
                    );
                }
            }
        }

        // Generate test utilities
        let test_utils = self.generate_test_utilities(config)?;
        generated_tests.insert("tests/common/mod.rs".to_string(), test_utils);

        // Generate mock objects
        let mocks = self.generate_mocks(config)?;
        generated_tests.insert("tests/mocks.rs".to_string(), mocks);

        Ok(generated_tests)
    }

    /// Generate a single unit test
    fn generate_unit_test(
        &self,
        config: &ComponentConfig,
        test_name: &str,
    ) -> Result<String, TestGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("test_name", test_name);
        context.insert("test_type", "unit");

        // Determine test template based on component category and test name
        let template_name = self.get_unit_test_template(config, test_name);

        self.tera
            .render(&template_name, &context)
            .map_err(TestGenerationError::TemplateError)
    }

    /// Generate an integration test
    fn generate_integration_test(
        &self,
        config: &ComponentConfig,
        test_name: &str,
    ) -> Result<String, TestGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("test_name", test_name);
        context.insert("test_type", "integration");

        let template_name = self.get_integration_test_template(config, test_name);

        self.tera
            .render(&template_name, &context)
            .map_err(TestGenerationError::TemplateError)
    }

    /// Generate a performance test
    fn generate_performance_test(
        &self,
        config: &ComponentConfig,
        test_name: &str,
    ) -> Result<String, TestGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);
        context.insert("test_name", test_name);
        context.insert("test_type", "performance");

        let template_name = "performance_test.rs.tera";

        self.tera
            .render(template_name, &context)
            .map_err(TestGenerationError::TemplateError)
    }

    /// Generate test utilities
    fn generate_test_utilities(
        &self,
        config: &ComponentConfig,
    ) -> Result<String, TestGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);

        self.tera
            .render("test_utilities.rs.tera", &context)
            .map_err(TestGenerationError::TemplateError)
    }

    /// Generate mock objects
    fn generate_mocks(&self, config: &ComponentConfig) -> Result<String, TestGenerationError> {
        let mut context = Context::new();
        context.insert("component", config);

        // Analyze dependencies to determine what mocks are needed
        let mock_dependencies = self.analyze_mock_dependencies(config);
        context.insert("mock_dependencies", &mock_dependencies);

        self.tera
            .render("mocks.rs.tera", &context)
            .map_err(TestGenerationError::TemplateError)
    }

    /// Get appropriate unit test template based on component and test
    fn get_unit_test_template(&self, config: &ComponentConfig, test_name: &str) -> String {
        match config.category_str() {
            "auth" => match test_name {
                name if name.contains("token") => "auth/token_test.rs.tera".to_string(),
                name if name.contains("login") => "auth/login_test.rs.tera".to_string(),
                name if name.contains("permission") => "auth/permission_test.rs.tera".to_string(),
                _ => "auth/generic_auth_test.rs.tera".to_string(),
            },
            "database" => match test_name {
                name if name.contains("connection") => {
                    "database/connection_test.rs.tera".to_string()
                }
                name if name.contains("query") => "database/query_test.rs.tera".to_string(),
                name if name.contains("migration") => "database/migration_test.rs.tera".to_string(),
                _ => "database/generic_db_test.rs.tera".to_string(),
            },
            "api" => match test_name {
                name if name.contains("endpoint") => "api/endpoint_test.rs.tera".to_string(),
                name if name.contains("middleware") => "api/middleware_test.rs.tera".to_string(),
                name if name.contains("validation") => "api/validation_test.rs.tera".to_string(),
                _ => "api/generic_api_test.rs.tera".to_string(),
            },
            _ => "generic_unit_test.rs.tera".to_string(),
        }
    }

    /// Get appropriate integration test template
    fn get_integration_test_template(&self, config: &ComponentConfig, test_name: &str) -> String {
        match config.category_str() {
            "auth" => "auth/auth_integration_test.rs.tera".to_string(),
            "database" => "database/db_integration_test.rs.tera".to_string(),
            "api" => "api/api_integration_test.rs.tera".to_string(),
            _ => "generic_integration_test.rs.tera".to_string(),
        }
    }

    /// Analyze component dependencies to determine required mocks
    fn analyze_mock_dependencies(&self, config: &ComponentConfig) -> Vec<MockDependency> {
        let mut mocks = Vec::new();

        // Analyze Rust dependencies for common mockable services
        for dep in &config.dependencies.rust {
            if dep.contains("sqlx") || dep.contains("diesel") {
                mocks.push(MockDependency {
                    name: "MockDatabase".to_string(),
                    trait_name: "Database".to_string(),
                    methods: vec![
                        "connect".to_string(),
                        "execute".to_string(),
                        "query".to_string(),
                    ],
                });
            }

            if dep.contains("reqwest") || dep.contains("hyper") {
                mocks.push(MockDependency {
                    name: "MockHttpClient".to_string(),
                    trait_name: "HttpClient".to_string(),
                    methods: vec![
                        "get".to_string(),
                        "post".to_string(),
                        "put".to_string(),
                        "delete".to_string(),
                    ],
                });
            }

            if dep.contains("redis") {
                mocks.push(MockDependency {
                    name: "MockRedis".to_string(),
                    trait_name: "RedisClient".to_string(),
                    methods: vec![
                        "get".to_string(),
                        "set".to_string(),
                        "del".to_string(),
                        "exists".to_string(),
                    ],
                });
            }
        }

        // Add component-specific mocks based on templates
        if let Some(ref template_config) = config.templates {
            if let Some(ref generates) = template_config.generates {
                for template in generates {
                    if template.contains("email") {
                        mocks.push(MockDependency {
                            name: "MockEmailService".to_string(),
                            trait_name: "EmailService".to_string(),
                            methods: vec!["send_email".to_string(), "send_template".to_string()],
                        });
                    }

                    if template.contains("storage") || template.contains("file") {
                        mocks.push(MockDependency {
                            name: "MockStorageService".to_string(),
                            trait_name: "StorageService".to_string(),
                            methods: vec![
                                "upload".to_string(),
                                "download".to_string(),
                                "delete".to_string(),
                                "exists".to_string(),
                            ],
                        });
                    }
                }
            }
        }

        mocks
    }
}

/// Mock dependency configuration
#[derive(Debug, Clone, serde::Serialize)]
pub struct MockDependency {
    pub name: String,
    pub trait_name: String,
    pub methods: Vec<String>,
}

/// Test generation errors
#[derive(Debug, thiserror::Error)]
pub enum TestGenerationError {
    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid test configuration: {0}")]
    InvalidTestConfig(String),
}

// Template filters

/// Convert test name to valid Rust function name
fn test_name_filter(
    value: &tera::Value,
    _: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let test_name = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("test_name filter can only be applied to strings"))?;

    let rust_name = test_name
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    Ok(tera::Value::String(format!("test_{}", rust_name)))
}

/// Convert name to mock name
fn mock_name_filter(
    value: &tera::Value,
    _: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let name = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("mock_name filter can only be applied to strings"))?;

    let mock_name = format!(
        "Mock{}",
        name.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>()
    );

    Ok(tera::Value::String(mock_name))
}

/// Convert type to Rust type
fn rust_type_filter(
    value: &tera::Value,
    _: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let type_str = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("rust_type filter can only be applied to strings"))?;

    let rust_type = match type_str {
        "string" => "String",
        "integer" => "i32",
        "boolean" => "bool",
        "duration" => "std::time::Duration",
        "array" => "Vec<String>",
        "object" => "serde_json::Value",
        _ => type_str,
    };

    Ok(tera::Value::String(rust_type.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustform_core::component::{ComponentManifest, ApiCompatibility, ComponentDependencies, TestConfiguration, DocumentationConfig, TemplateConfig};

    fn create_test_component() -> ComponentManifest {
        ComponentManifest {
            name: "test-component".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test component".to_string()),
            author: Some("Test Author".to_string()),
            license: Some("MIT".to_string()),
            homepage: None,
            repository: None,
            keywords: vec!["auth".to_string()],
            category: Some("auth".to_string()),
            subcategory: None,
            priority: Some("high".to_string()),
            complexity: Some("medium".to_string()),
            api_compatibility: ApiCompatibility {
                api_version: "0.1.0".to_string(),
                min_version: "0.1.0".to_string(),
                max_version: None,
                required_features: None,
                experimental: None,
            },
            dependencies: ComponentDependencies {
                rust: vec![
                    "sqlx = \"0.7\"".to_string(),
                    "reqwest = \"0.11\"".to_string(),
                ],
                nix: None,
            },
            provides: None,
            config_schema: None,
            compliance: None,
            tests: Some(TestConfiguration {
                unit: Some(vec![
                    "test_token_creation".to_string(),
                    "test_token_validation".to_string(),
                ]),
                integration: Some(vec!["test_auth_flow".to_string()]),
                compliance: None,
                performance: Some(vec!["benchmark_token_operations".to_string()]),
                security: None,
            }),
            documentation: Some(DocumentationConfig {
                compliance_guide: None,
                dpo_manual: None,
                api_reference: Some(true),
                implementation_checklist: None,
                privacy_notice_template: None,
                cookie_policy_template: None,
                data_processing_records: None,
                breach_response_procedures: None,
            }),
            features: None,
            templates: Some(TemplateConfig {
                generates: Some(vec!["auth/token_handler.rs".to_string()]),
                requires: None,
            }),
            integrity: None,
            files: vec!["lib.rs".to_string()],
        }
    }

    #[test]
    fn test_mock_dependency_analysis() {
        let generator = TestGenerator::new().expect("Failed to create test generator");
        let component = create_test_component();

        let mocks = generator.analyze_mock_dependencies(&component);

        // Should detect sqlx and reqwest dependencies
        assert!(mocks.iter().any(|m| m.name == "MockDatabase"));
        assert!(mocks.iter().any(|m| m.name == "MockHttpClient"));
    }

    #[test]
    fn test_test_name_filter() {
        let value = tera::Value::String("Test Token Creation".to_string());
        let result = test_name_filter(&value, &HashMap::new()).unwrap();
        assert_eq!(result.as_str().unwrap(), "test_test_token_creation");
    }

    #[test]
    fn test_mock_name_filter() {
        let value = tera::Value::String("database_client".to_string());
        let result = mock_name_filter(&value, &HashMap::new()).unwrap();
        assert_eq!(result.as_str().unwrap(), "MockDatabaseClient");
    }
}
