use std::{io::Write, path::PathBuf};

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

impl From<boki::format::FormatError> for CLIError {
    fn from(value: boki::format::FormatError) -> Self {
        Self(format!("{:#?}", value))
    }
}

type CLIResult<T> = Result<T, CLIError>;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Export {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Format {
        files: Vec<PathBuf>,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn _main() -> CLIResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Export { file, output } => {
            let journal = boki::evaluate::evaluate_file(file.to_str().unwrap())?;
            let output_str =
                serde_json::to_string(&journal).map_err(|e| CLIError(e.to_string()))?;
            match output {
                None => println!("{output_str}"),
                Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
            };
        }
        Commands::Format { files } => {
            for file in files {
                let input =
                    std::fs::read_to_string(file.clone()).map_err(|e| CLIError(e.to_string()))?;
                let output = boki::format::format_string(&input)?;
                let mut f = std::fs::File::options()
                    .write(true)
                    .truncate(true)
                    .open(file)
                    .map_err(|e| CLIError(e.to_string()))?;
                f.write(output.as_bytes())
                    .map_err(|e| CLIError(e.to_string()))?;
            }
        }
    };

    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        eprintln!("{}", e);
    }
}
