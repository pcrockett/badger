mod cli;
mod commands;

use anyhow::Result;

use crate::cli::{CliArgs, CliCommand, Parser};
use crate::commands::{count, next, pending, publish};

fn main() -> Result<()> {
    let cli = CliArgs::parse();
    match cli.command {
        CliCommand::Publish(args) => publish(args),
        CliCommand::Next(args) => next(args),
        CliCommand::Count => count(),
        CliCommand::Pending => pending(),
    }
}
