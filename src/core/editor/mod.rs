//! editor - Provides functionality to open and delete files using system editors.
//!
//! Source Inspired - https://stackoverflow.com/a/56012454
//! Posted by Peter Varo, modified by community. See post 'Timeline' for change history.
//! Retrieved 2025-12-16, License - CC BY-SA 4.0

use crate::core::errors::{EditorError, FileSystemError};
use anyhow::Context;
use std::path::Path;
use std::{env, fs, process};

/// Opens the file in the system's default text editor.
///
/// Falls back to nano if the default editor is not found.
///
/// # Platform-specific behavior
///
/// - **Windows**: Uses `start` command
/// - **macOS**: Uses `open` command
/// - **Unix/Linux**: Uses `xdg-open` command
///
/// If the graphical editor fails, attempts to use the CLI editor specified
/// in the `EDITOR` environment variable, or `/bin/nano` as final fallback.
///
/// # Errors
///
/// Returns `EditorError` if neither the default editor nor the fallback editor can be executed.
pub fn open(path: &Path) -> anyhow::Result<()> {
    let command = get_platform_open_command();

    if command.is_empty() {
        return Err(EditorError::NoEditorFound(
            "No platform-specific command available".to_string(),
        )
        .into());
    }

    let canonical_path = path
        .canonicalize()
        .map_err(|e| FileSystemError::CanonicalizeFailed {
            path: path.to_path_buf(),
            source: e,
        })
        .with_context(|| format!("Failed to resolve absolute path for: {}", path.display()))?;

    match process::Command::new(command).arg(&canonical_path).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            eprintln!(
                "Default editor exited with status: {}. Attempting CLI editor...",
                status
            );
            open_with_cli_editor(path)
                .with_context(|| format!("All editor attempts failed for file: {}", path.display()))
        }
        Err(e) => {
            eprintln!(
                "Error opening file with default editor: {}. Attempting CLI editor...",
                e
            );
            open_with_cli_editor(path)
                .with_context(|| format!("All editor attempts failed for file: {}", path.display()))
        }
    }
}

/// Deletes the specified file from the filesystem.
///
/// # Note
///
/// There is no guarantee that the file is immediately deleted. Depending on
/// platform and open file descriptors, removal may be delayed.
///
/// # Errors
///
/// Returns `FileSystemError::DeleteFailed` if the file cannot be deleted.
pub fn delete(path: &Path) -> anyhow::Result<()> {
    fs::remove_file(path)
        .map_err(|e| FileSystemError::DeleteFailed {
            path: path.to_path_buf(),
            source: e,
        })
        .with_context(|| format!("Failed to delete file: {}", path.display()))?;

    Ok(())
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Returns the platform-specific command for opening files.
fn get_platform_open_command() -> &'static str {
    if cfg!(windows) {
        "start"
    } else if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(unix) {
        "xdg-open"
    } else {
        ""
    }
}

/// Opens the file using a CLI text editor.
fn open_with_cli_editor(path: &Path) -> anyhow::Result<()> {
    let default_cli_editor = env::var("EDITOR").unwrap_or_else(|e| {
        eprintln!(
            "Error reading EDITOR environment variable: {}. Falling back to nano.",
            e
        );
        "/bin/nano".to_string()
    });

    let status = process::Command::new(&default_cli_editor)
        .arg(path)
        .status()
        .map_err(|e| EditorError::OpenFailed {
            path: path.to_path_buf(),
            source: e,
        })
        .with_context(|| {
            format!(
                "Failed to launch editor '{}' for file: {}",
                default_cli_editor,
                path.display()
            )
        })?;

    if !status.success() {
        return Err(EditorError::ProcessFailed { status }.into());
    }

    Ok(())
}

#[cfg(test)]
mod editor_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_platform_open_command() {
        let command = get_platform_open_command();

        if cfg!(windows) {
            assert_eq!(command, "start");
        } else if cfg!(target_os = "macos") {
            assert_eq!(command, "open");
        } else if cfg!(unix) {
            assert_eq!(command, "xdg-open");
        } else {
            assert_eq!(command, "");
        }
    }

    #[test]
    fn test_delete_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content")?;

        assert!(file_path.exists());

        delete(&file_path)?;

        assert!(!file_path.exists());

        Ok(())
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let result = delete(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());

        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("Failed to delete"));
    }

    #[test]
    fn test_open_with_nonexistent_file() {
        let result = open(Path::new("/nonexistent/file.txt"));
        // This will fail because canonicalize fails on non-existent paths
        assert!(result.is_err());

        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("Failed to resolve") || error_msg.contains("canonicalize"));
    }

    #[test]
    fn test_delete_with_permission_error() {
        // This test is platform-specific and may not work in all environments
        // Just ensure error handling provides context
        let result = delete(Path::new("/root/protected_file.txt"));

        if let Err(e) = result {
            let error_msg = format!("{:?}", e);
            assert!(error_msg.contains("Failed to delete") || error_msg.contains("permission"));
        }
    }

    #[test]
    fn test_delete_provides_context() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");

        // Try to delete non-existent file
        let result = delete(&file_path);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_chain = format!("{:?}", error);

        // Should contain both the path and context
        assert!(error_chain.contains(&file_path.display().to_string()));
        assert!(error_chain.contains("Failed to delete"));

        Ok(())
    }
}
