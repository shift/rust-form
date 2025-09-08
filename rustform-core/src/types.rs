use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FieldType {
    Integer,
    String,
    Boolean,
    DateTime,
    Uuid,
    Json,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratedProject {
    pub name: String,
    pub files: Vec<GeneratedFile>,
}