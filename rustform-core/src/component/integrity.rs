use crate::error::{Error, Result};
use crate::component::Component;
use sha2::{Sha256, Sha384, Sha512, Digest};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct IntegrityVerifier {
    algorithms: HashMap<String, HashAlgorithm>,
}

#[derive(Debug, Clone)]
enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

#[derive(Debug, Clone)]
pub struct IntegrityInfo {
    pub algorithm: String,
    pub hash: String,
    pub size: Option<usize>,
}

impl IntegrityVerifier {
    pub fn new() -> Self {
        let mut algorithms = HashMap::new();
        algorithms.insert("sha256".to_string(), HashAlgorithm::Sha256);
        algorithms.insert("sha384".to_string(), HashAlgorithm::Sha384);
        algorithms.insert("sha512".to_string(), HashAlgorithm::Sha512);

        Self { algorithms }
    }

    /// Verify component integrity using SRI (Subresource Integrity) format
    pub fn verify(&self, component: &Component) -> Result<()> {
        if let Some(integrity_str) = &component.manifest.integrity {
            debug!("Verifying component integrity: {}", integrity_str);
            
            let integrity = self.parse_integrity(integrity_str)?;
            let computed_hash = self.compute_component_hash(component, &integrity.algorithm)?;
            
            if computed_hash != integrity.hash {
                return Err(Error::ComponentError(format!(
                    "Integrity verification failed for component '{}'. Expected: {}, Got: {}",
                    component.manifest.name,
                    integrity.hash,
                    computed_hash
                )));
            }
            
            debug!("Integrity verification passed for component: {}", component.manifest.name);
        } else {
            debug!("No integrity information provided for component: {}", component.manifest.name);
        }

        Ok(())
    }

    /// Generate integrity hash for a component
    pub fn generate_integrity(&self, component: &Component, algorithm: &str) -> Result<String> {
        let hash = self.compute_component_hash(component, algorithm)?;
        Ok(format!("{}-{}", algorithm, hash))
    }

    /// Parse SRI format integrity string (e.g., "sha384-ABC123...")
    fn parse_integrity(&self, integrity_str: &str) -> Result<IntegrityInfo> {
        let parts: Vec<&str> = integrity_str.splitn(2, '-').collect();
        if parts.len() != 2 {
            return Err(Error::ValidationError(format!(
                "Invalid integrity format. Expected 'algorithm-hash', got: {}",
                integrity_str
            )));
        }

        let algorithm = parts[0].to_string();
        let hash = parts[1].to_string();

        if !self.algorithms.contains_key(&algorithm) {
            return Err(Error::ValidationError(format!(
                "Unsupported hash algorithm: {}. Supported: {:?}",
                algorithm,
                self.algorithms.keys().collect::<Vec<_>>()
            )));
        }

        Ok(IntegrityInfo {
            algorithm,
            hash,
            size: None,
        })
    }

    /// Compute hash for entire component (all content)
    fn compute_component_hash(&self, component: &Component, algorithm: &str) -> Result<String> {
        let hash_algo = self.algorithms.get(algorithm)
            .ok_or_else(|| Error::ValidationError(format!("Unknown algorithm: {}", algorithm)))?;

        // Collect all component content into a deterministic byte stream
        let mut content_bytes = Vec::new();
        
        // Add manifest (normalize by serializing to YAML)
        let manifest_yaml = component.manifest.to_yaml()?;
        content_bytes.extend_from_slice(manifest_yaml.as_bytes());
        content_bytes.push(b'\n');

        // Add templates in sorted order for deterministic hashing
        let mut template_names: Vec<_> = component.content.templates.keys().collect();
        template_names.sort();
        for name in template_names {
            if let Some(template_content) = component.content.templates.get(name) {
                content_bytes.extend_from_slice(name.as_bytes());
                content_bytes.push(b':');
                content_bytes.extend_from_slice(template_content.as_bytes());
                content_bytes.push(b'\n');
            }
        }

        // Add assets in sorted order
        let mut asset_names: Vec<_> = component.content.assets.keys().collect();
        asset_names.sort();
        for name in asset_names {
            if let Some(asset_content) = component.content.assets.get(name) {
                content_bytes.extend_from_slice(name.as_bytes());
                content_bytes.push(b':');
                content_bytes.extend_from_slice(asset_content);
                content_bytes.push(b'\n');
            }
        }

        // Add hooks in sorted order
        let mut hook_names: Vec<_> = component.content.hooks.keys().collect();
        hook_names.sort();
        for name in hook_names {
            if let Some(hook_content) = component.content.hooks.get(name) {
                content_bytes.extend_from_slice(name.as_bytes());
                content_bytes.push(b':');
                content_bytes.extend_from_slice(hook_content.as_bytes());
                content_bytes.push(b'\n');
            }
        }

        // Compute hash based on algorithm
        let hash_bytes = match hash_algo {
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(&content_bytes);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(&content_bytes);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(&content_bytes);
                hasher.finalize().to_vec()
            }
        };

        // Encode as base64 (SRI standard) or hex
        Ok(hex::encode(hash_bytes))
    }

    /// Compute hash for individual file content
    pub fn compute_file_hash(&self, content: &[u8], algorithm: &str) -> Result<String> {
        let hash_algo = self.algorithms.get(algorithm)
            .ok_or_else(|| Error::ValidationError(format!("Unknown algorithm: {}", algorithm)))?;

        let hash_bytes = match hash_algo {
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(content);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(content);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(content);
                hasher.finalize().to_vec()
            }
        };

        Ok(hex::encode(hash_bytes))
    }

    /// Verify individual file integrity
    pub fn verify_file(&self, content: &[u8], expected_integrity: &str) -> Result<()> {
        let integrity = self.parse_integrity(expected_integrity)?;
        let computed_hash = self.compute_file_hash(content, &integrity.algorithm)?;
        
        if computed_hash != integrity.hash {
            return Err(Error::ComponentError(format!(
                "File integrity verification failed. Expected: {}, Got: {}",
                integrity.hash,
                computed_hash
            )));
        }

        Ok(())
    }

    /// Get supported hash algorithms
    pub fn supported_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }

    /// Get recommended algorithm (SHA-384 for SRI compliance)
    pub fn recommended_algorithm(&self) -> &str {
        "sha384"
    }

    /// Verify multiple integrity hashes (SRI allows multiple algorithms)
    pub fn verify_multiple(&self, component: &Component, integrity_strings: &[String]) -> Result<()> {
        if integrity_strings.is_empty() {
            return Ok(());
        }

        let mut verified = false;
        let mut errors = Vec::new();

        for integrity_str in integrity_strings {
            match self.verify_single_integrity(component, integrity_str) {
                Ok(()) => {
                    verified = true;
                    break;
                }
                Err(e) => errors.push(e),
            }
        }

        if !verified {
            return Err(Error::ComponentError(format!(
                "All integrity verifications failed: {:?}",
                errors
            )));
        }

        Ok(())
    }

    fn verify_single_integrity(&self, component: &Component, integrity_str: &str) -> Result<()> {
        let integrity = self.parse_integrity(integrity_str)?;
        let computed_hash = self.compute_component_hash(component, &integrity.algorithm)?;
        
        if computed_hash != integrity.hash {
            return Err(Error::ComponentError(format!(
                "Integrity verification failed. Expected: {}, Got: {}",
                integrity.hash,
                computed_hash
            )));
        }

        Ok(())
    }
}

impl Default for IntegrityVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{ComponentManifest, ComponentContent, ComponentInterface};
    use std::collections::HashMap;

    #[test]
    fn test_integrity_parsing() {
        let verifier = IntegrityVerifier::new();
        
        let integrity = verifier.parse_integrity("sha384-abc123def456").unwrap();
        assert_eq!(integrity.algorithm, "sha384");
        assert_eq!(integrity.hash, "abc123def456");
    }

    #[test]
    fn test_invalid_integrity_format() {
        let verifier = IntegrityVerifier::new();
        
        let result = verifier.parse_integrity("invalid-format-string");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_computation() {
        let verifier = IntegrityVerifier::new();
        
        let test_data = b"hello world";
        let hash = verifier.compute_file_hash(test_data, "sha256").unwrap();
        
        // Verify it's a valid hex string of expected length (64 chars for SHA-256)
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_supported_algorithms() {
        let verifier = IntegrityVerifier::new();
        let algorithms = verifier.supported_algorithms();
        
        assert!(algorithms.contains(&"sha256".to_string()));
        assert!(algorithms.contains(&"sha384".to_string()));
        assert!(algorithms.contains(&"sha512".to_string()));
    }

    #[test]
    fn test_component_hash_deterministic() {
        let verifier = IntegrityVerifier::new();
        
        let manifest = ComponentManifest {
            name: "test-component".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            author: None,
            license: None,
            homepage: None,
            repository: None,
            keywords: vec![],
            dependencies: HashMap::new(),
            provides: ComponentInterface::default(),
            integrity: None,
            files: vec![],
        };

        let content = ComponentContent {
            templates: {
                let mut map = HashMap::new();
                map.insert("template1".to_string(), "content1".to_string());
                map
            },
            assets: HashMap::new(),
            hooks: HashMap::new(),
        };

        let component = Component {
            manifest,
            content,
            resolved_path: std::path::PathBuf::from("/tmp"),
        };

        let hash1 = verifier.compute_component_hash(&component, "sha256").unwrap();
        let hash2 = verifier.compute_component_hash(&component, "sha256").unwrap();
        
        // Hash should be deterministic
        assert_eq!(hash1, hash2);
    }
}