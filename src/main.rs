use crate::commands::run::*;
use clap::Parser;
use cli::*;

mod cli;
mod commands;
mod core;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run(run_args) => run::execute(run_args)?,
    }

    Ok(())
}
