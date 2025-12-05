use std::path::PathBuf;

#[derive(clap::Args)]
pub struct RunArgs {
    /// Path to traverse (defaults to current directory)
    #[arg(default_value = ".", value_parser = validate_path)]
    pub input_path: PathBuf,

    /// Output path for extracted file
    #[arg(default_value = ".", value_parser = validate_path)]
    pub output_path: PathBuf,

    /// Exclude files/folders matching these patterns
    #[arg(short, long)]
    pub exclude: Vec<String>,

    /// Copy output to clipboard
    #[arg(long, default_value_t = true)]
    pub clipboard: bool,

    /// Show clipboard content statistics
    #[arg(long, default_value_t = false)]
    pub stats: bool,

    /// Open output file in the default text editor
    #[arg(long, default_value_t = false)]
    pub editor: bool,

    /// Delete the output file after editor is closed
    #[arg(long, default_value_t = false)]
    pub delete: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    // Basic validation - path doesn't need to exist yet
    if s.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    Ok(path)
}
