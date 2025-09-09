use crate::config::Config;
use crate::error::{ConfigError, ConfigResult, ValidationError};
use miette::{NamedSource, SourceSpan};
use serde_yaml::Value;
use std::path::Path;

pub struct ConfigParser {
    source: String,
    filename: String,
}

impl ConfigParser {
    pub fn new(source: String, filename: impl Into<String>) -> Self {
        Self {
            source,
            filename: filename.into(),
        }
    }

    pub fn parse(&self) -> ConfigResult<Config> {
        // First, parse as generic YAML to get better error locations
        let yaml_value: Value =
            serde_yaml::from_str(&self.source).map_err(|e| self.enhance_yaml_error(e))?;

        // Then parse into our config struct with validation
        let config: Config =
            serde_yaml::from_value(yaml_value).map_err(|e| self.enhance_yaml_error(e))?;

        // Validate the configuration
        crate::config::validate_config(&config)
            .map_err(|validation_error| self.enhance_validation_error(validation_error))?;

        Ok(config)
    }

    fn enhance_yaml_error(&self, error: serde_yaml::Error) -> ConfigError {
        if let Some(location) = error.location() {
            let line = location.line();
            let column = location.column();

            // Calculate approximate byte offset for span
            let lines: Vec<&str> = self.source.lines().collect();
            let mut byte_offset = 0;
            for (i, line_content) in lines.iter().enumerate() {
                if i == line {
                    byte_offset += column;
                    break;
                }
                byte_offset += line_content.len() + 1; // +1 for newline
            }

            let _span = SourceSpan::from((byte_offset, 1));
            let _named_source = NamedSource::new(&self.filename, self.source.clone());

            ConfigError::Yaml(error)
        } else {
            ConfigError::Yaml(error)
        }
    }

    fn enhance_validation_error(&self, error: ValidationError) -> ConfigError {
        // Try to find the location of the field mentioned in the error
        let field_location = self.find_field_location(&error);

        if let Some((line, column)) = field_location {
            // Calculate byte offset for better span reporting
            let _span = self.calculate_span(line, column);
            // We could add source spans to validation errors if we extended the error types
        }

        ConfigError::Validation(error)
    }

    fn find_field_location(&self, error: &ValidationError) -> Option<(usize, usize)> {
        // Parse the source to find field locations
        // This is a simplified version - a full implementation would use a YAML parser
        // that preserves location information throughout the parsing process

        match error {
            ValidationError::MissingField { field } => self.find_text_location(field),
            ValidationError::ModelNotFound { model } => self.find_text_location(model),
            ValidationError::InvalidPath { path, .. } => self.find_text_location(path),
            ValidationError::NoPrimaryKey { model } => self.find_text_location(model),
            _ => None,
        }
    }

    fn find_text_location(&self, text: &str) -> Option<(usize, usize)> {
        for (line_num, line) in self.source.lines().enumerate() {
            if let Some(column) = line.find(text) {
                return Some((line_num, column));
            }
        }
        None
    }

    fn calculate_span(&self, line: usize, column: usize) -> SourceSpan {
        let lines: Vec<&str> = self.source.lines().collect();
        let mut byte_offset = 0;

        for (i, line_content) in lines.iter().enumerate() {
            if i == line {
                byte_offset += column;
                break;
            }
            byte_offset += line_content.len() + 1; // +1 for newline
        }

        SourceSpan::from((byte_offset, 1))
    }
}

pub fn parse_config_file(path: &Path) -> ConfigResult<Config> {
    let source = std::fs::read_to_string(path)?;
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("config.yml")
        .to_string();

    let parser = ConfigParser::new(source, filename);
    parser.parse()
}

pub fn parse_config_str(source: &str, filename: &str) -> ConfigResult<Config> {
    let parser = ConfigParser::new(source.to_string(), filename);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_config() {
        let yaml = r#"
project_name: test_api
version: "0.1.0"
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
  endpoints:
    - path: /todos
      model: Todo
      crud:
        read_all: true
"#;

        let result = parse_config_str(yaml, "test.yml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = r#"
project_name: test_api
version: "0.1.0"
database:
  type: sqlite
  url_env: DATABASE_URL
  invalid_syntax: [unclosed array
"#;

        let result = parse_config_str(yaml, "test.yml");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_validation_error() {
        let yaml = r#"
project_name: test_api
version: "0.1.0"
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
          # Missing primary key
  endpoints:
    - path: /todos
      model: NonExistentModel
      crud:
        read_all: true
"#;

        let result = parse_config_str(yaml, "test.yml");
        assert!(result.is_err());
    }
}
