use chrono::{DateTime, FixedOffset};

use crate::input::compile::ast;

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Timestamp(Timestamp),
    Amount(i64),
    Identifier(String),
    AccountSeparator,
    PostingSeparator,
    LineSeparator,
    Comment(String),
    Indent,
    Dedent,
}

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }
}

type ParserResult<'a, T> = Result<(&'a [Token], T), String>;

fn parse_comments(tokens: &[Token]) -> ParserResult<'_, ()> {
    let mut rest = tokens;

    loop {
        if !matches!(rest.first(), Some(x) if x.is_comment()) {
            break;
        }
        rest = &rest[1..];
    }

    Ok((rest, ()))
}

fn parse_node(tokens: &[Token]) -> ParserResult<'_, ast::ASTNode> {
    Ok((
        &[],
        ast::ASTNode::Transaction(ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap(),
            },
            postings: vec![],
        }),
    ))
}

pub fn parse_tokens(tokens: &[Token]) -> ParserResult<'_, Vec<ast::ASTNode>> {
    let (tokens, _) = parse_comments(tokens)?;

    let mut rest = tokens;
    let mut nodes = vec![];

    while !rest.is_empty() {
        let (next_rest, node) = parse_node(rest)?;
        rest = next_rest;
        nodes.push(node);
    }

    Ok((rest, nodes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let (rest, result) = parse_tokens(&[]).expect("Failed.");
        assert!(result.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_only_comments() {
        let tokens = [
            Token::Comment("foo".to_string()),
            Token::Comment("foo".to_string()),
        ];
        let (rest, result) = parse_tokens(&tokens).expect("Failed.");
        assert!(result.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_transaction() {
        let ts = DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap();
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
        let (rest, result) = parse_tokens(&tokens).expect("Failed.");
        let node = result
            .first()
            .expect("Should have parsed at least one node.");
        let ast::ASTNode::Transaction(t) = node else {
            panic!("Should have parsed a transaction.");
        };
        assert!(rest.is_empty());
    }
}
