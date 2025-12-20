//! clipboard - Handles system clipboard operations for file content.

use crate::core::errors::{ClipboardError, FileSystemError};
use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

/// Maximum clipboard content size (100MB) to prevent memory issues.
const MAX_CLIPBOARD_SIZE: usize = 100 * 1024 * 1024;

/// Clipboard provides an interface to interact with the system clipboard.
pub struct Clipboard {
    /// Path to the data file to be copied to clipboard.
    data: PathBuf,
    /// Handle to the system clipboard.
    clip: arboard::Clipboard,
}

impl Clipboard {
    /// Creates a new Clipboard instance for the specified file path.
    ///
    /// # Errors
    ///
    /// Returns `ClipboardError::InitializationFailed` if the clipboard cannot be initialized.
    pub fn new(data: &Path) -> Result<Self, ClipboardError> {
        let clip = arboard::Clipboard::new().map_err(|e| {
            ClipboardError::InitializationFailed(format!(
                "Failed to access system clipboard: {}",
                e
            ))
        })?;

        Ok(Self {
            data: data.to_path_buf(),
            clip,
        })
    }

    /// Reads the output file and places its contents into the system clipboard.
    ///
    /// # Platform Notes
    ///
    /// - **Windows/macOS**: Clipboard contents persist after program exit.
    /// - **Linux**: Persistence depends on running clipboard service
    ///   (e.g., GNOME/KDE clipboard, CopyQ, wl-clipboard).
    ///
    /// This follows standard CLI behavior: sets clipboard and exits immediately.
    /// On most desktop environments this works out of the box. On minimal window
    /// managers without a clipboard manager, contents may not persist after exit.
    ///
    /// # Errors
    ///
    /// Returns `ClipboardError` if:
    /// - File cannot be read
    /// - File is too large (>100MB)
    /// - Clipboard cannot be accessed
    pub fn set_clipboard(&mut self) -> anyhow::Result<()> {
        // Check file size first
        let metadata = std::fs::metadata(&self.data)
            .with_context(|| format!("Failed to read file metadata: {}", self.data.display()))?;

        let file_size = metadata.len() as usize;
        if file_size > MAX_CLIPBOARD_SIZE {
            return Err(ClipboardError::ContentTooLarge {
                size: file_size,
                max: MAX_CLIPBOARD_SIZE,
            }
            .into());
        }

        // TODO: Optimize for huge files - consider streaming or chunking instead of loading entire file
        // Read entire file into memory (clipboard APIs require full content as string)
        let mut output_file = File::options()
            .read(true)
            .open(&self.data)
            .map_err(|e| FileSystemError::ReadFailed {
                path: self.data.clone(),
                source: e,
            })
            .with_context(|| {
                format!(
                    "Failed to open file for clipboard operation: {}",
                    self.data.display()
                )
            })?;

        let mut output_content = String::new();
        output_file
            .read_to_string(&mut output_content)
            .map_err(|e| FileSystemError::ReadFailed {
                path: self.data.clone(),
                source: e,
            })
            .with_context(|| {
                format!(
                    "Failed to read file contents for clipboard: {}",
                    self.data.display()
                )
            })?;

        // Set clipboard text
        // On Linux, clipboard managers usually take ownership immediately
        self.clip
            .set()
            .text(output_content)
            .map_err(|e| ClipboardError::SetFailed(format!("Clipboard operation failed: {}", e)))
            .with_context(|| "Failed to set clipboard content - clipboard may not be available")?;

        // NOTE: Sleep guarantees clipboard ownership (required by arboard on some platforms)
        thread::sleep(Duration::from_millis(100));

        Ok(())
    }
}

#[cfg(test)]
mod clipboard_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_clipboard_creation() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content")?;

        let clipboard = Clipboard::new(&file_path);
        assert!(clipboard.is_ok());

        Ok(())
    }

    #[test]
    fn test_clipboard_with_nonexistent_file() {
        let result = Clipboard::new(Path::new("/nonexistent/file.txt"));
        // Should still create clipboard instance (file is read later)
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_clipboard_with_content() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, clipboard!")?;

        let mut clipboard = Clipboard::new(&file_path)?;
        let result = clipboard.set_clipboard();

        // May fail in CI environments without clipboard support
        // So we just check it doesn't panic and provides context
        match result {
            Ok(_) => {} // Success in environments with clipboard
            Err(e) => {
                // Should have context message
                let error_msg = format!("{:?}", e);
                assert!(
                    error_msg.contains("clipboard") || error_msg.contains("Failed to"),
                    "Error should have context: {}",
                    error_msg
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_set_clipboard_with_empty_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("empty.txt");
        fs::write(&file_path, "")?;

        let mut clipboard = Clipboard::new(&file_path)?;
        let result = clipboard.set_clipboard();

        // May fail in CI without clipboard support
        let _ = result;

        Ok(())
    }

    #[test]
    fn test_clipboard_size_limit() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("huge.txt");

        // Create a file larger than MAX_CLIPBOARD_SIZE
        let large_content = "x".repeat(MAX_CLIPBOARD_SIZE + 1);
        fs::write(&file_path, large_content)?;

        let mut clipboard = Clipboard::new(&file_path)?;
        let result = clipboard.set_clipboard();

        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("too large"));

        Ok(())
    }

    #[test]
    fn test_clipboard_nonexistent_file_error() {
        let file_path = PathBuf::from("/nonexistent/file.txt");
        let mut clipboard = Clipboard::new(&file_path).unwrap();
        let result = clipboard.set_clipboard();

        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("Failed to"));
    }
}
