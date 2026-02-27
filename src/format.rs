use crate::lex;
use std::io;

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

pub fn format_string(s: &str) -> Result<String, FormatError> {
    Ok(s.to_string())
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
