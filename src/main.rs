mod pdf;
mod cli;
mod tui;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::handle_command(Some(cli.command))
}