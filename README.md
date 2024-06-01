# Directory Structure CLI

This is a command-line interface (CLI) application written in Rust that provides various functionality for analyzing and extracting information from a directory structure.

## Prerequisites

- Rust programming language (version 1.x.x)
- Cargo package manager

## Installation

1. Clone the repository:
```
git clone <repository_url>
```

2. Navigate to the project directory:
```
cd repository
```

3. Build the project:
```
cargo build --release
```

## Usage

The CLI app provides the following commands:

### 1. Generate Directory Structure
To generate a textual representation of the directory structure, use the `dir` command:
```
cargo run --release -- dir [--path <directory_path>]
```
- `--path`: Optional flag to specify the directory path. If not provided, the current working directory will be used.

The generated directory structure will be displayed in the console and saved to a file named `directory_structure.txt` in the specified directory.

### 2. Extract Definitions

To extract definitions (classes, methods, functions, etc.) from files with a specific extension, use the `extract` command:
```
cargo run --release -- extract [--path <directory_path>] [--ext <file_extension>]
```
- `--path`: Optional flag to specify the directory path. If not provided, the current working directory will be used.
- `--ext`: Optional flag to specify the file extension. If not provided, it defaults to "js".

The extracted definitions will be saved to a file named `extracted.txt` in the specified directory.

### 3. Extract Raw Content

To extract the raw content of files with specific extensions, use the `raw` command:
```
cargo run --release -- raw [--path <directory_path>] --ext <file_extensions>
```
- `--path`: Optional flag to specify the directory path. If not provided, the current working directory will be used.
- `--ext`: Required flag to specify the file extensions (comma-separated) to include.

The extracted raw content will be saved to a file named `raw.txt` in the specified directory.

### 4. Process Files and Store in SQLite Database

To process files in a directory and store their information in an SQLite database, use the `db` command:
```
cargo run --release -- db [--path <directory_path>]
```
- `--path`: Optional flag to specify the directory path. If not provided, the current working directory will be used.

The processed file information will be stored in an SQLite database file named `context.db` in the specified directory.

## Examples

- Generate directory structure for the current directory:
```
cargo run --release -- dir
```
- Extract definitions from JavaScript files in the "src" directory:
```
cargo run --release -- extract --path src --ext js
```
- Extract raw content of Rust files in the current directory:
```
cargo run --release -- raw --ext rs
```
- Process files in the "project" directory and store in the database:
```
cargo run --release -- db --path project
```
## License

This project is licensed under the [MIT License](LICENSE).


