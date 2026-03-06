use std::path::PathBuf;

mod error;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(clap::Args)]
pub struct Args {
    file: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub fn run(args: &Args) -> Result<()> {
    let journal = boki::evaluate::evaluate_file(args.file.to_str().unwrap()).expect("TODO");
    let output_str = serde_json::to_string(&journal).expect("TODO");
    match &args.output {
        None => println!("{output_str}"),
        Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
    };

    Ok(())
}
