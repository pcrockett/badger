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

    /// Display the next notification in the list
    Next(NextArgs),

    /// Get notification count
    Count,

    /// Determine by exit code if notifications are pending
    Pending,
}

#[derive(Args)]
pub struct PublishArgs {
    pub message: String,

    #[arg(short, long)]
    pub level: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub data: Option<String>,
}

#[derive(Args)]
pub struct NextArgs {
    #[arg(short, long)]
    pub peek: bool,

    #[arg(short, long)]
    pub format: Option<String>,
}
