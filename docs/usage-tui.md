# Usage - Terminal User Interface (TUI) Mode

## Interactive TUI Mode
Launch the terminal user interface for guided operations:

- If you are using Cargo:
```bash
cargo run -- tui
```

- Or if you have installed the binary or built from source:
```bash
pdf-cutter tui
```

**Controls:**
- In main menu : 
    - ↑/↓ arrows: Navigate menus 
    - Enter: Select option
    - Esc/q: Exit application
    - Press 1, 2, 3 or 4 to quickly access Merge, Delete, Split or Help modes

- In file list :
    - ↑/↓ arrows: Navigate files
    - Alt+↑/↓: Reorder files in merge mode
    - Tab: Allow writing in input field (for specifying output path in delete mode)
    - Backspace: Remove selected file
    - ENTER: Go to next step (e.g., configure options, confirm operation)

- In delete mode :
    - Tab: Allow writing in input field (for specifying output path in delete mode)
    - P: Allow writing in input field (for specifying pages to delete in delete mode)
    - Enter: Confirm and execute deletion

- In split mode :
    - P: Allow writing in input field (for specifying output path in split mode)
    - S: Allow writing in input field (for specifying pages to split in split mode)
    - Space: Toggle named segments on/off
    - Enter: Confirm and execute splitting
    - Esc: Go back or exit

- In merge mode :
    - Alt+↑/↓: Reorder files
    - Tab: Allow writing in input field (for specifying output path in merge mode)
    - Enter: Confirm and execute merging
    - Esc: Go back or exit

---
## Examples
### Merge PDFs
1. Launch TUI:
    ```bash
    cargo run -- tui
    ```
2. Select "Merge PDFs" from the main menu.
3. Add PDF files to merge.
4. Specify output file path.
5. Confirm to merge.

---
### Delete Pages
1. Launch TUI:
    ```bash
    cargo run -- tui
    ```
2. Select "Delete Pages" from the main menu.
3. Add the PDF file.
4. Specify pages to delete (e.g., `1,3,5-7`).
5. Specify output file path.
6. Confirm to delete pages.

---
### Split Pages
1. Launch TUI:
    ```bash
    cargo run -- tui
    ```
2. Select "Split Pages" from the main menu.
3. Add the PDF file.
4. Specify segments or pages to split (e.g., `1-3,4-6` or `intro:1-3, outro:4-6`).
5. Specify output file path.
6. Confirm to split pages.
---

## See Also
- [Command Line Interface Usage](usage-cli.md)
- [Installation Guide](install.md)