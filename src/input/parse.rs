use crate::input::contracts::ast;
use crate::input::contracts::tokens;

mod core;
mod set_attribute;
mod transaction;

use core::{Parser, ParserResult};

fn parse_comments(tokens: &[tokens::Token]) -> ParserResult<'_, ()> {
    let mut rest = tokens;

    loop {
        if !matches!(rest.first(), Some(x) if x.is_comment()) {
            break;
        }
        rest = &rest[1..];
    }

    Ok((rest, ()))
}

fn parse_transaction(tokens: &[tokens::Token]) -> ParserResult<'_, ast::ASTNode> {
    let (tokens, t) = transaction::TransactionParser::parse(tokens)?;
    Ok((tokens, ast::ASTNode::Transaction(t)))
}

fn parse_set_attribute(tokens: &[tokens::Token]) -> ParserResult<'_, ast::ASTNode> {
    let (tokens, (name, value)) = set_attribute::SetAttributeParser::new().parse(tokens)?;
    Ok((tokens, ast::ASTNode::SetAttribute(name, value)))
}

fn parse_node(tokens: &[tokens::Token]) -> ParserResult<'_, ast::ASTNode> {
    let parsers = [parse_transaction, parse_set_attribute];
    let result = core::one_of(&parsers).parse(tokens);
    result
}

pub fn parse_tokens(tokens: &[tokens::Token]) -> ParserResult<'_, Vec<ast::ASTNode>> {
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
    use crate::input::parse::tokens::Token;
    use chrono::DateTime;

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
            Token::Indent,
            Token::Identifier("expense".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
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
