use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Export {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    let source = std::fs::read_to_string(cli.file).expect("Failed to read journal.");
    let journal = boki::input::compile_string(&source).expect("Failed to compile journal.");

    match &cli.command {
        Commands::Export { output } => {
            let output_str = serde_json::to_string(&journal).expect("Failed to serialize journal.");
            match output {
                None => println!("{output_str}"),
                Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
            };
        }
    }
}
