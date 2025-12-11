use std::path::Path;

pub fn should_exclude(path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let path_str = path.to_string_lossy().to_lowercase();
    patterns
        .iter()
        .any(|pattern| path_str.contains(&pattern.to_lowercase()))
}

pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
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

#[cfg(test)]
mod filter_tests {
    use crate::core::traversal::filter::{is_hidden, should_exclude};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_should_exclude() {
        let path = Path::new("/home/user/project/node_modules/package");
        let patterns = vec!["node_modules".to_string(), ".git".to_string()];

        assert!(should_exclude(path, &patterns));

        let path2 = Path::new("/home/user/project/src/main.rs");
        assert!(!should_exclude(path2, &patterns));
    }

    #[test]
    fn test_should_exclude_case_insensitive() {
        let path = Path::new("/home/user/project/NODE_MODULES/package");
        let patterns = vec!["node_modules".to_string()];

        assert!(should_exclude(path, &patterns));
    }

    #[test]
    fn test_is_hidden() {
        // Create a mock entry
        let temp_dir = TempDir::new().unwrap();
        let hidden_file = temp_dir.path().join(".hidden");
        fs::write(&hidden_file, "").unwrap();

        let entry = walkdir::WalkDir::new(temp_dir.path())
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        assert!(is_hidden(&entry));
    }
}
