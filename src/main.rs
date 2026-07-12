mod cli;
mod commands;
mod signals;

use anyhow::Result;
use std::process::exit;

use crate::cli::{CliArgs, CliCommand, Parser};
use crate::commands::{count, next, pending, publish, run};

fn main() -> Result<()> {
    let cli = CliArgs::parse();
    let exit_code = match cli.command {
        CliCommand::Publish(args) => publish(args),
        CliCommand::Run(args) => run(args),
        CliCommand::Next(args) => next(args),
        CliCommand::Count => count(),
        CliCommand::Pending => pending(),
    }?;
    exit(exit_code);
}
