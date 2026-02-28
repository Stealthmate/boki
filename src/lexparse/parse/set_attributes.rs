use crate::tokens::Keyword;

use crate::parsing::{self, TokenScanner};

pub struct SetAttributeParser;

impl SetAttributeParser {
    pub fn new() -> Self {
        SetAttributeParser
    }

    pub fn parse(&self, scanner: &mut TokenScanner) -> parsing::ParserResult<(String, String)> {
        parsing::parse_keyword(scanner, Keyword::Set)?;
        let name = parsing::parse_identifier(scanner)?;
        let value = parsing::parse_identifier(scanner)?;
        parsing::parse_line_separator(scanner)?;

        Ok((name, value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::{Keyword, Token};

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
