mod cli;
mod commands;

use crate::cli::{CliArgs, CliCommand, Parser};
use crate::commands::{count, next, pending, publish};

fn main() {
    let cli = CliArgs::parse();
    let exit_code = match cli.command {
        CliCommand::Publish(args) => publish(args),
        CliCommand::Next(args) => next(args),
        CliCommand::Count => count(),
        CliCommand::Pending => pending(),
    };
    std::process::exit(exit_code)
}
