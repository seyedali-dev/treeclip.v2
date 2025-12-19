use colored::Colorize;
use std::io::{stdout, Write};
use std::{thread, time};

pub struct Spinner {
    frames: Vec<&'static str>,
    colors: Vec<colored::Color>,
}

impl Spinner {
    pub fn new_tree() -> Self {
        Self {
            frames: vec!["🌱", "🌿", "🍃", "🌳", "🌲", "🎄"],
            colors: vec![
                colored::Color::Green,
                colored::Color::BrightGreen,
                colored::Color::Cyan,
                colored::Color::BrightCyan,
            ],
        }
    }

    pub fn new_loading() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            colors: vec![
                colored::Color::Cyan,
                colored::Color::BrightCyan,
                colored::Color::Blue,
                colored::Color::BrightBlue,
            ],
        }
    }

    pub fn spin(&self, message: &str, duration_ms: u64) {
        let frame_duration = duration_ms / self.frames.len() as u64;
        for (i, frame) in self.frames.iter().enumerate() {
            let color = &self.colors[i % self.colors.len()];
            print!(
                "\r{} {} {}",
                frame.color(*color),
                message.bright_cyan(),
                "...".dimmed()
            );
            stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(frame_duration));
        }
        println!(
            "\r{} {} {}",
            "✓".bright_green(),
            message.bright_green(),
            "Done!".dimmed()
        );
    }
}

pub fn animated_dots(text: &str, count: usize, delay_ms: u64) {
    print!("{}", text.bright_yellow());
    for _ in 0..count {
        print!("{}", ".".bright_yellow());
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(delay_ms));
    }
    println!();
}

pub fn progress_counter(emoji_set: &[&str], current: usize, interval: usize) -> Option<String> {
    if current % interval == 0 {
        let idx = (current / interval) % emoji_set.len();
        Some(format!(
            "{} Collected {} files so far...",
            emoji_set[idx], current
        ))
    } else {
        None
    }
}
