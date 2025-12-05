use crate::commands::run::{args};
use clap::Parser;

#[derive(Parser)]
#[command(name = "treeclip")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Traverse directories and extract contents")]
#[command(
    long_about = "Traverse directories and files, extracting contents into a temporary folder and/or clipboard."
)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Run treeclip on a directory
    Run(args::RunArgs),
}
