use clap::Parser;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(clap::Parser)]
#[command(name = "treeclip")]
#[command(version = "v0.1.0")]
#[command(
    about = "Traverse directories and files and extract it's contents",
    long_about = "Traverse directories and files and extract contents into a temporary folder and/or clipboard."
)]
#[command(next_line_help = true)]
struct Cli {
    /// Path to traverse (defaults to current directory)
    #[arg(default_value_t = String::from("."))]
    input_path: String,

    /// Output path for extracted file (defaults to current directory)
    #[arg(default_value_t = String::from("."))]
    output_path: String,

    /// Exclude files/folders matching these patterns
    #[arg(short, long)]
    exclude: Vec<String>,

    /// Copy output to clipboard
    #[arg(long, default_value_t = true)]
    clipboard: bool,

    /// Show clipboard content statistics
    #[arg(long, default_value_t = false)]
    stats: bool,

    /// Open output file in the default text editor
    #[arg(long, default_value_t = false)]
    editor: bool,

    /// Delete the output file after editor is closed
    #[arg(long, default_value_t = false)]
    delete: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("🚀 Starting TreeClip...");
        println!("📁 Input Path: {}", cli.output_path);
        println!("📋 Clipboard: {}", cli.clipboard);
        println!("📊 Stats: {}", cli.stats);
        println!("✏️  Editor: {}", cli.editor);
        println!("🗑️  Delete: {}", cli.delete);
        if !cli.exclude.is_empty() {
            println!("🚫 Exclude patterns: {:?}", cli.exclude);
        }
    }

    // Your core logic would run here
    run_treeclip(
        &cli.output_path,
        &cli.exclude,
        &cli.output_path,
        cli.verbose,
    )?;

    // TODO: Implement clipboard and editor logic based on the boolean flags
    if cli.clipboard {
        println!("(Would copy to clipboard)");
    }

    if cli.editor {
        println!("(Would open in editor)");
    }

    Ok(())
}

fn run_treeclip(
    path: &str,
    exclude_paths: &[String],
    output_path: &str,
    verbose: bool,
) -> anyhow::Result<()> {
    let path_buf = PathBuf::from(path);

    // Check if path exists
    if !path_buf.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }

    if path.eq(".") {
        let cwd = env::current_dir()?;
        println!("Traversing directory: {}", cwd.display());
    } else {
        println!("Traversing directory: {}", path_buf.display());
    }

    // Skip entries we can't access
    for entry in WalkDir::new(&path_buf).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            println!("📄 {}", path.display());
        } else if path.is_dir() {
            println!("📁 {}", path.display());
        }
    }

    Ok(())
}
