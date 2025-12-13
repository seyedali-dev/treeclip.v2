use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use arboard::SetExtLinux;

pub struct Clipboard {
    /// data to put onto clipboard
    data: PathBuf,
}

impl Clipboard {
    pub fn new(data: &PathBuf) -> Self {
        Self { data: data.clone() }
    }
}

impl Clipboard {
    pub fn set_clipboard(self) -> anyhow::Result<()> {
        let mut output_file = File::options().read(true).open(&self.data)?;
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content)?; //TODO: bad reading the full content into memory? what if it's huge?

        let mut clipboard = arboard::Clipboard::new()?;
        clipboard
            .set()
            .wait()
            .text(output_content)
            .with_context(|| "failed to set output content in clipboard")?;

        Ok(())
    }
}
