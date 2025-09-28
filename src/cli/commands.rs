use clap::Parser;

/// PDF Cutter - A CLI tool for merging and deleting pages from PDF files
#[derive(Parser, Debug)]
#[command(author, version, override_usage = "cargo run -- -<COMMAND> -<COMMAND_ARGS>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Merge multiple PDFs into one
    Merge {
        /// Output PDF file
        #[arg(short, long)]
        output: String,
        
        /// Input PDF files (at least 2)
        #[arg(required = true)]
        inputs: Vec<String>,
    },
    /// Delete pages from a PDF
    Delete {
        /// Input PDF file
        #[arg(short, long)]
        input: String,
        
        /// Output PDF file
        #[arg(short, long)]
        output: String,
        
        /// Pages to delete (e.g., "3", "3-5", "1,3,5-7")
        #[arg(short = 'p', long)]
        pages: String,
    },

    /// Launch Terminal User Interface
    Tui,
}