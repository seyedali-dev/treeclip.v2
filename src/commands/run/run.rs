use super::args::RunArgs;
use crate::core::constants;
use crate::core::{clipboard::clipboard, editor::editor, traversal::walker, utils};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
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

    log_info(&args, &root, &input, &output)?;

    // Run core logic
    let walker = walker::Walker::new(&root, &input, &output, &args.exclude);
    walker.process_dir(&args)?;

    let mut clip = clipboard::Clipboard::new(&output)?;
    if args.clipboard {
        clip.set_clipboard()?;
    }

    if args.stats {
        let content = fs::read_to_string(&output)?; // fixme: bad. very bad. cpu and ram intensive!
        let lines = content.split("\n").count();
        let chars = content.chars().count();
        let words = content.split_whitespace().count();
        let bytes = content.len();

        #[rustfmt::skip] println!("{} {:<width$}", "📊", " Clipboard content stats:".bold().white(), width = constants::RIGHT_PADDING);
        #[rustfmt::skip] println!("  {} {:<12} {:>10}", "📝", " Characters".italic(), utils::format_number(chars as i64).dimmed());
        #[rustfmt::skip] println!("  {} {:<12} {:>10}", "📄", " Lines".italic(), utils::format_number(lines as i64).dimmed());
        #[rustfmt::skip] println!("  {} {:<12} {:>10}", "💬", " Words".italic(), utils::format_number(words as i64).dimmed());
        #[rustfmt::skip] println!("  {} {:<12} {:>10}", "💾", " Size".italic(), utils::format_bytes(bytes).dimmed());
    }

    if args.editor {
        editor::open(&output)?;
    }

    if args.delete && args.editor {
        editor::delete(&output)?;
    }

    Ok(())
}

#[rustfmt::skip]
fn log_info(args: &RunArgs,root: &PathBuf,input: &PathBuf,output: &PathBuf) -> anyhow::Result<()>{
    fn colorize_bool(val: bool) -> String {
        if val {
            "true".green().to_string()
        } else {
            "false".red().to_string()
        }
    }

    let header = format!("{} {}", "🚀", " Starting TreeClip...".bold().bright_magenta());
    println!("{}", header);
    println!("-----------------------");
    println!("{} {:<width$} {}", "📁", " Root Path".bold(), root.canonicalize()?.display().to_string().cyan(), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "📁", " Input Path".bold(), input.canonicalize()?.display().to_string().cyan(), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "📁", " Output Path".bold(), output.canonicalize()?.display().to_string().cyan(), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "✏️", " Editor".bold(), colorize_bool(args.editor), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "🗑️", " Delete".bold(), colorize_bool(args.delete), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "📋", " Clipboard".bold(), colorize_bool(args.clipboard), width = constants::RIGHT_PADDING);
    println!("{} {:<width$} {}", "📊", " Stats".bold(), colorize_bool(args.stats), width = constants::RIGHT_PADDING);

    if !args.exclude.is_empty() {
        let exclude_display = format!(
            "[{}]",
            args.exclude
                .iter()
                .map(|s| s.dimmed().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("{} {:<width$} {}", "🚫".red(), " Exclude patterns".bold(), exclude_display, width = constants::RIGHT_PADDING);
    }

    Ok(())
}
