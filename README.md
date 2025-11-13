# rtw-head

> [!NOTE]
> **RTW Series (Reinvent The Wheel)**
> This project is part of a series called RTW - Reinvent The Wheel. The main goal of this series is not to create alternatives to existing tools, but to deeply understand how these tools work by reimplementing them. Each project suffixed with `rtw-` is a technical and educational exploration.

## Description

`rtw-head` is a simplified reimplementation of the Unix `head` tool in Rust. It displays the first N lines of one or more files using efficient buffered streaming.

## What is head?

`head` is a command-line utility used to display the beginning of text files. By default, it shows the first 10 lines, but this can be customized. It's commonly used to quickly preview files or extract headers from large datasets.

### Classic Workflow

1. **File Opening**: The program opens the target file(s) and obtains file descriptors
2. **Buffered Reading**: Content is read line by line using a buffer
3. **Line Counting**: Stop after reading N lines (default: 10)
4. **Multi-file Support**: Automatically display headers when processing multiple files
5. **Memory Efficiency**: Only processes needed lines, rest of file is never loaded

## Technical Highlights

This implementation demonstrates important systems programming concepts:

- **File descriptors**: Using `File::open()` to get a handle without loading data
- **Buffered I/O**: `BufReader` for efficient line-based reading
- **Iterator adapters**: `.lines().take()` for lazy line processing
- **Error handling**: Proper `Result` types with `Box<dyn Error>` for flexibility
- **Path handling**: Safe UTF-8 validation with fallback strategies
- **Output locking**: `StdoutLock` for efficient multi-write operations

## Features

- ✅ Display first N lines of files (default: 10)
- ✅ Negative line counts (display all but last N lines)
- ✅ Support for multiple files with automatic headers
- ✅ Verbose mode to force headers even for single files
- ✅ Quiet mode to suppress all headers
- ✅ Robust error handling for invalid paths and encodings

## Usage

### Basic usage
```bash
cargo run -- <file_path>
```

Example:
```bash
# Display first 10 lines
cargo run -- example.txt

# Display first 5 lines
cargo run -- -n 5 example.txt

# Display all but the last 3 lines
cargo run -- -n -3 example.txt

# Process multiple files (headers shown automatically)
cargo run -- file1.txt file2.txt file3.txt

# Force header display even for single file
cargo run -- -v example.txt

# Suppress headers even for multiple files
cargo run -- -q file1.txt file2.txt
```

### With cargo build
```bash
cargo build --release
./target/release/rtw-head example.txt
./target/release/rtw-head -n 20 file1.txt file2.txt
```

### Command-line options

- `-n, --lines <NUM>`: Print the first NUM lines (default: 10). With a leading '-', print all but the last NUM lines
- `-v, --verbose`: Always print headers with file names
- `-q, --quiet`: Never print headers with file names
- `-h, --help`: Display help information
- `-V, --version`: Display version information

## Code Structure

- `src/main.rs`: Application entry point and error handling
- `src/lib.rs`: Core streaming logic with buffered I/O and multi-file support
- `src/cli.rs`: CLI configuration and argument parsing using clap

## Learning Resources

This project explores:
- How Unix line-oriented tools process text efficiently
- The power of Rust iterators and lazy evaluation
- Why buffered I/O matters for line-based processing
- Handling multiple files with proper separation
- Type-safe command-line parsing with clap
- Error propagation patterns in Rust (`?` operator)

## Differences from GNU head

This implementation focuses on core functionality. Missing features:
- `-c/--bytes`: Display first N bytes instead of lines
- stdin support (reading from pipe)
