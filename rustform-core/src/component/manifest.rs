use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    
    pub dependencies: HashMap<String, String>,
    pub provides: ComponentInterface,
    
    pub integrity: Option<String>,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentInterface {
    pub templates: Vec<TemplateSpec>,
    pub assets: Vec<AssetSpec>,
    pub hooks: Vec<HookSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateSpec {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub variables: Vec<VariableSpec>,
    pub target: TemplateTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateTarget {
    Frontend,
    Backend,
    Migration,
    Config,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableSpec {
    pub name: String,
    pub type_name: String,
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetSpec {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Style,
    Script,
    Image,
    Font,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookSpec {
    pub name: String,
    pub phase: HookPhase,
    pub script: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookPhase {
    PreGenerate,
    PostGenerate,
    PreBuild,
    PostBuild,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub manifest: ComponentManifest,
    pub content: ComponentContent,
    pub resolved_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ComponentContent {
    pub templates: HashMap<String, String>,
    pub assets: HashMap<String, Vec<u8>>,
    pub hooks: HashMap<String, String>,
}

impl ComponentManifest {
    pub fn from_yaml(content: &str) -> Result<Self> {
        serde_yaml::from_str(content).map_err(Into::into)
    }

    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(crate::error::Error::ValidationError(
                "Component name cannot be empty".to_string()
            ));
        }

        if self.version.is_empty() {
            return Err(crate::error::Error::ValidationError(
                "Component version cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for ComponentInterface {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
            assets: Vec::new(),
            hooks: Vec::new(),
        }
    }
}