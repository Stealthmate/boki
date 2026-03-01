use crate::parsing::TokenScanner;
use crate::tokens;
use crate::{lex, parsing};
use std::io;

mod _ast;
mod parse;
mod write;

#[derive(Clone, Debug)]
pub enum FormatError {
    General(String),
}

impl From<lex::LexerError> for FormatError {
    fn from(value: lex::LexerError) -> Self {
        Self::General(format!("{:#?}", value))
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
        let input_str =
            std::fs::read_to_string("src/format/input.boki").expect("Could not read input file.");
        let formatted_str = super::format_string(&input_str).expect("Failed.");
        let rhs =
            std::fs::read_to_string("src/format/output.boki").expect("Could not read output file.");

        assert_eq!(formatted_str, rhs);
    }
}
