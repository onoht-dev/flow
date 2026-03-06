use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "flow")]
#[command(author = "onoht")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool for developer context preservation", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Save what you're currently working on
    Note {
        /// The note describing what you're working on
        #[arg(required = true)]
        note: String,
    },

    /// Show the current context
    Status,

    /// Get help resuming work after an interruption
    Resume,

    /// View past contexts
    History {
        /// Number of past contexts to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Filter by note text (case-insensitive)
        #[arg(long)]
        search: Option<String>,

        /// Filter by repository name (case-insensitive)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Mark the current task as complete and clear context
    Done,
}
