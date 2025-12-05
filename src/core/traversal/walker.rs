use crate::commands::run::args::RunArgs;
use std::path::Path;
use walkdir::WalkDir;

pub fn process_dir(run_args: &RunArgs) -> anyhow::Result<()> {
    validate_path_exists(&run_args.input_path)?;
    log_starting_path(&run_args.input_path);
    traverse_directory(
        &run_args.input_path,
        &run_args.exclude,
        run_args.skip_hidden,
        run_args.verbose,
    )?;
    println!("✅ Extraction complete");
    Ok(())
}

fn validate_path_exists(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }
    Ok(())
}

fn log_starting_path(path: &Path) {
    if path == Path::new(".") {
        if let Ok(cwd) = std::env::current_dir() {
            println!("Traversing directory: {}", cwd.display());
        }
    } else {
        println!("Traversing directory: {}", path.display());
    }
}

fn traverse_directory(
    root: &Path,
    exclude_patterns: &[String],
    skip_hidden: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let walker = WalkDir::new(root).into_iter().filter_entry(|entry| {
        let non_excluded_path = !should_exclude(entry.path(), exclude_patterns);
        let non_hidden_path = !skip_hidden || !is_hidden(entry);
        non_excluded_path && non_hidden_path
    });

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if verbose {
                println!("📄 {}", path.display());
            }
            // TODO: Process file content
        } else if path.is_dir() {
            if verbose {
                println!("📁 {}", path.display());
            }
        }
    }

    Ok(())
}

fn should_exclude(path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let path_str = path.to_string_lossy().to_lowercase();
    patterns
        .iter()
        .any(|pattern| path_str.contains(&pattern.to_lowercase()))
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|str| {
            let hidden_entry = str.starts_with(".");
            if hidden_entry {
                println!("Hidden entry '{}' was skipped", entry.path().display());
            }
            hidden_entry
        })
        .unwrap_or(false)
}
