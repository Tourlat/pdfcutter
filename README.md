### PDF Cutter

A command-line tool for manipulating PDF files, including merging, deleting pages. Built with Rust for performance and safety.

#### Features

- **Merge PDFs**: Combine multiple PDF files into one.
- **Delete Pages**: Remove specific pages from a PDF file.

#### Usage
- **Merge PDFs**:
  ```
  cargo run -- merge -o <output_path> <input_path> <input_path> ...
  ```
- **Delete Pages**:
    - Delete page 1:
        ```
        cargo run -- delete -i <pdf_path> -o <output_path> -p "1"
        ```
    - Delete pages 3 to 5:
        ```
        cargo run -- delete -i <pdf_path> -o <output_path> -p "3-5"
        ```
    - Delete pages 1, 3, and 5 to 7:
        ```
        cargo run -- delete -i <pdf_path> -o <output_path> -p "1,3,5-7"

        ```
#### Examples

- Remove the first page of `a.pdf`:
  ```
  cargo run -- delete -i tests/tests_pdf/a.pdf -o test_sans_page1.pdf -p "1"
  ```
- Remove pages 2 and 3:
  ```
  cargo run -- delete -i tests/tests_pdf/a.pdf -o test_sans_pages2-3.pdf -p "2-3"
  ```


### How to Build and Run

1. Clone the repository:
   ```
   git clone https://github.com/tourlat/pdfcutter.git
   ```
2. Change into the project directory:
   ```
   cd pdf-manipulator
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. Run the project:
   ```
   cargo run -- <command>
   ```  Replace `<command>` with the desired operation (e.g., `merge`, `delete`, `help`).

#### Motivation

This project was created to provide a simple and efficient way to manipulate PDF files from the command line, I wanted to stop using online tools for simple tasks like merging or deleting pages from PDFs. With this tool, users can easily manage their PDF documents without relying on third-party services and can keep their files private.

#### Built With

- [lopdf](https://crates.io/crates/lopdf): Low-level PDF editing (merging, splitting, watermarking).

- [clap](https://crates.io/crates/clap): Command-line argument parsing.


Still in development. 

Future plans include adding more PDF manipulation features and adding a CLI terminal user interface (TUI) made with RATATUI.