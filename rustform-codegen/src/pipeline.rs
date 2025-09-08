use crate::error::CodeGenError;

pub struct GenerationPipeline;

impl GenerationPipeline {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self) -> Result<(), CodeGenError> {
        println!("Generation pipeline - not implemented yet");
        Ok(())
    }
}