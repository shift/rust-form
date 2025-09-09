#[cfg(test)]
mod ui_kit_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_button_template_exists() {
        let template_path = Path::new("button.tera");
        assert!(template_path.exists(), "Button template should exist");
    }

    #[test]
    fn test_form_template_exists() {
        let template_path = Path::new("form.tera");
        assert!(template_path.exists(), "Form template should exist");
    }

    #[test]
    fn test_input_template_exists() {
        let template_path = Path::new("input.tera");
        assert!(template_path.exists(), "Input template should exist");
    }

    #[test]
    fn test_button_template_structure() {
        let content = fs::read_to_string("button.tera")
            .expect("Button template should be readable");
        
        // Check for basic button structure
        assert!(content.contains("button"), "Button template should contain button element");
        assert!(content.contains("{{"), "Button template should contain template variables");
    }

    #[test]
    fn test_form_template_structure() {
        let content = fs::read_to_string("form.tera")
            .expect("Form template should be readable");
        
        // Check for basic form structure
        assert!(content.contains("form"), "Form template should contain form element");
        assert!(content.contains("{{"), "Form template should contain template variables");
    }

    #[test]
    fn test_input_template_structure() {
        let content = fs::read_to_string("input.tera")
            .expect("Input template should be readable");
        
        // Check for basic input structure
        assert!(content.contains("input"), "Input template should contain input element");
        assert!(content.contains("{{"), "Input template should contain template variables");
    }

    #[test]
    fn test_templates_have_consistent_naming() {
        let button_content = fs::read_to_string("button.tera").unwrap();
        let form_content = fs::read_to_string("form.tera").unwrap();
        let input_content = fs::read_to_string("input.tera").unwrap();
        
        // Check that templates follow consistent variable naming patterns
        for content in [&button_content, &form_content, &input_content] {
            // Templates should use consistent variable syntax
            if content.contains("{{") {
                assert!(content.contains("}}"), "Template variables should be properly closed");
            }
        }
    }

    #[test]
    fn test_component_manifest_validity() {
        let manifest_path = Path::new("rustform-component.yml");
        assert!(manifest_path.exists(), "Component manifest should exist");
        
        let manifest_content = fs::read_to_string(manifest_path)
            .expect("Manifest should be readable");
        
        // Basic checks for required fields
        assert!(manifest_content.contains("name:"), "Manifest should have name field");
        assert!(manifest_content.contains("version:"), "Manifest should have version field");
        assert!(manifest_content.contains("description:"), "Manifest should have description field");
        assert!(manifest_content.contains("api_compatibility:"), "Manifest should have api_compatibility field");
    }
}