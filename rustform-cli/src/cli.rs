use crate::commands::*;

pub fn run_cli() -> Result<(), crate::error::CliError> {
    println!("CLI - not implemented yet");
    generate()?;
    init()?;
    Ok(())
}