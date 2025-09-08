Feature: Component Testing
  As a component developer
  I want to validate my component meets quality standards
  So that users can confidently integrate my component

  Scenario: Valid component passes all tests
    Given a well-formed component with tests and documentation
    When I run the component test command
    Then all test phases should pass
    And the quality score should be above 70
    And the component should be marked as production ready

  Scenario: Component with missing documentation
    Given a component without README or documentation
    When I run the component test command
    Then the quality assessment should identify missing documentation
    And suggestions should be provided for improvement
    And the quality score should reflect the deficiency

  Scenario: Component with failing unit tests
    Given a component with failing unit tests
    When I run the component test command
    Then the unit test phase should fail
    And failing test details should be reported
    And the overall test should fail

  Scenario: Component generates invalid test application
    Given a component that causes compilation errors
    When I run the component test with test app generation
    Then the test application generation should fail
    And compilation errors should be reported
    And suggestions for fixing should be provided

  Scenario: Component test with custom test frameworks
    Given a component using custom test frameworks
    When I run the component test command
    Then the test discovery should adapt to the framework
    And test results should be parsed correctly
    And framework-specific metrics should be reported

  Scenario: Component test performance validation
    Given a component with performance requirements
    When I run component tests with performance validation
    Then template rendering speed should be measured
    And performance metrics should be within acceptable limits
    And performance regression should be detected

  Scenario: Component security validation
    Given a component with template files
    When I run component tests with security validation
    Then input validation should be checked
    And XSS prevention should be verified
    And security vulnerabilities should be reported

  Scenario: Component test with external dependencies
    Given a component with external dependencies
    When I run the component test command
    Then dependency resolution should be validated
    And dependency compatibility should be checked
    And missing dependencies should be reported

  Scenario: Component test output formats
    Given a component being tested
    When I run component tests with different output formats
    Then results should be available in JSON format
    And results should be available in human-readable format
    And CI-friendly output should be supported

  Scenario: Component test coverage reporting
    Given a component with unit tests
    When I run component tests with coverage reporting
    Then test coverage percentage should be calculated
    And coverage reports should be generated
    And coverage thresholds should be enforced