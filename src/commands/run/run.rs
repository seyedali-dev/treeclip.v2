use super::args::RunArgs;
use crate::core::ui::{animations, banner, formatter, messages};
use crate::core::{clipboard::clipboard, editor::editor, traversal::walker};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    banner::print_welcome();

    let input = if &args.input_path == Path::new(".") {
        env::current_dir()?
    } else {
        args.input_path.clone()
    };

    let output = match &args.output_path {
        Some(path) if path == Path::new(".") => PathBuf::from("./treeclip_temp.txt"),
        Some(path) => path.clone(),
        None => PathBuf::from("./treeclip_temp.txt"),
    };

    let root = match &args.root {
        Some(path) if path == Path::new(".") => env::current_dir()?,
        Some(path) => path.to_path_buf(),
        None => env::current_dir()?,
    };

    log_config(&args, &root, &input, &output)?;

    println!("\n{}", messages::Messages::starting_adventure());
    animations::animated_dots(&messages::Messages::scanning_files(), 3, 300);

    // Run core logic
    let walker = walker::Walker::new(&root, &input, &output, &args.exclude);

    let spinner = animations::Spinner::new_tree();
    spinner.spin(&messages::Messages::traversing_tree(), 1200);
    walker.process_dir(&args)?;

    println!("\n{}", messages::Messages::gathering_leaves());

    let mut clip = clipboard::Clipboard::new(&output)?;

    if args.clipboard {
        let spinner = animations::Spinner::new_loading();
        spinner.spin(&messages::Messages::copying_clipboard(), 800);
        clip.set_clipboard()?;
        println!("{}", messages::Messages::clipboard_ready());
    } else {
        println!("{}", messages::Messages::clipboard_skipped());
    }

    if args.stats {
        println!("\n{}", messages::Messages::showing_stats());
        show_stats(&output)?;
    }

    if args.editor {
        println!("\n{}", messages::Messages::opening_editor());
        editor::open(&output)?;
        println!("{}", messages::Messages::editor_opened());
    }

    if args.delete && args.editor {
        println!("\n{}", messages::Messages::cleaning_up());
        editor::delete(&output)?;
        println!("{}", messages::Messages::cleaned_up());
    }

    banner::print_goodbye();
    Ok(())
}

fn show_stats(output: &PathBuf) -> anyhow::Result<()> {
    use colored::Colorize;

    let content = fs::read_to_string(output)?;
    let lines = content.split("\n").count();
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    let stats = formatter::StatsBox::new(lines, chars, words, bytes);
    println!("{}", stats.render().bright_cyan());

    let (emoji, message) = stats.get_size_message();
    println!("  {} {}", emoji, message);

    Ok(())
}

#[rustfmt::skip]
fn log_config(args: &RunArgs, root: &PathBuf, input: &PathBuf, output: &PathBuf) -> anyhow::Result<()> {
    println!("{}", formatter::ConfigFormatter::format_section_header("Configuration Settings", "🔧"));

    let config_items = vec![
        ("🌍", "Root Path", formatter::ConfigFormatter::format_path(root)),
        ("📂", "Input Path", formatter::ConfigFormatter::format_path(input)),
        ("💾", "Output Path", formatter::ConfigFormatter::format_path(output)),
        ("✏️", "Editor", formatter::ConfigFormatter::format_bool(args.editor)),
        ("🗑️", "Cleanup", formatter::ConfigFormatter::format_bool(args.delete)),
        ("📋", "Clipboard", formatter::ConfigFormatter::format_bool(args.clipboard)),
        ("📊", "Stats", formatter::ConfigFormatter::format_bool(args.stats)),
        ("👻", "Skip Hidden", formatter::ConfigFormatter::format_bool(args.skip_hidden)),
    ];

    for (icon, label, value) in config_items {
        println!("{}", formatter::ConfigFormatter::format_config_line(icon, label, value));
    }

    if !args.exclude.is_empty() {
        println!("{}", formatter::ConfigFormatter::format_section_header("Excluded Patterns", "🚫"));
        for pattern in &args.exclude {
            println!("{}", formatter::ConfigFormatter::format_list_item("▸", pattern));
        }
    }

    println!("{}", messages::Messages::ready_to_launch());
    Ok(())
}
