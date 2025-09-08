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