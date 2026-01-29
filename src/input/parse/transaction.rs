use crate::input::compile::ast;
use crate::input::parse::core;
use crate::input::parse::core::Parser;
use crate::input::parse::core::Token;

pub struct TransactionParser;

impl TransactionParser {
    fn parse_header(tokens: &[Token]) -> core::ParserResult<'_, ast::TransactionHeader> {
        let (tokens, timestamp) = core::parse_timestamp(tokens)?;
        let (tokens, _) = core::parse_line_separator(tokens)?;
        Ok((tokens, ast::TransactionHeader { timestamp }))
    }

    fn parse_posting(tokens: &[Token]) -> core::ParserResult<'_, ast::Posting> {
        todo!()
    }

    pub fn parse(tokens: &[Token]) -> core::ParserResult<'_, ast::Transaction> {
        let (tokens, header) = Self::parse_header(tokens)?;
        let (tokens, postings) = core::many(Self::parse_posting).parse(tokens)?;
        let (tokens, _) = core::parse_dedent(tokens)?;

        Ok((&[], ast::Transaction { header, postings }))
    }
}

#[cfg(test)]
mod test {
    use super::TransactionParser;
    use crate::input::parse::core::{Timestamp, Token};

    fn sample_timestamp() -> Timestamp {
        Timestamp::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00").unwrap()
    }

    #[test]
    fn test_header_simple() {
        let ts = sample_timestamp();
        let tokens = [Token::Timestamp(ts), Token::LineSeparator];
        let (rest, result) = TransactionParser::parse_header(&tokens).expect("Failed.");
        assert_eq!(result.timestamp, sample_timestamp());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_simple() {
        let ts = sample_timestamp();
        let tokens = [
            Token::Timestamp(ts),
            Token::LineSeparator,
            Token::Indent,
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
            Token::Identifier("expense".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
        ];
        let (rest, result) = TransactionParser::parse(&tokens).expect("Failed.");
        assert!(rest.is_empty());
    }
}
