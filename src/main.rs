use std::path::PathBuf;

use boki::utils::indent_string;
use clap::{Parser, Subcommand};

enum CLIError {
    InputError(boki::input::InputError),
    OtherError(String),
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CLIError::InputError(e) => {
                write!(f, "Input Error:\n{}", indent_string(&e.to_string()))?
            }
            CLIError::OtherError(e) => write!(f, "Other Error:\n{}", indent_string(e))?,
        };

        Ok(())
    }
}

type CLIResult<T> = Result<T, CLIError>;

impl From<boki::input::InputError> for CLIError {
    fn from(value: boki::input::InputError) -> Self {
        Self::InputError(value)
    }
}

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

    let journal = boki::input::compile_file(cli.file.to_str().unwrap())?;

    match &cli.command {
        Commands::Export { output } => {
            let output_str =
                serde_json::to_string(&journal).map_err(|e| CLIError::OtherError(e.to_string()))?;
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
