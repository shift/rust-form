use crate::{TemplateEngine, GenerationPipeline};
use rstest::*;

#[rstest]
fn test_codegen_engine_creation() {
    let _engine = TemplateEngine::new().expect("Should create template engine");
}

#[rstest]
fn test_generation_pipeline_creation() {
    let _pipeline = GenerationPipeline::new().expect("Should create generation pipeline");
}