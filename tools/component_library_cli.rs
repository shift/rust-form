use clap::{Arg, ArgMatches, Command};
use std::collections::HashMap;

mod component_generator;
use component_generator::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Component Library Generator")
        .version("1.0.0")
        .about("Rapidly generate rust-form component libraries")
        .subcommand(
            Command::new("generate")
                .about("Generate components")
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .value_name("CATEGORY")
                        .help("Component category (auth, payments, dashboards, ecommerce, cms)")
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("Component name")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .value_name("DESCRIPTION")
                        .help("Component description")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT_DIR")
                        .help("Output directory")
                        .default_value("./generated_components"),
                ),
        )
        .subcommand(
            Command::new("batch")
                .about("Generate multiple components from predefined specifications")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT_DIR")
                        .help("Output directory")
                        .default_value("./component_library"),
                )
                .arg(
                    Arg::new("categories")
                        .short('c')
                        .long("categories")
                        .value_name("CATEGORIES")
                        .help("Comma-separated list of categories to generate (default: all)"),
                ),
        )
        .subcommand(Command::new("list-categories").about("List available component categories"))
        .subcommand(
            Command::new("test-generated")
                .about("Test all generated components using rust-form component test")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .help("Path to component library")
                        .default_value("./component_library"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            generate_single_component(sub_matches).await?;
        }
        Some(("batch", sub_matches)) => {
            generate_component_library(sub_matches).await?;
        }
        Some(("list-categories", _)) => {
            list_categories();
        }
        Some(("test-generated", sub_matches)) => {
            test_generated_components(sub_matches).await?;
        }
        _ => {
            println!("No subcommand provided. Use --help for available commands.");
        }
    }

    Ok(())
}

async fn generate_single_component(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let category = matches.get_one::<String>("category").unwrap();
    let name = matches.get_one::<String>("name").unwrap();
    let description = matches.get_one::<String>("description").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();

    println!(
        "🚀 Generating component: {} in category: {}",
        name, category
    );

    let generator = ComponentLibraryGenerator::new(output_dir.clone());

    let config = ComponentGenerationConfig {
        description: description.clone(),
        author: "rust-form".to_string(),
        features: vec!["core".to_string()],
        dependencies: HashMap::new(),
        variables: HashMap::new(),
    };

    match generator.generate_component(category, name, config).await {
        Ok(component) => {
            println!("✅ Successfully generated component: {}", component.name);
            println!("   📁 Path: {}", component.path);
            println!("   📄 Templates: {}", component.template_files.len());
            println!(
                "   🏆 Initial Quality Score: {}/100",
                component.quality_score
            );
            println!();
            println!("🧪 To test the component:");
            println!("   rustform component test {}", component.path);
        }
        Err(e) => {
            eprintln!("❌ Failed to generate component: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn generate_component_library(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = matches.get_one::<String>("output").unwrap();
    let categories_filter = matches.get_one::<String>("categories");

    println!("🏗️  Generating rust-form component library...");
    println!("📁 Output directory: {}", output_dir);

    let generator = ComponentLibraryGenerator::new(output_dir.clone());
    let mut specs = get_component_library_specs();

    // Filter by categories if specified
    if let Some(categories) = categories_filter {
        let filter_set: std::collections::HashSet<String> = categories
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        specs.retain(|spec| filter_set.contains(&spec.category));
        println!("📋 Filtering to categories: {}", categories);
    }

    println!("🔄 Generating {} components...", specs.len());

    match generator.generate_component_library(specs).await {
        Ok(components) => {
            println!(
                "\n✅ Successfully generated {} components!",
                components.len()
            );

            // Group by category for reporting
            let mut by_category: std::collections::HashMap<String, Vec<&GeneratedComponent>> =
                std::collections::HashMap::new();
            for component in &components {
                by_category
                    .entry(component.category.clone())
                    .or_insert_with(Vec::new)
                    .push(component);
            }

            println!("\n📊 Component Library Summary:");
            for (category, category_components) in by_category {
                println!(
                    "   📂 {}: {} components",
                    category,
                    category_components.len()
                );
                for component in category_components {
                    println!(
                        "      • {} (Quality: {}/100)",
                        component.name, component.quality_score
                    );
                }
            }

            // Calculate overall statistics
            let total_quality: u32 = components.iter().map(|c| c.quality_score as u32).sum();
            let avg_quality = total_quality / components.len() as u32;
            let high_quality_count = components.iter().filter(|c| c.quality_score >= 80).count();

            println!("\n🏆 Quality Metrics:");
            println!("   Average Quality Score: {}/100", avg_quality);
            println!(
                "   High Quality Components (80+): {}/{}",
                high_quality_count,
                components.len()
            );
            println!(
                "   Component Library Grade: {}",
                get_library_grade(avg_quality)
            );

            println!("\n🧪 To test all components:");
            println!(
                "   ./tools/component_generator test-generated --path {}",
                output_dir
            );

            println!("\n🎯 Next Steps:");
            println!("   1. Test all generated components for quality assurance");
            println!("   2. Enhance components with additional templates as needed");
            println!("   3. Integrate with rust-form studio for visual management");
            println!("   4. Add components to the rust-form registry");
        }
        Err(e) => {
            eprintln!("❌ Failed to generate component library: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn list_categories() {
    let generator = ComponentLibraryGenerator::new("./temp".to_string());

    println!("📚 Available Component Categories:");
    println!();

    for (key, category) in &generator.categories {
        println!("🏷️  {} ({})", category.name, key);
        println!("   📖 {}", category.description);
        println!("   📄 Templates: {}", category.templates.len());
        println!("   📦 Dependencies: {}", category.common_dependencies.len());
        println!(
            "   🎯 Min Quality Score: {}",
            category.quality_requirements.min_test_coverage
        );
        println!();
    }

    println!("💡 Usage Examples:");
    println!("   Generate single component:");
    println!("     ./tools/component_generator generate -c auth -n my-auth-component -d \"Custom authentication system\"");
    println!();
    println!("   Generate all components:");
    println!("     ./tools/component_generator batch");
    println!();
    println!("   Generate specific categories:");
    println!("     ./tools/component_generator batch -c \"auth,payments,dashboards\"");
}

async fn test_generated_components(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let library_path = matches.get_one::<String>("path").unwrap();

    println!("🧪 Testing generated component library...");
    println!("📁 Library path: {}", library_path);

    // Find all component directories
    let components_dir = std::path::Path::new(library_path).join("components");
    if !components_dir.exists() {
        eprintln!(
            "❌ Components directory not found: {}",
            components_dir.display()
        );
        return Ok(());
    }

    let mut test_results = Vec::new();
    let mut total_components = 0;
    let mut passing_components = 0;

    // Recursively find all component manifests
    for category_entry in std::fs::read_dir(&components_dir)? {
        let category_entry = category_entry?;
        let category_path = category_entry.path();

        if category_path.is_dir() {
            let category_name = category_path.file_name().unwrap().to_string_lossy();
            println!("\n📂 Testing category: {}", category_name);

            for component_entry in std::fs::read_dir(&category_path)? {
                let component_entry = component_entry?;
                let component_path = component_entry.path();

                if component_path.is_dir() {
                    let component_name = component_path.file_name().unwrap().to_string_lossy();
                    let manifest_path = component_path.join("rustform-component.yml");

                    if manifest_path.exists() {
                        total_components += 1;
                        println!("   🧪 Testing: {}", component_name);

                        // Get rustform binary path from environment or use default
                        let rustform_binary = std::env::var("RUSTFORM_BINARY_PATH")
                            .unwrap_or_else(|_| "rustform".to_string());

                        // Run rustform component test
                        let output = std::process::Command::new(&rustform_binary)
                            .args(&[
                                "component",
                                "test",
                                component_path.to_string_lossy().as_ref(),
                            ])
                            .output();

                        match output {
                            Ok(result) => {
                                if result.status.success() {
                                    passing_components += 1;
                                    println!("      ✅ PASSED");
                                    test_results.push((
                                        component_name.to_string(),
                                        true,
                                        "".to_string(),
                                    ));
                                } else {
                                    let stderr = String::from_utf8_lossy(&result.stderr);
                                    println!(
                                        "      ❌ FAILED: {}",
                                        stderr.lines().next().unwrap_or("Unknown error")
                                    );
                                    test_results.push((
                                        component_name.to_string(),
                                        false,
                                        stderr.to_string(),
                                    ));
                                }
                            }
                            Err(e) => {
                                println!("      ❌ ERROR: {}", e);
                                test_results.push((
                                    component_name.to_string(),
                                    false,
                                    e.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    // Print summary
    println!("\n📊 Component Library Test Summary:");
    println!("   Total Components: {}", total_components);
    println!("   Passing: {}", passing_components);
    println!("   Failing: {}", total_components - passing_components);
    println!(
        "   Success Rate: {:.1}%",
        (passing_components as f64 / total_components as f64) * 100.0
    );

    if passing_components == total_components {
        println!("\n🎉 All components are passing! Component library is ready for production.");
    } else {
        println!("\n⚠️  Some components need attention. Review the failing tests above.");

        println!("\n📋 Failed Components:");
        for (name, passed, error) in test_results {
            if !passed {
                println!(
                    "   ❌ {}: {}",
                    name,
                    error.lines().next().unwrap_or("No error details")
                );
            }
        }
    }

    Ok(())
}

fn get_library_grade(avg_quality: u32) -> &'static str {
    match avg_quality {
        90..=100 => "A+ (Excellent)",
        80..=89 => "A (Very Good)",
        70..=79 => "B (Good)",
        60..=69 => "C (Fair)",
        50..=59 => "D (Poor)",
        _ => "F (Needs Improvement)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_grading() {
        assert_eq!(get_library_grade(95), "A+ (Excellent)");
        assert_eq!(get_library_grade(85), "A (Very Good)");
        assert_eq!(get_library_grade(75), "B (Good)");
        assert_eq!(get_library_grade(65), "C (Fair)");
        assert_eq!(get_library_grade(55), "D (Poor)");
        assert_eq!(get_library_grade(45), "F (Needs Improvement)");
    }
}
