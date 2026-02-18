//! The parser stage.
//!
//! This module is responsible for producing AST nodes from
//! a potentially incomplete list of tokens.
//!
//! The parser depends only on the Token type.
//!
//! The parser assumes that end of input is signified
//! by the presence of an Eof token. If the parser consumes
//! all tokens without seeing an Eof token, it throws a special kind of error.
//! The caller should catch that error and re-attepmt parsing
//! when more tokens are available.

use crate::ast;
use crate::lexparse::contracts::tokens;

mod basic;
mod combinators;
mod core;
mod set_attributes;
mod transaction;

use core::{Parser, ParserResult};
pub(super) use core::{ParserError, ParserErrorDetails, TokenScanner};

fn parse_initial_whitespace_and_comments(scanner: &mut TokenScanner) -> ParserResult<()> {
    loop {
        match core::peek_next(scanner)? {
            tokens::Token::Comment(_) => {}
            tokens::Token::LineSeparator => {}
            _ => return Ok(()),
        };

        scanner.advance(1)?;
    }
}

fn parse_transaction(scanner: &mut TokenScanner) -> ParserResult<ast::ASTNode> {
    transaction::TransactionParser::parse(scanner).map(ast::ASTNode::Transaction)
}

fn parse_set_attribute(scanner: &mut TokenScanner) -> ParserResult<ast::ASTNode> {
    set_attributes::SetAttributeParser::new()
        .parse(scanner)
        .map(|(x, y)| ast::ASTNode::SetAttribute(x, y))
}

fn parse_a_node(scanner: &mut TokenScanner) -> ParserResult<ast::ASTNode> {
    let parsers = [parse_transaction, parse_set_attribute];
    let node = combinators::one_of(&parsers).parse(scanner)?;
    Ok(node)
}

/// The main entrypoint for the parser stage.
///
/// Reads tokens from the provided scanner, skipping through all
/// initial whitespace and comments, and attempts to parse an AST node.
/// If it encounters an Eof token before attempting a node parse, returns `None`.
pub fn parse_node(scanner: &mut TokenScanner) -> ParserResult<Option<ast::ASTNode>> {
    parse_initial_whitespace_and_comments(scanner)?;

    if let tokens::Token::Eof = core::peek_next(scanner)? {
        return Ok(None);
    };

    parse_a_node(scanner).map(Some)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexparse::contracts::tokens::Token;
    use crate::lexparse::parse::core;

    #[test]
    fn test_eof_token() {
        let mut scanner = super::TokenScanner::from_slice(&[Token::Eof]);
        let node = parse_node(&mut scanner).expect("Failed.");
        assert!(node.is_none());
    }

    #[test]
    fn test_no_tokens() {
        let mut scanner = super::TokenScanner::from_slice(&[]);
        let err = parse_node(&mut scanner).expect_err("Should have failed.");
        assert!(matches!(err.details, core::ParserErrorDetails::Incomplete));
    }

    #[test]
    fn test_initial_whitespace_no_eof() {
        let mut scanner = super::TokenScanner::from_slice(&[
            tokens::Token::LineSeparator,
            tokens::Token::Comment("foo".to_string()),
            tokens::Token::LineSeparator,
        ]);
        let err = parse_node(&mut scanner).expect_err("Should have failed.");
        assert!(matches!(err.details, core::ParserErrorDetails::Incomplete));
        assert_eq!(err.location, 3)
    }

    #[test]
    fn test_single_transaction() {
        let ts = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap();
        let mut scanner = super::TokenScanner::from_slice(&[
            tokens::Token::Timestamp(ts),
            tokens::Token::LineSeparator,
            tokens::Token::Indent,
            tokens::Token::Identifier("asset".to_string()),
            tokens::Token::AccountSeparator,
            tokens::Token::Identifier("cce".to_string()),
            tokens::Token::AccountSeparator,
            tokens::Token::Identifier("cash".to_string()),
            tokens::Token::PostingSeparator,
            tokens::Token::Identifier("JPY".to_string()),
            tokens::Token::PostingSeparator,
            tokens::Token::Amount(1000),
            tokens::Token::LineSeparator,
            tokens::Token::Indent,
            tokens::Token::Identifier("expense".to_string()),
            tokens::Token::PostingSeparator,
            tokens::Token::Identifier("JPY".to_string()),
            tokens::Token::PostingSeparator,
            tokens::Token::Amount(1000),
            tokens::Token::LineSeparator,
        ]);
        let node = parse_node(&mut scanner).expect("Failed.");
        assert!(matches!(node, Some(ast::ASTNode::Transaction(_))));
    }
}
