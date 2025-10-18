# PDF Cutter - Command Line Interface (CLI) Documentation

PDF Cutter is a powerful command-line tool for manipulating PDF files. It allows you to merge multiple PDFs, delete specific pages, split PDFs into smaller parts, or launch an interactive Terminal User Interface (TUI).

## Installation

```bash
# Clone the repository
git clone https://github.com/tourlat/pdfcutter.git
cd pdfcutter

# Build the project
cargo build --release

# The executable will be available in target/release/pdf-cutter
```

For more installation options, see the [Installation Guide](install.md).

## General Usage

```bash
pdf-cutter <COMMAND> [OPTIONS]
```

### Available Commands

- `merge` - Merge multiple PDFs into one
- `delete` - Delete pages from a PDF
- `split` - Split a PDF into multiple smaller PDFs
- `tui` - Launch Terminal User Interface

---

## Command: `merge`

Merge multiple PDF files into a single output file.

### Syntax

```bash
pdf-cutter merge -o <OUTPUT> <INPUT1> <INPUT2> [INPUT3...]
```

### Arguments

- `-o, --output <OUTPUT>` - Name of the output PDF file
- `<INPUTS>...` - List of input PDF files to merge (minimum 2 files required)

### Examples

```bash
# Merge two PDFs
pdf-cutter merge -o merged_document.pdf file1.pdf file2.pdf

# Merge multiple PDFs
pdf-cutter merge -o complete_book.pdf chapter1.pdf chapter2.pdf chapter3.pdf appendix.pdf

# Using full paths
pdf-cutter merge -o /home/user/merged.pdf /path/to/doc1.pdf /path/to/doc2.pdf
```

### Notes

- Files are merged in the order specified on the command line
- At least 2 input files are required
- Output file will be created in the current directory unless a full path is specified
- Input files must be valid PDF documents

---

## Command: `delete`

Delete specific pages from a PDF document.

### Syntax

```bash
pdf-cutter delete -i <INPUT> -o <OUTPUT> -p <PAGES>
```

### Arguments

- `-i, --input <INPUT>` - Input PDF file
- `-o, --output <OUTPUT>` - Output PDF file
- `-p, --pages <PAGES>` - Pages to delete (see formats below)

### Page Format Options

- Single page: `3`
- Page range: `3-5` (deletes pages 3, 4, and 5)
- Multiple selections: `1,3,5-7` (deletes pages 1, 3, 5, 6, and 7)
- Mixed format: `2,5-8,10,12-15`

### Examples

```bash
# Delete page 3
pdf-cutter delete -i document.pdf -o document_modified.pdf -p "3"

# Delete pages 1 to 5
pdf-cutter delete -i report.pdf -o report_without_intro.pdf -p "1-5"

# Delete multiple specific pages
pdf-cutter delete -i book.pdf -o book_edited.pdf -p "1,3,7-9,15"

# Delete last pages (if document has 20 pages)
pdf-cutter delete -i document.pdf -o document_clean.pdf -p "18-20"
```

### Notes

- Page numbers start from 1
- Invalid page numbers are ignored
- Page ranges are inclusive (1-3 includes pages 1, 2, and 3)
- The original file is not modified

---

## Command: `split`

Split a PDF into multiple smaller documents based on page ranges.

### Syntax

```bash
pdf-cutter split -i <INPUT> -p <PAGES> -o <OUTPUT_PREFIX> [--named]
```

### Arguments

- `-i, --input <INPUT>` - Input PDF file to split
- `-p, --pages <PAGES>` - Page ranges for splitting (see formats below)
- `-o, --output-prefix <PREFIX>` - Prefix for output files
- `--named` - Use named segments format (optional)

### Page Format Options

#### Standard Format (without --named)
Pages are specified as ranges separated by commas:
- `1-3,5,7-9` - Creates 3 files: pages 1-3, page 5, and pages 7-9

#### Named Format (with --named)
Each segment can have a custom name:
- `intro:1-3,chapter1:4-10,conclusion:11` - Creates files with custom names

### Examples

#### Standard Split
```bash
# Split into 3 parts
pdf-cutter split -i book.pdf -p "1-5,6-10,11-15" -o "part_"
# Creates: part_1.pdf, part_2.pdf, part_3.pdf

# Split individual pages
pdf-cutter split -i document.pdf -p "1,3,5" -o "page_"
# Creates: page_1.pdf, page_2.pdf, page_3.pdf

# Complex split
pdf-cutter split -i manual.pdf -p "1-2,5-8,10,12-20" -o "section_"
# Creates: section_1.pdf, section_2.pdf, section_3.pdf, section_4.pdf
```

#### Named Split
```bash
# Split with custom names
pdf-cutter split -i book.pdf -p "preface:1-2,chapter1:3-15,chapter2:16-30" -o "book_" --named
# Creates: book_preface.pdf, book_chapter1.pdf, book_chapter2.pdf

# Academic paper split
pdf-cutter split -i paper.pdf -p "abstract:1,introduction:2-4,methods:5-8,results:9-12,conclusion:13-14" -o "paper_" --named
# Creates: paper_abstract.pdf, paper_introduction.pdf, paper_methods.pdf, paper_results.pdf, paper_conclusion.pdf
```

### Notes

- Page numbers start from 1
- In standard mode, files are numbered sequentially (1, 2, 3...)
- In named mode, the segment name is used in the filename
- Invalid page ranges are skipped
- Overlapping ranges are allowed

---

## Command: `tui`

Launch the interactive Terminal User Interface for PDF manipulation.

### Syntax

```bash
pdf-cutter tui
```

### Features

The TUI provides an interactive interface with:
- File selection and management
- Visual preview of operations
- Step-by-step configuration
- Real-time validation
- Error handling and feedback

See [TUI Documentation](usage-tui.md) for detailed instructions.

---

## Error Handling

### Common Errors

1. **File not found**
   ```
   Error: Could not open file 'nonexistent.pdf'
   ```

2. **Invalid PDF**
   ```
   Error: 'corrupted.pdf' is not a valid PDF file
   ```

3. **Permission denied**
   ```
   Error: Permission denied writing to 'output.pdf'
   ```

4. **Invalid page range**
   ```
   Error: Invalid page range '25-30' (document has only 20 pages)
   ```

## Advanced Examples

### Complex Page Management

```bash
# Remove cover and last page, then split into chapters
pdf-cutter delete -i book.pdf -o book_clean.pdf -p "1,100"
pdf-cutter split -i book_clean.pdf -p "1-10,11-25,26-40" -o "chapter_"

# Create presentation handouts (pages 1,3,5... only)
pdf-cutter split -i presentation.pdf -p "1,3,5,7,9,11,13,15" -o "handout_page_"
```

## Troubleshooting

### Performance Issues

- **Large files**: Processing very large PDFs may take time
- **Memory usage**: Complex operations may require significant RAM
- **Disk space**: Ensure sufficient space for output files

### Compatibility

- **Encrypted PDFs**: Password-protected files are not currently supported

### Getting Help

```bash
# View help for main command
pdf-cutter --help

# View help for specific subcommand
pdf-cutter merge --help
pdf-cutter delete --help
pdf-cutter split --help
```

## See Also
- [Terminal User Interface Usage](usage-tui.md)
- [Installation Guide](install.md)