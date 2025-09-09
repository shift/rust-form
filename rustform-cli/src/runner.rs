use crate::cli::{Cli, Commands};
use crate::commands::{GenerateCommand, InitCommand};
use crate::error::CliError;
use miette::{IntoDiagnostic, Result};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub async fn run_cli() -> Result<(), CliError> {
    let cli = Cli::parse_args();

    // Setup logging
    setup_logging(cli.verbose, cli.quiet)?;

    info!("Starting rustform CLI");

    // Execute command
    match cli.command {
        Commands::Generate {
            config,
            output,
            force,
        } => {
            let cmd = GenerateCommand::new(config, output, force);
            cmd.execute().await?;
        }

        Commands::Init {
            name,
            directory,
            database,
        } => {
            let cmd = InitCommand::new(name, directory, database);
            cmd.execute().await?;
        }

        Commands::Component { component } => {
            component.execute().await?;
        }
    }

    Ok(())
}

fn setup_logging(verbose: bool, quiet: bool) -> Result<(), CliError> {
    let filter = if quiet {
        EnvFilter::new("warn")
    } else if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .into_diagnostic()?
    };

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_level(true)
                .with_writer(std::io::stderr),
        )
        .with(filter)
        .init();

    Ok(())
}
