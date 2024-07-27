# Frust

Frust is a command-line tool designed to extract frontmatter from Markdown files efficiently. It supports processing individual files or entire directories, including recursive directory traversal.

## Features

- Extracts YAML frontmatter from Markdown files
- Processes individual files or directories
- Supports recursive directory processing for convenient bulk operations
- Offers flexible output options: console or specified file

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the Frust repository:
   ```sh
   git clone https://github.com/yourusername/frust.git
   ```
3. Build the project using Cargo:
   ```sh
   cd frust
   cargo build --release
   ```

## Usage

```sh
frust [OPTIONS] <input>
```

### Arguments

- `<input>`: Specify the input file or directory path.

### Options

- `-o, --output <output>`: Provide the output file path. If not specified, the result will be printed to the console.
- `-R, --recursive`: Enable recursive processing of directories.
- `-v, --verbose`: Enable verbose output for detailed logging.

## Examples

### Extract frontmatter from a single Markdown file

```sh
frust input.md
```

### Extract frontmatter from a directory recursively and write to a file

```sh
frust -R -o output.json input_directory/
```

### Verbose output while processing a directory

```sh
frust -R -v input_directory/
```

## License

This project is licensed under the [MIT License](https://choosealicense.com/licenses/mit/).
