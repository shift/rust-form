use rustform_core::component::ComponentManifest;
use std::fs;

fn main() {
    let content = fs::read_to_string("./components/backend/compliance/gdpr-data-subject-rights/component.yml")
        .expect("Failed to read component.yml");
    
    println!("📁 Reading component manifest...");
    
    match ComponentManifest::from_yaml(&content) {
        Ok(manifest) => {
            println!("✅ Successfully parsed manifest!");
            println!("   Name: {}", manifest.name);
            println!("   Version: {}", manifest.version);
            println!("   Category: {:?}", manifest.category);
            println!("   Rust dependencies: {}", manifest.dependencies.rust.len());
            if let Some(ref templates) = manifest.templates {
                if let Some(ref generates) = templates.generates {
                    println!("   Templates: {}", generates.len());
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to parse manifest!");
            println!("Error: {}", e);
            
            // Try to parse as raw YAML to see the structure
            match serde_yaml::from_str::<serde_yaml::Value>(&content) {
                Ok(_) => println!("ℹ️  YAML is valid, so it's a structure mismatch"),
                Err(yaml_err) => println!("❌ YAML is invalid: {}", yaml_err),
            }
        }
    }
}