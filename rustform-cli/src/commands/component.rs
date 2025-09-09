use crate::error::CliError;
use clap::Args;
use rustform_core::component::{ComponentSystem, ComponentUri};
use std::fs;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ComponentCommand {
    #[command(subcommand)]
    pub action: ComponentAction,
}

#[derive(clap::Subcommand, Debug)]
pub enum ComponentAction {
    /// Install a component
    Install {
        /// Component URI (e.g., github:org/repo@v1.0.0, path:./local)
        uri: String,
        /// Output directory for components
        #[arg(short, long, default_value = ".rustform/components")]
        output: PathBuf,
    },
    /// List installed components
    List {
        /// Components directory
        #[arg(short, long, default_value = ".rustform/components")]
        directory: PathBuf,
    },
    /// Remove a component
    Remove {
        /// Component name
        name: String,
        /// Components directory
        #[arg(short, long, default_value = ".rustform/components")]
        directory: PathBuf,
    },
    /// Validate component manifest
    Validate {
        /// Path to component directory or manifest file
        path: PathBuf,
    },
    /// Check component compatibility
    Compatibility {
        /// Component URI or path to manifest
        uri_or_path: String,
        /// rust-form version to check against (defaults to current)
        #[arg(long)]
        version: Option<String>,
    },
    /// Test a component
    Test {
        /// Component name or path to component directory
        component: String,
        /// Components directory
        #[arg(short, long, default_value = ".rustform/components")]
        directory: PathBuf,
        /// Generate test application for validation
        #[arg(long)]
        generate_test_app: bool,
        /// Run component's test suite only
        #[arg(long)]
        unit_tests_only: bool,
        /// Skip compatibility check
        #[arg(long)]
        skip_compatibility: bool,
    },
}

impl ComponentCommand {
    pub async fn execute(&self) -> Result<(), CliError> {
        match &self.action {
            ComponentAction::Install { uri, output } => self.install_component(uri, output).await,
            ComponentAction::List { directory } => self.list_components(directory).await,
            ComponentAction::Remove { name, directory } => {
                self.remove_component(name, directory).await
            }
            ComponentAction::Validate { path } => self.validate_component(path).await,
            ComponentAction::Compatibility {
                uri_or_path,
                version,
            } => {
                self.check_compatibility(uri_or_path, version.as_deref())
                    .await
            }
            ComponentAction::Test {
                component,
                directory,
                generate_test_app,
                unit_tests_only,
                skip_compatibility,
            } => {
                self.test_component(
                    component,
                    directory,
                    *generate_test_app,
                    *unit_tests_only,
                    *skip_compatibility,
                )
                .await
            }
        }
    }

    async fn install_component(&self, uri: &str, _output: &PathBuf) -> Result<(), CliError> {
        let component_uri = uri
            .parse::<ComponentUri>()
            .map_err(|e| CliError::ComponentError(format!("Invalid URI: {}", e)))?;

        let mut component_system = ComponentSystem::new().map_err(|e| {
            CliError::ComponentError(format!("Failed to initialize component system: {}", e))
        })?;

        println!("Installing component from: {}", uri);

        let component = component_system
            .install_component(&component_uri)
            .await
            .map_err(|e| CliError::ComponentError(format!("Failed to install component: {}", e)))?;

        println!(
            "✅ Successfully installed component: {}",
            component.manifest.name
        );
        println!("   Version: {}", component.manifest.version);
        if let Some(description) = &component.manifest.description {
            println!("   Description: {}", description);
        }

        Ok(())
    }

    async fn list_components(&self, directory: &PathBuf) -> Result<(), CliError> {
        if !directory.exists() {
            println!("No components directory found at: {}", directory.display());
            return Ok(());
        }

        let entries = fs::read_dir(directory).map_err(|e| {
            CliError::ComponentError(format!("Failed to read components directory: {}", e))
        })?;

        let mut components = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                CliError::ComponentError(format!("Failed to read directory entry: {}", e))
            })?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("rustform-component.yml");
                if manifest_path.exists() {
                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = serde_yaml::from_str::<
                            rustform_core::component::ComponentManifest,
                        >(&content)
                        {
                            components.push((
                                path.file_name().unwrap().to_string_lossy().to_string(),
                                manifest,
                            ));
                        }
                    }
                }
            }
        }

        if components.is_empty() {
            println!("No components installed.");
        } else {
            println!("Installed components:");
            for (dir_name, manifest) in components {
                println!("  {} ({})", manifest.name, manifest.version);
                if let Some(description) = &manifest.description {
                    println!("    {}", description);
                }
                println!("    Directory: {}", dir_name);
                println!();
            }
        }

        Ok(())
    }

    async fn remove_component(&self, name: &str, directory: &PathBuf) -> Result<(), CliError> {
        let component_path = directory.join(name);

        if !component_path.exists() {
            return Err(CliError::ComponentError(format!(
                "Component '{}' not found",
                name
            )));
        }

        fs::remove_dir_all(&component_path)
            .map_err(|e| CliError::ComponentError(format!("Failed to remove component: {}", e)))?;

        println!("✅ Successfully removed component: {}", name);
        Ok(())
    }

    async fn validate_component(&self, path: &PathBuf) -> Result<(), CliError> {
        let manifest_path =
            if path.is_file() && path.file_name().unwrap() == "rustform-component.yml" {
                path.clone()
            } else if path.is_dir() {
                path.join("rustform-component.yml")
            } else {
                return Err(CliError::ComponentError(
                    "Path must be a directory or rustform-component.yml file".to_string(),
                ));
            };

        if !manifest_path.exists() {
            return Err(CliError::ComponentError(
                "Component manifest not found".to_string(),
            ));
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| CliError::ComponentError(format!("Failed to read manifest: {}", e)))?;

        let manifest =
            serde_yaml::from_str::<rustform_core::component::ComponentManifest>(&content)
                .map_err(|e| CliError::ComponentError(format!("Invalid manifest format: {}", e)))?;

        manifest
            .validate()
            .map_err(|e| CliError::ComponentError(format!("Manifest validation failed: {}", e)))?;

        println!("✅ Component manifest is valid:");
        println!("   Name: {}", manifest.name);
        println!("   Version: {}", manifest.version);
        if let Some(description) = &manifest.description {
            println!("   Description: {}", description);
        }
        println!("   API Compatibility:");
        println!(
            "     API Version: {}",
            manifest.api_compatibility.api_version
        );
        println!(
            "     Min Version: {}",
            manifest.api_compatibility.min_version
        );
        if let Some(ref max_version) = manifest.api_compatibility.max_version {
            println!("     Max Version: {}", max_version);
        }
        if let Some(ref features) = manifest.api_compatibility.required_features {
            println!("     Required Features: {}", features.join(", "));
        }
        if manifest.api_compatibility.experimental.unwrap_or(false) {
            println!("     ⚠️  Uses experimental APIs");
        }
        println!(
            "   Templates: {}",
            manifest
                .provides
                .as_ref()
                .map(|p| p.templates.len())
                .unwrap_or(0)
        );
        println!(
            "   Assets: {}",
            manifest
                .provides
                .as_ref()
                .map(|p| p.assets.len())
                .unwrap_or(0)
        );
        println!(
            "   Hooks: {}",
            manifest
                .provides
                .as_ref()
                .map(|p| p.hooks.len())
                .unwrap_or(0)
        );

        Ok(())
    }

    async fn test_component(
        &self,
        component: &str,
        directory: &PathBuf,
        generate_test_app: bool,
        unit_tests_only: bool,
        skip_compatibility: bool,
    ) -> Result<(), CliError> {
        println!("🧪 Testing component: {}", component);

        // Find component path
        let component_path = if PathBuf::from(component).exists() {
            // Direct path provided
            PathBuf::from(component)
        } else {
            // Look in components directory
            let path = directory.join(component);
            if !path.exists() {
                return Err(CliError::ComponentError(format!(
                    "Component '{}' not found in directory '{}'",
                    component,
                    directory.display()
                )));
            }
            path
        };

        // Look for manifest file - try both possible names
        let manifest_path = {
            let rustform_manifest = component_path.join("rustform-component.yml");
            let component_manifest = component_path.join("component.yml");

            println!("🔍 Looking for manifest files in: {:?}", component_path);
            println!(
                "   Checking: {:?} (exists: {})",
                rustform_manifest,
                rustform_manifest.exists()
            );
            println!(
                "   Checking: {:?} (exists: {})",
                component_manifest,
                component_manifest.exists()
            );

            if rustform_manifest.exists() {
                rustform_manifest
            } else if component_manifest.exists() {
                component_manifest
            } else {
                return Err(CliError::ComponentError("Component manifest not found (looking for rustform-component.yml or component.yml)".to_string()));
            }
        };

        // Load and validate manifest
        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| CliError::ComponentError(format!("Failed to read manifest: {}", e)))?;

        println!("📄 Manifest file found: {:?}", manifest_path);
        println!("📝 Parsing manifest content ({} bytes)...", content.len());

        let manifest = serde_yaml::from_str::<rustform_core::component::ComponentManifest>(
            &content,
        )
        .map_err(|e| {
            println!("❌ Parsing failed with error: {}", e);

            // Try to validate raw YAML first
            match serde_yaml::from_str::<serde_yaml::Value>(&content) {
                Ok(_) => println!("ℹ️  YAML syntax is valid - this is a structure/schema mismatch"),
                Err(yaml_err) => println!("❌ YAML syntax error: {}", yaml_err),
            }

            CliError::ComponentError(format!("Invalid manifest format: {}", e))
        })?;

        println!("📋 Component: {} v{}", manifest.name, manifest.version);

        // Phase 1: Manifest validation
        println!("\n🔍 Phase 1: Manifest Validation");
        manifest
            .validate()
            .map_err(|e| CliError::ComponentError(format!("Manifest validation failed: {}", e)))?;
        println!("   ✅ Manifest is valid");

        // Phase 2: Compatibility check
        if !skip_compatibility {
            println!("\n🔗 Phase 2: Compatibility Check");
            let rust_form_version = env!("CARGO_PKG_VERSION");
            let status = manifest
                .compatibility_status(rust_form_version)
                .map_err(|e| {
                    CliError::ComponentError(format!("Failed to check compatibility: {}", e))
                })?;

            if !status.is_compatible() {
                println!("   ❌ {}", status.message());
                return Err(CliError::ComponentError(
                    "Component is not compatible with current rust-form version".to_string(),
                ));
            }
            println!(
                "   ✅ Component is compatible with rust-form v{}",
                rust_form_version
            );
        } else {
            println!("\n⏭️  Phase 2: Compatibility Check (Skipped)");
        }

        // Phase 3: Unit tests (if component has test files)
        println!("\n🧪 Phase 3: Component Unit Tests");
        let test_results = self.run_component_unit_tests(&component_path).await?;

        if unit_tests_only {
            self.print_test_summary(&manifest.name, &test_results);
            return Ok(());
        }

        // Phase 4: Integration test application generation
        if generate_test_app {
            println!("\n🏗️  Phase 4: Test Application Generation");
            self.generate_test_application(&manifest, &component_path)
                .await?;
            println!("   ✅ Test application generated successfully");
        } else {
            println!("\n⏭️  Phase 4: Test Application Generation (Skipped)");
        }

        // Phase 5: Quality metrics
        println!("\n📊 Phase 5: Quality Assessment");
        let quality_metrics = self
            .assess_component_quality(&manifest, &component_path)
            .await?;

        // Print comprehensive results
        self.print_test_summary(&manifest.name, &test_results);
        self.print_quality_metrics(&quality_metrics);

        Ok(())
    }

    async fn run_component_unit_tests(
        &self,
        component_path: &PathBuf,
    ) -> Result<ComponentTestResults, CliError> {
        let mut results = ComponentTestResults {
            unit_tests_passed: 0,
            unit_tests_failed: 0,
            test_files_found: 0,
            coverage_percentage: None,
        };

        // Look for test files in the component
        let test_patterns = ["tests/", "src/", "test/"];
        let mut test_files = Vec::new();

        for pattern in &test_patterns {
            let test_dir = component_path.join(pattern);
            if test_dir.exists() {
                if let Ok(entries) = fs::read_dir(&test_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            let file_name = path.file_name().unwrap().to_string_lossy();
                            if file_name.ends_with("_test.rs")
                                || file_name.ends_with(".test.js")
                                || file_name.starts_with("test_")
                            {
                                test_files.push(path);
                            }
                        }
                    }
                }
            }
        }

        results.test_files_found = test_files.len();

        if test_files.is_empty() {
            println!("   ℹ️  No test files found in component");
            return Ok(results);
        }

        println!("   📁 Found {} test files", test_files.len());

        // Check if there's a Cargo.toml for Rust tests
        let cargo_toml = component_path.join("Cargo.toml");
        if cargo_toml.exists() {
            println!("   🦀 Running Rust tests...");
            match self.run_cargo_test(&component_path).await {
                Ok((passed, failed)) => {
                    results.unit_tests_passed = passed;
                    results.unit_tests_failed = failed;
                    println!(
                        "   ✅ Rust tests completed: {} passed, {} failed",
                        passed, failed
                    );
                }
                Err(e) => {
                    println!("   ⚠️  Failed to run Rust tests: {}", e);
                    results.unit_tests_failed = 1; // Mark as failed if we can't run tests
                }
            }
        }

        // Check for package.json for JavaScript/TypeScript tests
        let package_json = component_path.join("package.json");
        if package_json.exists() {
            println!("   📦 Running JavaScript/TypeScript tests...");
            match self.run_npm_test(&component_path).await {
                Ok((passed, failed)) => {
                    results.unit_tests_passed += passed;
                    results.unit_tests_failed += failed;
                    println!(
                        "   ✅ JS/TS tests completed: {} passed, {} failed",
                        passed, failed
                    );
                }
                Err(e) => {
                    println!("   ⚠️  Failed to run JS/TS tests: {}", e);
                    results.unit_tests_failed += 1;
                }
            }
        }

        Ok(results)
    }

    async fn run_cargo_test(&self, component_path: &PathBuf) -> Result<(usize, usize), CliError> {
        use std::process::Command;

        // Check if we need to run in Nix environment
        let in_nix_shell = std::env::var("IN_NIX_SHELL").is_ok();

        // Convert to absolute path
        let absolute_component_path = if component_path.is_absolute() {
            component_path.clone()
        } else {
            std::env::current_dir()
                .map_err(|e| CliError::ComponentError(format!("Failed to get current dir: {}", e)))?
                .join(component_path)
        };

        let manifest_path = absolute_component_path.join("Cargo.toml");

        if !manifest_path.exists() {
            return Err(CliError::ComponentError(format!(
                "Cargo.toml not found at: {}",
                manifest_path.display()
            )));
        }

        let mut cmd = if in_nix_shell {
            Command::new("cargo")
        } else {
            let mut nix_cmd = Command::new("nix");
            nix_cmd.args(&["develop", "-c", "--", "cargo"]);
            nix_cmd
        };

        cmd.arg("test")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .current_dir(&absolute_component_path);

        println!(
            "   Running: cargo test --manifest-path {} (from {})",
            manifest_path.display(),
            absolute_component_path.display()
        );

        let output = cmd
            .output()
            .map_err(|e| CliError::ComponentError(format!("Failed to run cargo test: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            // Parse output to count test results
            // Look for patterns like "test result: ok. 5 passed; 0 failed"
            let mut passed = 0;
            let mut failed = 0;

            if let Some(result_line) = stdout.lines().find(|line| line.contains("test result:")) {
                // Extract passed/failed counts from "test result: ok. X passed; Y failed"
                if let Some(passed_part) = result_line.split(" passed").next() {
                    if let Some(passed_str) = passed_part.split_whitespace().last() {
                        passed = passed_str.parse().unwrap_or(0);
                    }
                }
                if let Some(failed_part) = result_line.split(" failed").next() {
                    if let Some(failed_str) = failed_part.split("; ").nth(1) {
                        if let Some(failed_num) = failed_str.split_whitespace().next() {
                            failed = failed_num.parse().unwrap_or(0);
                        }
                    }
                }
            }

            // If we couldn't parse the summary, count individual test results
            if passed == 0 && failed == 0 {
                passed = stdout
                    .matches("test ")
                    .filter(|line| line.contains(" ... ok"))
                    .count();
                failed = stdout
                    .matches("test ")
                    .filter(|line| line.contains(" ... FAILED"))
                    .count();
            }

            Ok((passed, failed))
        } else {
            Err(CliError::ComponentError(format!(
                "Cargo test failed: {}\n{}",
                stderr, stdout
            )))
        }
    }

    async fn run_npm_test(&self, component_path: &PathBuf) -> Result<(usize, usize), CliError> {
        use std::process::Command;

        let output = Command::new("npm")
            .arg("test")
            .current_dir(component_path)
            .output()
            .map_err(|e| CliError::ComponentError(format!("Failed to run npm test: {}", e)))?;

        if output.status.success() {
            // Parse npm test output (simplified)
            let stdout = String::from_utf8_lossy(&output.stdout);
            let passed = stdout.matches("✓").count();
            let failed = stdout.matches("✗").count().max(stdout.matches("×").count());
            Ok((passed, failed))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(CliError::ComponentError(format!(
                "npm test failed: {}",
                stderr
            )))
        }
    }

    async fn generate_test_application(
        &self,
        manifest: &rustform_core::component::ComponentManifest,
        component_path: &PathBuf,
    ) -> Result<(), CliError> {
        let test_app_path = component_path.join("test-app");

        // Create test application directory
        if test_app_path.exists() {
            fs::remove_dir_all(&test_app_path).map_err(|e| {
                CliError::ComponentError(format!("Failed to clean test app directory: {}", e))
            })?;
        }

        fs::create_dir_all(&test_app_path).map_err(|e| {
            CliError::ComponentError(format!("Failed to create test app directory: {}", e))
        })?;

        // Generate a simple test configuration that uses this component
        let test_config = format!(
            r#"project_name: "{}-test-app"
version: "0.1.0"

database:
  type: sqlite
  url_env: DATABASE_URL

server:
  host: "127.0.0.1"
  port: 3000

components:
  - name: "{}"
    path: "../"

api:
  models:
    TestModel:
      table_name: test_models
      fields:
        id:
          type: integer
          primary_key: true
          auto_increment: true
        name:
          type: string
          max_length: 255
          required: true
        created_at:
          type: datetime
          auto_now_add: true

  endpoints:
    - path: /test
      model: TestModel
      crud:
        create: true
        read_all: true
        read_one: true
        update: true
        delete: true

middleware:
  - logger: true
  - cors:
      allow_origin: "*"
      allow_methods: ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"]
      allow_headers: ["Content-Type", "Authorization"]
      allow_credentials: false
"#,
            manifest.name, manifest.name
        );

        let config_path = test_app_path.join("rustform.yml");
        fs::write(&config_path, test_config)
            .map_err(|e| CliError::ComponentError(format!("Failed to write test config: {}", e)))?;

        // Try to generate the test application using rustform
        println!("   🔧 Generating test application with component...");

        // Check if we're in Nix environment and use appropriate command
        let mut cmd = if std::env::var("IN_NIX_SHELL").is_ok() {
            // We're in nix shell, use cargo run directly
            let mut nix_cmd = std::process::Command::new("cargo");
            nix_cmd.args(&["run", "--bin", "rustform", "--"]);
            nix_cmd
        } else {
            // Use nix develop to run the command
            let mut nix_cmd = std::process::Command::new("nix");
            nix_cmd.args(&[
                "develop", "-c", "--", "cargo", "run", "--bin", "rustform", "--",
            ]);
            nix_cmd
        };

        let output = cmd
            .arg("generate")
            .arg(config_path)
            .arg("--output")
            .arg(&test_app_path)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("   ✅ Test application generated successfully");

                    // Try to build the test application
                    let build_output = std::process::Command::new("cargo")
                        .arg("check")
                        .current_dir(&test_app_path)
                        .output();

                    match build_output {
                        Ok(build_result) => {
                            if build_result.status.success() {
                                println!("   ✅ Test application compiles successfully");
                            } else {
                                let stderr = String::from_utf8_lossy(&build_result.stderr);
                                println!(
                                    "   ⚠️  Test application compilation warnings: {}",
                                    stderr
                                );
                            }
                        }
                        Err(e) => {
                            println!("   ⚠️  Failed to compile test application: {}", e);
                        }
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    return Err(CliError::ComponentError(format!(
                        "Failed to generate test application: {}",
                        stderr
                    )));
                }
            }
            Err(e) => {
                return Err(CliError::ComponentError(format!(
                    "Failed to run rustform generate: {}",
                    e
                )));
            }
        }

        Ok(())
    }

    async fn assess_component_quality(
        &self,
        manifest: &rustform_core::component::ComponentManifest,
        component_path: &PathBuf,
    ) -> Result<ComponentQualityMetrics, CliError> {
        let mut metrics = ComponentQualityMetrics {
            has_documentation: false,
            has_examples: false,
            has_tests: false,
            template_count: manifest
                .provides
                .as_ref()
                .map(|p| p.templates.len())
                .unwrap_or(0),
            asset_count: manifest
                .provides
                .as_ref()
                .map(|p| p.assets.len())
                .unwrap_or(0),
            hook_count: manifest
                .provides
                .as_ref()
                .map(|p| p.hooks.len())
                .unwrap_or(0),
            quality_score: 0.0,
        };

        // Check for documentation
        let doc_files = ["README.md", "docs/", "CHANGELOG.md"];
        for doc_file in &doc_files {
            if component_path.join(doc_file).exists() {
                metrics.has_documentation = true;
                break;
            }
        }

        // Check for examples
        let example_dirs = ["examples/", "example/", "samples/"];
        for example_dir in &example_dirs {
            if component_path.join(example_dir).exists() {
                metrics.has_examples = true;
                break;
            }
        }

        // Check for tests
        let test_dirs = ["tests/", "test/", "src/"];
        for test_dir in &test_dirs {
            let test_path = component_path.join(test_dir);
            if test_path.exists() {
                if let Ok(entries) = fs::read_dir(&test_path) {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.contains("test") {
                            metrics.has_tests = true;
                            break;
                        }
                    }
                }
            }
        }

        // Calculate quality score
        let mut score = 0.0;

        // Base score from manifest completeness
        if manifest.description.is_some() {
            score += 10.0;
        }
        if manifest.author.is_some() {
            score += 5.0;
        }
        if manifest.license.is_some() {
            score += 5.0;
        }
        if manifest.repository.is_some() {
            score += 5.0;
        }

        // Score from content
        if metrics.has_documentation {
            score += 20.0;
        }
        if metrics.has_examples {
            score += 15.0;
        }
        if metrics.has_tests {
            score += 25.0;
        }

        // Score from functionality
        score += metrics.template_count as f64 * 2.0;
        score += metrics.asset_count as f64 * 1.0;
        score += metrics.hook_count as f64 * 3.0;

        // Cap at 100
        metrics.quality_score = score.min(100.0);

        println!(
            "   📋 Documentation: {}",
            if metrics.has_documentation {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "   📚 Examples: {}",
            if metrics.has_examples { "✅" } else { "❌" }
        );
        println!(
            "   🧪 Tests: {}",
            if metrics.has_tests { "✅" } else { "❌" }
        );
        println!("   📄 Templates: {}", metrics.template_count);
        println!("   🎨 Assets: {}", metrics.asset_count);
        println!("   🪝 Hooks: {}", metrics.hook_count);

        Ok(metrics)
    }

    fn print_test_summary(&self, component_name: &str, results: &ComponentTestResults) {
        println!("\n📊 Test Summary for '{}':", component_name);
        println!("   Test Files: {}", results.test_files_found);
        println!("   Tests Passed: {}", results.unit_tests_passed);
        println!("   Tests Failed: {}", results.unit_tests_failed);

        if let Some(coverage) = results.coverage_percentage {
            println!("   Test Coverage: {:.1}%", coverage);
        }

        let total_tests = results.unit_tests_passed + results.unit_tests_failed;
        if total_tests > 0 {
            let success_rate = (results.unit_tests_passed as f64 / total_tests as f64) * 100.0;
            println!("   Success Rate: {:.1}%", success_rate);

            if results.unit_tests_failed == 0 {
                println!("   Status: ✅ All tests passing");
            } else {
                println!(
                    "   Status: ❌ {} test(s) failing",
                    results.unit_tests_failed
                );
            }
        } else if results.test_files_found > 0 {
            println!("   Status: ⚠️  Test files found but no tests executed");
        } else {
            println!("   Status: ℹ️  No tests found");
        }
    }

    fn print_quality_metrics(&self, metrics: &ComponentQualityMetrics) {
        println!("\n🏆 Quality Assessment:");
        println!("   Overall Score: {:.1}/100", metrics.quality_score);

        let grade = match metrics.quality_score {
            90.0..=100.0 => "A+ (Excellent)",
            80.0..=89.9 => "A (Very Good)",
            70.0..=79.9 => "B (Good)",
            60.0..=69.9 => "C (Fair)",
            50.0..=59.9 => "D (Poor)",
            _ => "F (Needs Improvement)",
        };

        println!("   Grade: {}", grade);

        if metrics.quality_score < 70.0 {
            println!("\n💡 Improvement Suggestions:");
            if !metrics.has_documentation {
                println!("   • Add comprehensive documentation (README.md, API docs)");
            }
            if !metrics.has_examples {
                println!("   • Include usage examples");
            }
            if !metrics.has_tests {
                println!("   • Add unit tests and integration tests");
            }
            if metrics.template_count == 0 {
                println!("   • Consider providing template files");
            }
        }
    }

    async fn check_compatibility(
        &self,
        uri_or_path: &str,
        version: Option<&str>,
    ) -> Result<(), CliError> {
        let rust_form_version = version.unwrap_or(env!("CARGO_PKG_VERSION"));

        // Check if it's a local path first
        let path = PathBuf::from(uri_or_path);
        println!("🔍 Checking path: {:?} (exists: {})", path, path.exists());

        let manifest = if path.exists() {
            // Local component - try both possible manifest filenames
            let manifest_path = if path.is_file() {
                println!("📄 Path is a file");
                let filename = path.file_name().unwrap();
                if filename == "rustform-component.yml" || filename == "component.yml" {
                    path
                } else {
                    return Err(CliError::ComponentError(
                        "File must be rustform-component.yml or component.yml".to_string(),
                    ));
                }
            } else if path.is_dir() {
                println!("📁 Path is a directory, looking for manifest files...");
                // Try rustform-component.yml first, then component.yml
                let rustform_manifest = path.join("rustform-component.yml");
                let component_manifest = path.join("component.yml");

                println!(
                    "   Checking: {:?} (exists: {})",
                    rustform_manifest,
                    rustform_manifest.exists()
                );
                println!(
                    "   Checking: {:?} (exists: {})",
                    component_manifest,
                    component_manifest.exists()
                );

                if rustform_manifest.exists() {
                    rustform_manifest
                } else if component_manifest.exists() {
                    component_manifest
                } else {
                    return Err(CliError::ComponentError("No manifest file found (looking for rustform-component.yml or component.yml)".to_string()));
                }
            } else {
                return Err(CliError::ComponentError(
                    "Path must be a directory or manifest file".to_string(),
                ));
            };

            if !manifest_path.exists() {
                return Err(CliError::ComponentError(
                    "Component manifest not found".to_string(),
                ));
            }

            let content = fs::read_to_string(&manifest_path)
                .map_err(|e| CliError::ComponentError(format!("Failed to read manifest: {}", e)))?;

            println!("📄 Manifest file found: {:?}", manifest_path);
            println!("📝 Parsing manifest content ({} bytes)...", content.len());

            serde_yaml::from_str::<rustform_core::component::ComponentManifest>(&content).map_err(
                |e| {
                    println!("❌ Parsing failed with error: {}", e);

                    // Try to validate raw YAML first
                    match serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        Ok(_) => println!(
                            "ℹ️  YAML syntax is valid - this is a structure/schema mismatch"
                        ),
                        Err(yaml_err) => println!("❌ YAML syntax error: {}", yaml_err),
                    }

                    CliError::ComponentError(format!("Invalid manifest format: {}", e))
                },
            )?
        } else if let Ok(component_uri) = uri_or_path.parse::<ComponentUri>() {
            // Remote component
            let component_system = ComponentSystem::new().map_err(|e| {
                CliError::ComponentError(format!("Failed to initialize component system: {}", e))
            })?;

            println!("🔍 Fetching component manifest from: {}", uri_or_path);
            component_system
                .fetch_manifest(&component_uri)
                .await
                .map_err(|e| CliError::ComponentError(format!("Failed to fetch manifest: {}", e)))?
        } else {
            return Err(CliError::ComponentError(
                "Invalid path or URI format".to_string(),
            ));
        };

        println!("📋 Component: {} v{}", manifest.name, manifest.version);
        println!(
            "🔧 Checking compatibility with rust-form v{}",
            rust_form_version
        );
        println!();

        let status = manifest
            .compatibility_status(rust_form_version)
            .map_err(|e| {
                CliError::ComponentError(format!("Failed to check compatibility: {}", e))
            })?;

        println!("{}", status.message());

        if !status.is_compatible() {
            println!();
            println!("💡 Suggestions:");
            match status {
                rustform_core::component::CompatibilityStatus::TooOld {
                    current,
                    required_min,
                } => {
                    println!(
                        "   • Upgrade rust-form from {} to {} or higher",
                        current, required_min
                    );
                    println!("   • Or find an older version of this component");
                }
                rustform_core::component::CompatibilityStatus::TooNew {
                    current,
                    supported_max,
                } => {
                    println!("   • Update the component to support rust-form {}", current);
                    println!("   • Or downgrade rust-form to {} or lower", supported_max);
                    println!("   • This may still work but is untested");
                }
                _ => {}
            }
            return Err(CliError::ComponentError(
                "Component is not compatible".to_string(),
            ));
        }

        println!();
        println!("✅ Component is compatible!");

        if let Some(ref features) = manifest.api_compatibility.required_features {
            println!();
            println!("📦 Required Features:");
            for feature in features {
                println!("   • {}", feature);
            }
        }

        if manifest.api_compatibility.experimental.unwrap_or(false) {
            println!();
            println!("⚠️  Warning: This component uses experimental APIs that may change.");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct ComponentTestResults {
    unit_tests_passed: usize,
    unit_tests_failed: usize,
    test_files_found: usize,
    coverage_percentage: Option<f64>,
}

#[derive(Debug)]
struct ComponentQualityMetrics {
    has_documentation: bool,
    has_examples: bool,
    has_tests: bool,
    template_count: usize,
    asset_count: usize,
    hook_count: usize,
    quality_score: f64,
}
