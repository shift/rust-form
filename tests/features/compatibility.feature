Feature: Component Compatibility
  As a developer
  I want to ensure components are compatible with my rust-form version
  So that I can safely integrate third-party functionality

  Scenario: Compatible component installation
    Given a component with compatible API version
    When I install the component
    Then the installation should succeed
    And the component should be available for use

  Scenario: Incompatible component detection
    Given a component with incompatible API version
    When I attempt to install the component
    Then the installation should fail with a clear error message
    And the error should suggest compatible alternatives

  Scenario: Experimental component warning
    Given a component marked as experimental
    When I install the component
    Then the installation should succeed with warnings
    And the warnings should indicate experimental status

  Scenario: Component testing with compatibility check
    Given a component installed locally
    When I run component tests
    Then the compatibility check should pass
    And all test phases should complete successfully

  Scenario: Component testing with incompatible version
    Given a component with incompatible API version
    When I run component tests
    Then the compatibility check should fail
    And the test should exit with appropriate error message

  Scenario: Component testing with unit tests
    Given a component with unit tests
    When I run component tests
    Then unit tests should be discovered and executed
    And test results should be reported accurately

  Scenario: Component testing without unit tests
    Given a component without unit tests
    When I run component tests
    Then the system should report no tests found
    And continue with other test phases

  Scenario: Component test application generation
    Given a valid component
    When I run component tests with test app generation
    Then a test application should be generated
    And the test application should compile successfully

  Scenario: Component quality assessment
    Given a component with documentation and examples
    When I run component tests
    Then quality metrics should be calculated
    And a quality score should be provided

  Scenario: Component testing with custom directory
    Given components in a custom directory
    When I run component tests specifying the directory
    Then the component should be found and tested
    And all test phases should complete normally

  Scenario: Component testing unit tests only
    Given a component with unit tests
    When I run component tests in unit-tests-only mode
    Then only unit tests should be executed
    And other test phases should be skipped

  Scenario: Component testing skip compatibility
    Given a component that may have compatibility issues
    When I run component tests with skip-compatibility flag
    Then compatibility check should be skipped
    And other test phases should continue normally