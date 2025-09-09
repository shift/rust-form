# Jwt Authentication Component

JWT-based authentication with token management

## Category
Auth

## Features
- Feature 1: Core jwt authentication functionality
- Feature 2: Advanced jwt authentication configuration
- Feature 3: Integration with rust-form ecosystem
- Feature 4: Comprehensive error handling
- Feature 5: Security best practices

## Installation

Add this component to your rust-form project:

```yaml
components:
  jwt-authentication: "path:./components/auth/jwt-authentication"
```

## Configuration

### Basic Usage

```yaml
project_name: my_app
version: "0.1.0"

components:
  jwt-authentication: "path:./components/auth/jwt-authentication"

api:
  models:
    MyModel:
      table_name: my_models
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        name:
          type: string
          required: true

  endpoints:
    - path: /my-models
      model: MyModel
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true
```

### Advanced Configuration

```yaml
# Add advanced configuration examples here
jwt-authentication_settings:
  feature_enabled: true
  custom_options:
    option1: "value1"
    option2: "value2"
```

## API Reference

### Handlers
- `create_jwt_authentication`: Create new jwt authentication
- `get_jwt_authentication_by_id`: Retrieve jwt authentication by ID
- `update_jwt_authentication`: Update existing jwt authentication
- `delete_jwt_authentication`: Delete jwt authentication

### Models
- `JwtAuthentication`: Main data model for jwt authentication

### Middleware
- `jwt_authentication_middleware`: Core jwt authentication middleware
- `validate_jwt_authentication_request`: Request validation
- `log_jwt_authentication_activity`: Activity logging

## Examples

See the `examples/` directory for complete usage examples:
- `examples/basic-usage.yml`: Simple setup
- `examples/advanced-config.yml`: Advanced configuration
- `examples/integration-test.yml`: Integration testing

## Testing

Run component tests:

```bash
rustform component test components/auth/jwt-authentication
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Changelog

### v1.0.0
- Initial release
- Core jwt authentication functionality
- Basic integration with rust-form
- Comprehensive test suite
