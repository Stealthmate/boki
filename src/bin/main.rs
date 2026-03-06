use clap::{Parser, Subcommand};

mod cli;
mod error;

use error::CLIErrorResult;

#[derive(Subcommand)]
enum Commands {
    Export(cli::export::Args),
    Format(cli::format::Args),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Export(args) => cli::export::run(args).or_quit(),
        Commands::Format(args) => cli::format::run(args).or_quit(),
    };
}
