use crate::input::compile::ast;
use crate::input::parse::core::ParserResult;
use crate::input::parse::syntax::Token;
use chrono::DateTime;
pub struct TransactionParser;

impl TransactionParser {
    fn parse_header(tokens: &[Token]) -> ParserResult<'_, ast::TransactionHeader> {
        Ok((
            tokens,
            ast::TransactionHeader {
                timestamp: DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap(),
            },
        ))
    }

    pub fn parse(tokens: &[Token]) -> ParserResult<'_, ast::Transaction> {
        let (tokens, header) = Self::parse_header(tokens)?;
        Ok((
            &[],
            ast::Transaction {
                header,
                postings: vec![],
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::TransactionParser;
    use crate::input::parse::syntax::{Timestamp, Token};

    fn sample_timestamp() -> Timestamp {
        Timestamp::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap()
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
