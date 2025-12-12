# treeclip.v2

A CLI tool to traverse your project files and extract them into a single file or clipboard, designed specifically to facilitate sharing code with AI assistants.

## What is treeclip.v2?

Treeclip.v2 is a command-line utility that recursively traverses directories and consolidates all text-based file contents into a single output file. Each file's content is preceded by a header indicating its path, making it easy to share entire project structures with AI models for code review, debugging, or other purposes.

## Why was it created?

This tool was created to solve the common problem of sharing multi-file projects with AI assistants. Instead of copying and pasting individual files, developers can use treeclip to generate a complete, structured dump of their project that preserves file hierarchy and context - perfect for providing comprehensive context to AI models.

## Installation

### From Source
```bash
git clone https://github.com/seyedali-dev/treeclip.v2.git
cd treeclip.v2
cargo build --release
```

### Using Cargo
```bash
cargo install --path .
```

## Usage

Basic usage:
```bash
treeclip run [OPTIONS] [INPUT_PATH]
```

### Examples

Extract the current directory:
```bash
treeclip run
```

Extract a specific directory to a custom output file:
```bash
treeclip run ./my-project \          #this is input 
             ./extracted-content.txt # this is output
```

Extract while excluding certain directories:
```bash
treeclip run ./my-project --exclude node_modules --exclude .git --exclude target
```

Extract and copy to clipboard automatically:
```bash
treeclip run ./my-project --clipboard
```

Open the output file in your default editor:
```bash
treeclip run ./my-project --editor
```

## Command Options

### Positional Arguments
- `INPUT_PATH`: Path to traverse (defaults to current directory)

### Options
- `--output-path <OUTPUT_PATH>`: Output path for extracted file (defaults to current directory, with filename `./treeclip_temp.txt`)
- `-e, --exclude <PATTERNS>`: Exclude files/folders matching these patterns (can be used multiple times)
- `--clipboard`: Copy output to clipboard (enabled by default)
- `--no-clipboard`: Disable copying output to clipboard
- `--stats`: Show clipboard content statistics
- `--editor`: Open output file in the default text editor
- `--delete`: Delete the output file after editor is closed
- `-v, --verbose`: Verbose output
- `-H, --skip-hidden`: Skip hidden files and folders in Unix systems (enabled by default)
- `-r, --raw`: Extract raw content without additional metadata (enabled by default)
- `-h, --help`: Display help information

## Output Format

The tool generates a file with the following format:
```
==> path/to/file1.ext
Contents of the first file here...

==> path/to/file2.ext
Contents of the second file here...

==> another/path/file3.ext
Contents of the third file here...
```

This format clearly separates different files and indicates their paths, preserving the project structure while combining everything into a single document suitable for AI consumption.
