use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod cli;
mod error;

use error::CLIErrorResult;

// #[derive(Debug)]
// struct CLIError(String);

// impl std::fmt::Display for CLIError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "{}", self.0)
//     }
// }

// impl From<Box<boki::evaluate::EvaluateError>> for CLIError {
//     fn from(value: Box<boki::evaluate::EvaluateError>) -> Self {
//         Self(format!("{}", *value))
//     }
// }

// impl From<boki::format::FormatError> for CLIError {
//     fn from(value: boki::format::FormatError) -> Self {
//         Self(format!("{}", value))
//     }
// }

// type CLIResult<T> = Result<T, CLIError>;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Export {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
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
        Commands::Export { file, output } => {
            let journal = boki::evaluate::evaluate_file(file.to_str().unwrap()).expect("TODO");
            let output_str = serde_json::to_string(&journal).expect("TODO");
            match output {
                None => println!("{output_str}"),
                Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
            };
        }
        Commands::Format(args) => cli::format::run(args).or_quit(),
    };
}
