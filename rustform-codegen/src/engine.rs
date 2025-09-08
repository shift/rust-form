use crate::error::CodeGenError;

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn new() -> Result<Self, CodeGenError> {
        Ok(Self)
    }
}