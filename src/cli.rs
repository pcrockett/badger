pub use clap::Parser;
use clap::{Args, Subcommand, ValueEnum};

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
    /// Notification content
    pub message: String,

    #[arg(short, long, default_value = "info")]
    /// Notification level (can be anything)
    pub level: String,

    #[arg(short, long)]
    /// Additional notification metadata in JSON format
    pub data: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    /// Command to run
    pub command: Vec<String>,

    #[arg(short, long, default_value = "sh")]
    /// Shell to use when running the command
    pub shell: String,
}

#[derive(Args)]
pub struct NextArgs {
    #[arg(short, long)]
    /// Just view notification contents; don't consume it
    pub peek: bool,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Quiet)]
    pub format: OutputFormat,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Json,
    Quiet,
}
