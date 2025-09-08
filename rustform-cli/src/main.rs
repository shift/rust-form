use rustform_cli::run_cli;

#[tokio::main]
async fn main() -> miette::Result<()> {
    run_cli().await?;
    Ok(())
}