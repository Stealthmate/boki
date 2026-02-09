use crate::input::contracts::tokens::{Keyword, Token};
use crate::input::parse_v2::{basic, combinators, core};

use crate::input::parse_v2::core::{Parser, ParserError, TokenScanner};

pub struct SetAttributeParser;

impl SetAttributeParser {
    pub fn new() -> Self {
        SetAttributeParser
    }

    pub fn parse(&self, scanner: &mut TokenScanner) -> core::ParserResult<(String, String)> {
        let w = basic::parse_keyword(scanner, Keyword::Set)?;
        let name = basic::parse_identifier(scanner)?;
        let value = basic::parse_identifier(scanner)?;
        basic::parse_line_separator(scanner)?;

        Ok((name, value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::contracts::tokens::{Keyword, Token};

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
