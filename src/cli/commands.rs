use clap::Parser;

/// PDF Cutter - A CLI tool for merging and deleting pages from PDF files
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    override_usage = "cargo run -- -<COMMAND> -<COMMAND_ARGS>"
)]
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

    /// Split a PDF into multiple smaller PDFs
    Split {
        // Input PDF file
        #[arg(short = 'i', long)]
        input: String,

        // Pages slices (e.g., "1-3", "5", "7-9", "1,3,5-7")
        #[arg(short = 'p', long)]
        pages: String,

        // Output file prefix, e.g., "output_" will create files like "output_1.pdf", "output_2.pdf", etc.
        #[arg(short = 'o', long = "output-prefix")]
        output_prefix: String,

        /// Use named segments format (name:pages)
        #[arg(long)]
        named: bool,
    },

    /// Launch Terminal User Interface
    Tui,
}
