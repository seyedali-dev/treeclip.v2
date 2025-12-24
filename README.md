# TreeClip 🌳✨

*A delightfully fast CLI tool that bundles your code into a single, AI-friendly format*

---

## What's This All About? (◕‿◕✿)

Ever tried explaining your entire codebase to an AI assistant, only to spend 20 minutes copy-pasting files? Yeah, me
too. That's why TreeClip exists!

TreeClip traverses your project directory, gathers all your code files, and bundles them into one neat package with
proper headers showing where each piece came from. It's like creating a "highlight reel" of your project that AI models
can actually digest in one go.

**Think of it as:** *Your project, but as a single, well-organized document that preserves all the context.*

---

## Why I Built This 🛠️

Honestly? I was learning Rust and needed a practical project. But also, I was tired of the tedious workflow of sharing
code with AI assistants:

1. Copy file contents
2. Paste into chat
3. Explain the file structure
4. Repeat 47 times
5. Lose track of which files you've shared
6. Question your life choices

TreeClip turns that into: `treeclip run --clipboard` → done. One command, everything's ready to paste.

Plus, I wanted to practice Rust's error handling, understand the module system, work with the filesystem APIs, and build
something actually useful. Learning by building real tools just hits different, you know?

---

## Installation

### From Source

```bash
git clone https://github.com/seyedali-dev/treeclip.v2.git
cd treeclip.v2
cargo build --release
```

The binary will be in `target/release/treeclip`

### Using Cargo

```bash
cargo install --path .
```

This installs it globally so you can run `treeclip` from anywhere!

---

## Quick Start 🚀

The most common use case (what I use 90% of the time):

```bash
# Bundle current directory and copy to clipboard
treeclip run --clipboard
```

Now just paste into your AI chat. That's it. You're welcome. (づ｡◕‿‿◕｡)づ

---

## Usage Guide

### Basic Structure

```bash
treeclip run [INPUT_PATH] [OPTIONS]
```

- **INPUT_PATH**: The directory to traverse (defaults to current directory `.`)
- **OPTIONS**: Various flags to customize behavior

### Understanding Defaults

When you run `treeclip run` with no arguments, here's what happens:

- **Input:** Current directory (`.`)
- **Output:** `./treeclip_temp.txt` in the current directory
- **Clipboard:** Does NOT copy automatically (you need `--clipboard`)
- **Hidden files:** Skipped (uses `--skip-hidden` by default)
- **Stats:** Not shown
- **Editor:** Not opened
- **Fast mode:** Off (shows cute animations ♡)

---

## Common Usage Patterns

### 1. Quick Clipboard Copy (My Daily Driver)

```bash
treeclip run --clipboard
```

**What happens:**

- Scans current directory
- Creates `treeclip_temp.txt`
- Copies everything to clipboard
- Shows progress with cute tree emojis 🌳

**When to use:** When you just need to paste your code into ChatGPT/Claude/etc.

---

### 2. Specific Directory with Output File

```bash
treeclip run ./src -o ./docs/code-dump.txt
```

**What happens:**

- Scans the `./src` directory
- Saves output to `./docs/code-dump.txt`
- Doesn't touch clipboard

**When to use:** Documenting specific parts of your project, creating code archives.

---

### 3. Exclude Noise

```bash
treeclip run --exclude node_modules --exclude target --exclude .git
```

**What happens:**

- Scans current directory
- Ignores `node_modules/`, `target/`, and `.git/`
- You can stack multiple `--exclude` flags!

**When to use:** Working with projects that have large build artifacts or dependencies.

**Pro tip:** Create a `.treeclipignore` file (works like `.gitignore`) for permanent exclusions:

```
# .treeclipignore
node_modules
target
.git
dist
build
*.log
```

---

### 4. Review Before Sharing

```bash
treeclip run --editor --delete
```

**What happens:**

- Creates `treeclip_temp.txt`
- Opens it in your default editor (respects `$EDITOR`)
- After you close the editor, deletes the temp file automatically

**When to use:** When you want to review/edit the output before sharing it with AI.

---

### 5. The Full Experience™

```bash
treeclip run ./my-project \
  --output-path ./export/project-snapshot.txt \
  --exclude node_modules \
  --exclude "*.lock" \
  --clipboard \
  --stats \
  --verbose
```

**What happens:**

- Scans `./my-project`
- Saves to `./export/project-snapshot.txt`
- Excludes node_modules and lockfiles
- Copies to clipboard
- Shows detailed statistics (lines, words, bytes)
- Prints verbose progress info

**When to use:** When you want EVERYTHING and want to see exactly what's happening.

---

### 6. Fast Mode for Large Projects ⚡

```bash
treeclip run --fast-mode --clipboard
```

**What happens:**

- Skips all animations and progress indicators
- Instant execution
- Perfect for scripts or large codebases

**When to use:** CI/CD pipelines, automation scripts, or when you're in a hurry.

---

### 7. Including Hidden Files

By default, TreeClip skips hidden files (those starting with `.`). To include them:

```bash
treeclip run --no-skip-hidden
```

**When to use:** When you need config files like `.env.example`, `.editorconfig`, etc.

---

## All Command Options

### Positional Arguments

| Argument     | Description           | Default                 |
|--------------|-----------------------|-------------------------|
| `INPUT_PATH` | Directory to traverse | `.` (current directory) |

### Optional Arguments

| Flag                   | Short | Description                          | Default               |
|------------------------|-------|--------------------------------------|-----------------------|
| `--output-path <PATH>` | `-o`  | Where to save the output file        | `./treeclip_temp.txt` |
| `--root <PATH>`        |       | Root directory for `.treeclipignore` | `.`                   |
| `--exclude <PATTERN>`  | `-e`  | Patterns to exclude (can repeat)     | None                  |
| `--clipboard`          | `-c`  | Copy output to clipboard             | Off                   |
| `--stats`              |       | Show content statistics              | Off                   |
| `--editor`             |       | Open output in default editor        | Off                   |
| `--delete`             |       | Delete output after closing editor   | Off                   |
| `--verbose`            | `-v`  | Show detailed progress               | Off                   |
| `--skip-hidden`        | `-H`  | Skip hidden files/folders            | **On**                |
| `--no-skip-hidden`     |       | Include hidden files                 | Off                   |
| `--raw`                | `-r`  | Extract raw content                  | **On**                |
| `--fast-mode`          | `-f`  | Skip animations, instant execution   | Off                   |
| `--help`               | `-h`  | Show help message                    | -                     |
| `--version`            | `-V`  | Show version                         | -                     |

---

## Output Format

TreeClip creates a clean, AI-friendly format:

```
==> src/main.rs
fn main() {
    println!("Hello, world!");
}

==> src/lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

==> tests/integration_test.rs
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

Each file is clearly separated with its path, making it easy for AI to understand your project structure. No confusion,
no missing context!

---

## Tips & Tricks 💡

### Combine with Other Tools

```bash
# Count total lines in your project
treeclip run --fast-mode && wc -l treeclip_temp.txt

# Compare two versions of your code
treeclip run ./v1 -o v1.txt --fast-mode
treeclip run ./v2 -o v2.txt --fast-mode
diff v1.txt v2.txt
```

### Create Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
# Quick clipboard copy
alias clip='treeclip run --clipboard --fast-mode'

# Review before sharing
alias clipr='treeclip run --clipboard --editor --delete'

# Full verbose snapshot
alias clips='treeclip run --clipboard --stats --verbose'
```

### Project-Specific Configurations

Keep a `.treeclipignore` in your project root:

```
# Dependencies
node_modules
target
vendor

# Build outputs
dist
build
out
*.min.js

# Logs and temp files
*.log
*.tmp
.DS_Store

# Don't share secrets!
.env
secrets.json
```

---

## Examples from Real Projects

### React Project

```bash
treeclip run ./src \
  --exclude node_modules \
  --exclude "*.test.js" \
  --clipboard
```

### Rust Project

```bash
treeclip run \
  --exclude target \
  --exclude Cargo.lock \
  --clipboard \
  --stats
```

### Python Project

```bash
treeclip run \
  --exclude __pycache__ \
  --exclude "*.pyc" \
  --exclude venv \
  --clipboard
```

---

## Future Plans 🚧

- [ ] Configuration file support (`.treecliprc`)
- [ ] Interactive mode for selecting files
- [ ] Multiple output format support (JSON, Markdown, HTML)
- [ ] Token counting for AI models
- [ ] Smart exclusion patterns (auto-detect `.gitignore`)
- [ ] Streaming for huge projects
- [ ] Plugin system for custom processors

But honestly? I built this to learn Rust and solve my immediate problem. If you find it useful, awesome! If you want to
contribute, even better! ♡

---

## Contributing

Found a bug? Have an idea? Want to make it cuter?

1. Fork the repo
2. Make your changes
3. Submit a PR with a description

I'm still learning Rust, so if you spot any anti-patterns or improvements, I'm all ears! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧

---

## License

[MIT License](./LICENSE) - feel free to use this however you want!

---

## Credits

Built with:

- Rust 🦀
- Intention of becoming a rustacean
- A genuine desire to stop copy-pasting code files

---

**Happy clipping!** ✨

*Made with ♡ (and a lot of Stack Overflow) by someone who just wanted a better way to share code with AI*
