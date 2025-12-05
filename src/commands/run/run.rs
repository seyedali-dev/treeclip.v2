use super::args::RunArgs;
use crate::core::traversal;

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    if args.verbose {
        log_startup(&args);
    }

    // Run core logic
    traversal::walker::process_dir(
        &args.input_path,
        &args.exclude,
        &args.output_path,
        args.verbose,
    )?;

    if args.clipboard {
        // Will implement later
        println!("(Would copy to clipboard)");
    }

    if args.editor {
        println!("(Would open in editor)");
    }

    if args.delete && args.editor {
        println!("(Would delete after editor closes)");
    }

    Ok(())
}

fn log_startup(args: &RunArgs) {
    println!("🚀 Starting TreeClip...");
    println!("📁 Input Path: {}", args.input_path.display());
    println!("📁 Output Path: {}", args.output_path.display());
    println!("📋 Clipboard: {}", args.clipboard);
    println!("📊 Stats: {}", args.stats);
    println!("✏️  Editor: {}", args.editor);
    println!("🗑️  Delete: {}", args.delete);
    if !args.exclude.is_empty() {
        println!("🚫 Exclude patterns: {:?}", args.exclude);
    }
}
