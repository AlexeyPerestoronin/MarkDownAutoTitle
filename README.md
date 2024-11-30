# Markdown Title Auto-Add Tool

This is a command-line tool designed to automatically add titles to Markdown files. It allows users to specify a title message and customize the indentation level for the titles. The tool can also skip the first existing title in the document if needed.

## Features

- **Add Titles**: Automatically add a specified title to a Markdown file.
- **Customizable Indentation**: Set the number of spaces for indentation of titles.
- **Skip First Title**: Optionally skip the first existing title in the Markdown file.
- **Output to File**: Save the modified Markdown file to a specified result file or overwrite the original.

### General examples:
1. Add title to the file:
    ```bash
    auto_title.exe --file "target_file.md" --title-message "Title message:"
    ```
0. Regenerate title to the file:
    ```bash
    auto_title.exe --file "target_file.md" --title-message "Title message:" --skip-first-title
    ```

## Installation

To use this tool, you need to have Rust installed on your system. If Rust is not installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project using Cargo:

```bash
git clone <repository-url>
cd <repository-directory>
cargo build --release
```
