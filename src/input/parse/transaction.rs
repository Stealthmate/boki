use crate::input::compile::ast;
use crate::input::parse::core::ParserResult;
use crate::input::parse::syntax::Token;
use chrono::DateTime;
pub struct TransactionParser;

impl TransactionParser {
    pub fn parse(tokens: &[Token]) -> ParserResult<'_, ast::Transaction> {
        Ok((
            &[],
            ast::Transaction {
                header: ast::TransactionHeader {
                    timestamp: DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00")
                        .unwrap(),
                },
                postings: vec![],
            },
        ))
    }
}
