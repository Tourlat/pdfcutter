mod pdf;

use clap::Parser;
use anyhow::{Result, bail};
use std::path::Path;

/// Simple CLI to merge multiple PDFs into one or delete pages from a PDF
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Merge { inputs, output } => {
            if inputs.len() < 2 {
                bail!("You must provide at least two input PDF files");
            }
            for p in &inputs {
                if !Path::new(p).exists() {
                    bail!("Input file does not exist: {}", p);
                }
            }

            pdf::merge_pdfs(&inputs, &output)?;
            println!("✅ Merged {} files into '{}'", inputs.len(), output);
        }
        Commands::Delete { input, output, pages } => {
            if !Path::new(&input).exists() {
                bail!("Input file does not exist: {}", input);
            }

            let pages_to_delete = parse_page_ranges(&pages)?;
            pdf::delete_pages(&input, &output, &pages_to_delete)?;
            println!("✅ Deleted pages {} from '{}' and saved to '{}'", pages, input, output);
        }
    }
    Ok(())
}

/// Parse page ranges like "3", "3-5", "1,3,5-7" into a Vec of page numbers
fn parse_page_ranges(pages_str: &str) -> Result<Vec<u32>> {
    let mut pages = Vec::new();
    
    for part in pages_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                bail!("Invalid page range format: {}", part);
            }
            let start: u32 = range_parts[0].parse()
                .map_err(|_| anyhow::anyhow!("Invalid page number: {}", range_parts[0]))?;
            let end: u32 = range_parts[1].parse()
                .map_err(|_| anyhow::anyhow!("Invalid page number: {}", range_parts[1]))?;
            
            if start > end {
                bail!("Invalid range: start page {} is greater than end page {}", start, end);
            }
            
            for page in start..=end {
                pages.push(page);
            }
        } else {
            let page: u32 = part.parse()
                .map_err(|_| anyhow::anyhow!("Invalid page number: {}", part))?;
            pages.push(page);
        }
    }
    
    pages.sort();
    pages.dedup();
    Ok(pages)
}
