use anyhow::Result;
use clap::Parser;
use flow_ctx::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Note { note } => flow_ctx::commands::run_note(note)?,
        Commands::Status => flow_ctx::commands::run_status()?,
        Commands::Resume => flow_ctx::commands::run_resume()?,
        Commands::History {
            limit,
            search,
            repo,
        } => flow_ctx::commands::run_history(limit, search, repo)?,
        Commands::Done => flow_ctx::commands::run_done()?,
    }

    Ok(())
}
