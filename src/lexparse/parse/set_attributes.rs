use crate::lexparse::contracts::tokens::Keyword;
use crate::lexparse::parse::{basic, core};

use crate::lexparse::parse::core::TokenScanner;

pub struct SetAttributeParser;

impl SetAttributeParser {
    pub fn new() -> Self {
        SetAttributeParser
    }

    pub fn parse(&self, scanner: &mut TokenScanner) -> core::ParserResult<(String, String)> {
        basic::parse_keyword(scanner, Keyword::Set)?;
        let name = basic::parse_identifier(scanner)?;
        let value = basic::parse_identifier(scanner)?;
        basic::parse_line_separator(scanner)?;

        Ok((name, value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexparse::contracts::tokens::{Keyword, Token};

    #[test]
    fn test_simple() {
        let mut scanner = TokenScanner::from_slice(&[
            Token::Keyword(Keyword::Set),
            Token::Identifier("default_commodity".to_string()),
            Token::Identifier("JPY".to_string()),
            Token::LineSeparator,
        ]);
        let result = SetAttributeParser::new()
            .parse(&mut scanner)
            .expect("Failed.");
        assert_eq!(result.0, "default_commodity");
        assert_eq!(result.1, "JPY");
    }
}
