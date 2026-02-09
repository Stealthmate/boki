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

use crate::input::contracts::ast;
use crate::input::contracts::tokens;

pub mod core;

use core::{ParserResult, TokenScanner};

fn parse_initial_whitespace_and_comments(scanner: &mut TokenScanner) -> ParserResult<()> {
    loop {
        match core::peek_next(scanner)? {
            tokens::Token::Comment(_) => {}
            tokens::Token::LineSeparator => {}
            _ => return Ok(()),
        };

        scanner.advance(1);
    }
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

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::contracts::tokens::Token;
    use crate::input::parse_v2::core;

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
}
