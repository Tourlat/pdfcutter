mod cli;
mod pdf;
mod tui;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::handle_command(Some(cli.command))
}
