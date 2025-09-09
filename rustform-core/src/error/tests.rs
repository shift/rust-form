use crate::error::{ConfigError, ConfigResult, Error, Result, ValidationError, ValidationResult};
use rstest::*;
use similar_asserts::assert_eq;
use std::io;

#[rstest]
fn test_validation_error_creation() {
    let error = ValidationError::MissingField {
        field: "name".to_string(),
    };

    assert!(error
        .to_string()
        .contains("Required field 'name' is missing"));
}

#[rstest]
fn test_validation_error_invalid_type() {
    let error = ValidationError::InvalidType {
        field: "age".to_string(),
        expected: "integer".to_string(),
        actual: "string".to_string(),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid field type for 'age'"));
    assert!(error_msg.contains("expected integer"));
    assert!(error_msg.contains("got string"));
}

#[rstest]
fn test_validation_error_model_not_found() {
    let error = ValidationError::ModelNotFound {
        model: "NonExistentModel".to_string(),
    };

    assert!(error
        .to_string()
        .contains("Model 'NonExistentModel' referenced in endpoint but not defined"));
}

#[rstest]
fn test_validation_error_duplicate_table() {
    let error = ValidationError::DuplicateTableName {
        table_name: "users".to_string(),
        model1: "User".to_string(),
        model2: "Person".to_string(),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Duplicate table name 'users'"));
    assert!(error_msg.contains("'User'"));
    assert!(error_msg.contains("'Person'"));
}

#[rstest]
fn test_validation_error_invalid_path() {
    let error = ValidationError::InvalidPath {
        path: "invalid path".to_string(),
        reason: "contains spaces".to_string(),
    };

    assert!(error
        .to_string()
        .contains("Invalid endpoint path 'invalid path'"));
    assert!(error.to_string().contains("contains spaces"));
}

#[rstest]
fn test_validation_error_no_primary_key() {
    let error = ValidationError::NoPrimaryKey {
        model: "User".to_string(),
    };

    assert!(error
        .to_string()
        .contains("Model 'User' has no primary key defined"));
}

#[rstest]
fn test_validation_error_multiple_primary_keys() {
    let error = ValidationError::MultiplePrimaryKeys {
        model: "User".to_string(),
        fields: vec!["id".to_string(), "uuid".to_string()],
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Model 'User' has multiple primary keys"));
    assert!(error_msg.contains("id"));
    assert!(error_msg.contains("uuid"));
}

#[rstest]
fn test_validation_error_invalid_constraint() {
    let error = ValidationError::InvalidFieldConstraint {
        field: "email".to_string(),
        model: "User".to_string(),
        reason: "min_length cannot be negative".to_string(),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid field constraint for 'email' in model 'User'"));
    assert!(error_msg.contains("min_length cannot be negative"));
}

#[rstest]
fn test_validation_error_invalid_relationship() {
    let error = ValidationError::InvalidRelationship {
        model: "Post".to_string(),
        field: "author".to_string(),
        reason: "referenced model 'User' does not exist".to_string(),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid relationship in model 'Post', field 'author'"));
    assert!(error_msg.contains("referenced model 'User' does not exist"));
}

#[rstest]
fn test_validation_error_circular_dependency() {
    let error = ValidationError::CircularDependency {
        cycle: vec!["User".to_string(), "Post".to_string(), "User".to_string()],
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Circular dependency detected"));
    assert!(error_msg.contains("User"));
    assert!(error_msg.contains("Post"));
}

#[rstest]
fn test_validation_error_incompatible_api_version() {
    let error = ValidationError::IncompatibleApiVersion {
        requested: "2.0.0".to_string(),
        current: "1.0.0".to_string(),
        reason: "major version mismatch".to_string(),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Incompatible API version '2.0.0'"));
    assert!(error_msg.contains("current rust-form version is '1.0.0'"));
    assert!(error_msg.contains("major version mismatch"));
}

#[rstest]
fn test_config_error_from_yaml() {
    // Test YAML errors - Create a simple parsing error
    let yaml_result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: [}");
    let yaml_error = yaml_result.unwrap_err();
    let config_error = ConfigError::Yaml(yaml_error);

    assert!(config_error.to_string().contains("YAML parsing error"));
}

#[rstest]
fn test_config_error_from_validation() {
    let validation_error = ValidationError::MissingField {
        field: "project_name".to_string(),
    };
    let config_error = ConfigError::Validation(validation_error);

    assert!(config_error
        .to_string()
        .contains("Configuration validation failed"));
}

#[rstest]
fn test_config_error_from_io() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let config_error = ConfigError::Io(io_error);

    assert!(config_error.to_string().contains("IO error"));
}

#[rstest]
fn test_error_from_config() {
    let validation_error = ValidationError::MissingField {
        field: "database".to_string(),
    };
    let config_error = ConfigError::Validation(validation_error);
    let error = Error::Config(config_error);

    assert!(error.to_string().contains("Configuration error"));
}

#[rstest]
fn test_error_validation() {
    let error = Error::ValidationError("Custom validation message".to_string());

    assert!(error
        .to_string()
        .contains("Validation error: Custom validation message"));
}

#[rstest]
fn test_error_component() {
    let error = Error::ComponentError("Component not found".to_string());

    assert!(error
        .to_string()
        .contains("Component error: Component not found"));
}

#[rstest]
fn test_error_from_io() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let error = Error::Io(io_error);

    assert!(error.to_string().contains("IO error"));
}

#[rstest]
fn test_error_from_yaml() {
    let yaml_result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: syntax");
    let yaml_error = yaml_result.unwrap_err();
    let error = Error::Yaml(yaml_error);

    assert!(error.to_string().contains("YAML parsing error"));
}

#[rstest]
fn test_error_from_json() {
    let json_result = serde_json::from_str::<serde_json::Value>("invalid json {[}");
    let json_error = json_result.unwrap_err();
    let error = Error::Json(json_error);

    assert!(error.to_string().contains("JSON parsing error"));
}

#[rstest]
fn test_result_types() {
    // Test success cases
    let ok_result: Result<String> = Ok("success".to_string());
    assert!(ok_result.is_ok());
    assert_eq!(ok_result.unwrap(), "success");

    let validation_ok: ValidationResult<i32> = Ok(42);
    assert!(validation_ok.is_ok());
    assert_eq!(validation_ok.unwrap(), 42);

    let config_ok: ConfigResult<bool> = Ok(true);
    assert!(config_ok.is_ok());
    assert_eq!(config_ok.unwrap(), true);

    // Test error cases
    let err_result: Result<String> = Err(Error::ValidationError("test error".to_string()));
    assert!(err_result.is_err());

    let validation_err: ValidationResult<i32> = Err(ValidationError::MissingField {
        field: "test".to_string(),
    });
    assert!(validation_err.is_err());

    let config_err: ConfigResult<bool> = Err(ConfigError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        "not found",
    )));
    assert!(config_err.is_err());
}

mod error_property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_missing_field_error_property(
            field in "[a-zA-Z][a-zA-Z0-9_]{0,49}"
        ) {
            let error = ValidationError::MissingField {
                field: field.clone(),
            };

            let error_msg = error.to_string();
            prop_assert!(error_msg.contains(&field));
            prop_assert!(error_msg.contains("Required field"));
            prop_assert!(error_msg.contains("is missing"));
        }

        #[test]
        fn test_invalid_type_error_property(
            field in "[a-zA-Z][a-zA-Z0-9_]{0,49}",
            expected in "[a-zA-Z][a-zA-Z0-9_]{0,20}",
            actual in "[a-zA-Z][a-zA-Z0-9_]{0,20}"
        ) {
            let error = ValidationError::InvalidType {
                field: field.clone(),
                expected: expected.clone(),
                actual: actual.clone(),
            };

            let error_msg = error.to_string();
            prop_assert!(error_msg.contains(&field));
            prop_assert!(error_msg.contains(&expected));
            prop_assert!(error_msg.contains(&actual));
            prop_assert!(error_msg.contains("Invalid field type"));
        }

        #[test]
        fn test_model_not_found_error_property(
            model in "[A-Z][a-zA-Z0-9]{0,49}"
        ) {
            let error = ValidationError::ModelNotFound {
                model: model.clone(),
            };

            let error_msg = error.to_string();
            prop_assert!(error_msg.contains(&model));
            prop_assert!(error_msg.contains("referenced in endpoint but not defined"));
        }
    }
}

#[cfg(test)]
mod error_integration_tests {
    use super::*;

    #[test]
    fn test_error_chain() {
        // Test error conversion chain: ValidationError -> ConfigError -> Error
        let validation_error = ValidationError::MissingField {
            field: "api".to_string(),
        };

        let config_error = ConfigError::Validation(validation_error);
        let main_error = Error::Config(config_error);

        // Should be able to access the original error message through the chain
        assert!(main_error.to_string().contains("Configuration error"));

        // Test source chain
        use std::error::Error as StdError;
        let source = main_error.source();
        assert!(source.is_some());
        assert!(source
            .unwrap()
            .to_string()
            .contains("Configuration validation failed"));
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = ValidationError::InvalidProjectName {
            name: "invalid name!".to_string(),
            reason: "contains invalid characters".to_string(),
        };

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidProjectName"));
        assert!(debug_str.contains("invalid name!"));
        assert!(debug_str.contains("contains invalid characters"));
    }
}
