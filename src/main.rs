mod pdf;
mod cli;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::handle_command(cli.command)
}