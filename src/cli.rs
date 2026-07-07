pub use clap::Parser;
use clap::{Args, Subcommand};

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(name = "badger")]
#[command(about = "Publish and view notifications in your terminal")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Publish a notification
    Publish(PublishArgs),

    /// Run a command and publish a notification if it fails
    Run(RunArgs),

    /// Display the next notification in the list
    Next(NextArgs),

    /// Get notification count
    Count,

    /// Determine by exit code if notifications are pending
    Pending,
}

#[derive(Args)]
pub struct PublishArgs {
    #[arg(allow_hyphen_values = true)]
    pub message: String,

    #[arg(short, long)]
    pub level: Option<String>,

    #[arg(short, long)]
    pub data: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,

    #[arg(short, long)]
    pub shell: Option<String>,
}

#[derive(Args)]
pub struct NextArgs {
    #[arg(short, long)]
    pub peek: bool,

    #[arg(short, long)]
    pub format: Option<String>,
}
