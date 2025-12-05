use std::path::Path;
use walkdir::WalkDir;

pub fn process_dir(
    input_path: &Path,
    exclude_patterns: &[String],
    _output_path: &Path,
    verbose: bool,
) -> anyhow::Result<()> {
    validate_path_exists(input_path)?;

    log_starting_path(input_path);
    traverse_directory(input_path, exclude_patterns, verbose)?;

    // TODO: Implement actual file extraction
    if verbose {
        println!("✅ Extraction complete");
    }

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
    verbose: bool,
) -> anyhow::Result<()> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !should_exclude(e.path(), exclude_patterns));

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
