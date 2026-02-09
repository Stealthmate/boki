use crate::input::contracts::tokens::{Keyword, Token};
use crate::input::parse::core;

pub struct SetAttributeParser;

impl SetAttributeParser {
    pub fn new() -> Self {
        SetAttributeParser
    }

    pub fn parse<'a>(&self, tokens: &'a [Token]) -> core::ParserResult<'a, (String, String)> {
        let (tokens, w) = core::parse_keyword(tokens, Keyword::Set)?;
        let (tokens, name) = core::parse_identifier(tokens)?;
        let (tokens, value) = core::parse_identifier(tokens)?;
        let (tokens, _) = core::parse_line_separator(tokens)?;

        Ok((tokens, (name, value)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::contracts::tokens::{Keyword, Token};

    #[test]
    fn test_simple() {
        let tokens = [
            Token::Keyword(Keyword::Set),
            Token::Identifier("default_commodity".to_string()),
            Token::Identifier("JPY".to_string()),
            Token::LineSeparator,
        ];
        let (rest, result) = SetAttributeParser::new().parse(&tokens).expect("Failed.");
        assert_eq!(result.0, "default_commodity");
        assert_eq!(result.1, "JPY");
    }
}
