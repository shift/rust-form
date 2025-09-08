Feature: Rust-form Code Generation
  As a developer
  I want to generate working Rust backends from YAML configurations
  So that I can rapidly develop type-safe web APIs

  Scenario: Basic project generation
    Given a valid rust-form configuration
    When I generate the rust-form project
    Then the project should be generated successfully
    And all required files should be present
    And the generated project should compile

  Scenario: GDPR compliance generation
    Given a configuration with GDPR compliance requirements
    When I generate the rust-form project
    Then the project should be generated successfully
    And GDPR compliance handlers should be generated
    And the generated code should include data subject rights endpoints
    And the generated project should compile