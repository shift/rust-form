use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GenerationContext {
    pub project_name: String,
}