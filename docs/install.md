# Pdf Cutter Installation Guide


PDF Cutter can be installed in multiple ways depending on your setup and preference.  
Choose one of the methods below.

### Option 1: Precompiled Binaries
Download the latest release from the [Releases](https://github.com/tourlat/pdfcutter/releases) page.

⚠️ **Note**: Windows binaries are cross-compiled on Linux. For best compatibility on Windows, consider using WSL or building from source.

### Option 2: Build from Source

**Prerequisite:** [Rust and Cargo](https://rustup.rs)

```bash
git clone https://github.com/tourlat/pdfcutter.git
cd pdfcutter
cargo build --release
```

The binary will be available at `target/release/pdf-cutter`

### Option 3: Install via Cargo

From github repository :

**Prerequisite:** [Rust and Cargo](https://rustup.rs)

```bash
cargo install --git https://github.com/tourlat/pdfcutter.git
```

From a local directory :
Clone the repository and navigate to its directory, then run:
```bash
git clone https://github.com/tourlat/pdfcutter.git
cd pdfcutter
cargo install --path .
```

## Verify Installation

After installation, verify that `pdf-cutter` is accessible from your terminal:

```bash
# From precompiled binaries
pdf-cutter --help
```
or 
```bash
# With cargo installation
cargo run --help
```

You should see the help message with available commands and options.


## See Also
- [Command Line Interface Usage](usage-cli.md)
- [Terminal User Interface Usage](usage-tui.md)
