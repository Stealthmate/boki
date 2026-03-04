use std::io::Write;
use std::path::PathBuf;

mod _ast;
mod parse;
mod write;

pub struct Error(String);

impl crate::error::CLIError for Error {
    fn format(&self) -> String {
        self.0.clone()
    }
}

impl From<FormatError> for Error {
    fn from(value: FormatError) -> Self {
        Self(format!("{}", value))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self(format!("{}", value))
    }
}

#[derive(clap::Args)]
pub struct Args {
    files: Vec<PathBuf>,
}

pub fn run(args: &Args) -> Result<(), Error> {
    for file in &args.files {
        let input = std::fs::read_to_string(file.clone())?;
        let output = format_string(&input)?;
        let mut f = std::fs::File::options()
            .write(true)
            .truncate(true)
            .open(file)?;
        f.write_all(output.as_bytes())?;
    }

    Ok(())
}

use boki::parsing::TokenScanner;
use boki::tokens;
use boki::{lex, parsing};
use std::io;

#[derive(Clone, Debug)]
pub enum FormatError {
    General(String),
}

impl From<lex::LexerError> for FormatError {
    fn from(value: lex::LexerError) -> Self {
        Self::General(format!("{}", value))
    }
}

impl From<io::Error> for FormatError {
    fn from(value: io::Error) -> Self {
        Self::General(value.to_string())
    }
}

impl From<parsing::ParserError> for FormatError {
    fn from(value: parsing::ParserError) -> Self {
        Self::General(format!("{:#?}", value))
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::General(x) => {
                write!(f, "{x}")?;
            }
        };

        Ok(())
    }
}

pub fn format_string(s: &str) -> Result<String, FormatError> {
    let decoated_tokens = lex::lex_string(s)?;
    let tokens: Vec<tokens::Token> = decoated_tokens
        .iter()
        .map(|dt| dt.token().clone())
        .collect();
    let nodes = parse::parse(&mut TokenScanner::from_slice(tokens.as_slice()))?;

    let output = format!("{}", write::to_displayable(nodes.as_slice()));
    Ok(output)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_smoke() {
        let input_str = std::fs::read_to_string("src/bin/cli/format/input.boki")
            .expect("Could not read input file.");
        let formatted_str = super::format_string(&input_str).expect("Failed.");
        let rhs = std::fs::read_to_string("src/bin/cli/format/output.boki")
            .expect("Could not read output file.");

        assert_eq!(formatted_str, rhs);
    }
}
