# PDF Cutter

A command-line tool for manipulating PDF files, including merging and deleting pages. Built with Rust for performance and safety.

![TUI Merge Example](tests/pdfcutter-gif.gif)

## Features

- **Merge PDFs**: Combine multiple PDF files into one
- **Delete Pages**: Remove specific pages from a PDF file
- **Interactive TUI**: User-friendly terminal interface
- **Command Line**: Scriptable CLI interface
- **Privacy-First**: Process files locally, no cloud upload required

## Quick Start

### Using Precompiled Binaries

Download the latest release from the [Releases](https://github.com/tourlat/pdfcutter/releases) page:

- **Linux**: `pdf-cutter-linux-x86_64`
- **Windows**: `pdf-cutter-windows-x86_64.exe`

```bash
# Linux/macOS
./pdf-cutter-linux-x86_64 tui

# Windows
pdf-cutter-windows-x86_64.exe tui
```

### Building from Source

**Prerequisites**: [Rust and Cargo](https://rustup.rs/)

```bash
git clone https://github.com/tourlat/pdfcutter.git
cd pdfcutter
cargo build --release
```

## Usage

### Interactive TUI Mode

Launch the terminal user interface for guided operations:

```bash
cargo run -- tui
```

**Controls:**
- ↑/↓ arrows: Navigate menus and file lists
- Alt+↑/↓: Reorder files in merge mode
- Enter: Select options or add files
- Tab: Allow writing in input field (for specifying output path in delete mode)
- P: Allow writing in input field (for specifying pages to delete in delete mode)
- Esc: Go back or exit

### Command Line Interface

#### Merge PDFs
```bash
cargo run -- merge -o <output_path> <input_path1> <input_path2> ...
```

#### Delete Pages
```bash
cargo run -- delete -i <input_path> -o <output_path> -p "<pages>"
```

**Page formats:**
- Single page: `"1"`
- Page range: `"3-5"`
- Multiple pages/ranges: `"1,3,5-7"`

## Examples

### Merge Operations
```bash
# Merge two PDFs
cargo run -- merge -o merged.pdf document1.pdf document2.pdf

# Merge multiple PDFs
cargo run -- merge -o combined.pdf file1.pdf file2.pdf file3.pdf
```

### Delete Operations
```bash
# Remove first page
cargo run -- delete -i document.pdf -o output.pdf -p "1"

# Remove pages 3 to 5
cargo run -- delete -i document.pdf -o output.pdf -p "3-5"

# Remove pages 1, 3, and 5-7
cargo run -- delete -i document.pdf -o output.pdf -p "1,3,5-7"
```

## Installation Methods

### Option 1: Precompiled Binaries
Download from [Releases](https://github.com/tourlat/pdfcutter/releases)

⚠️ **Note**: Windows binaries are cross-compiled on Linux. For best compatibility on Windows, consider using WSL or building from source.

### Option 2: Build from Source
```bash
git clone https://github.com/tourlat/pdfcutter.git
cd pdfcutter
cargo build --release
```

The binary will be available at `target/release/pdf-cutter`

### Option 3: Install via Cargo
```bash
cargo install --git https://github.com/tourlat/pdfcutter.git
```

## Built With

- **[lopdf](https://crates.io/crates/lopdf)**: Low-level PDF manipulation
- **[clap](https://crates.io/crates/clap)**: Command-line argument parsing
- **[ratatui](https://crates.io/crates/ratatui)**: Terminal user interface

## Motivation

This project was created to provide a simple and efficient way to manipulate PDF files from the command line. I wanted to stop using online tools for simple tasks like merging or deleting pages from PDFs. With this tool, users can easily manage their PDF documents without relying on third-party services and keep their files private.

##Development Status

**Status**: In active development

**Planned features:**
- PDF splitting functionality

## 📝 License

This project is open source. See the repository for license details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.