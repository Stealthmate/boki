use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug)]
struct CLIError(String);

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl From<Box<boki::evaluate::EvaluateError>> for CLIError {
    fn from(value: Box<boki::evaluate::EvaluateError>) -> Self {
        Self(format!("{}", *value))
    }
}

type CLIResult<T> = Result<T, CLIError>;

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

fn _main() -> CLIResult<()> {
    let cli = Cli::parse();

    let journal = boki::evaluate::evaluate_file(cli.file.to_str().unwrap())?;

    match &cli.command {
        Commands::Export { output } => {
            let output_str =
                serde_json::to_string(&journal).map_err(|e| CLIError(e.to_string()))?;
            match output {
                None => println!("{output_str}"),
                Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
            };
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        eprintln!("{}", e);
    }
}
