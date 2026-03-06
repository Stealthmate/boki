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

use std::rc::Rc;

use boki::parsing::Parser;
use boki::{ast, lex, parsing, tokens};

mod set_attributes;
mod transaction;

fn fold_tokens(
    mut a: Vec<lex::DecoratedToken>,
    t: lex::DecoratedToken,
) -> Vec<lex::DecoratedToken> {
    let Some(last) = a.last() else {
        a.push(t);
        return a;
    };

    match (last.token().name(), t.token().name()) {
        // we skip comments
        (_, tokens::TOKEN_NAME_COMMENT) => a,
        // we skip whitespace
        (_, tokens::TOKEN_NAME_WHITESPACE) => a,
        // consecutive newlines are combined into one
        (tokens::TOKEN_NAME_LINE_SEPARATOR, tokens::TOKEN_NAME_LINE_SEPARATOR) => a,
        // indent followed by newline is considered as a single newline
        (tokens::TOKEN_NAME_INDENT, tokens::TOKEN_NAME_LINE_SEPARATOR) => {
            let mut i = a.pop().unwrap().location();
            // The second-to-last token could be a newline. In that case, we pop that as well.
            if a.last()
                .map(|t| matches!(t.token(), tokens::Token::LineSeparator))
                .unwrap_or(false)
            {
                i = a.pop().unwrap().location();
            }

            // Finally we put a newline at the end.
            a.push(lex::DecoratedToken::new(tokens::Token::LineSeparator, i));
            a
        }
        _ => {
            a.push(t);
            a
        }
    }
}

fn parse_initial_whitespace_and_comments(
    scanner: &mut parsing::TokenScanner,
) -> parsing::ParserResult<()> {
    loop {
        match parsing::peek_next(scanner)? {
            tokens::Token::Comment(_) => {}
            tokens::Token::LineSeparator => {}
            _ => return Ok(()),
        };

        scanner.advance(1)?;
    }
}

fn parse_transaction(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<ast::ASTNode> {
    transaction::TransactionParser::parse(scanner).map(ast::ASTNode::Transaction)
}

fn parse_set_attribute(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<ast::ASTNode> {
    set_attributes::SetAttributeParser::new()
        .parse(scanner)
        .map(|(x, y)| ast::ASTNode::SetAttribute(x, y))
}

fn parse_node(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<ast::ASTNode> {
    let parsers = [parse_transaction, parse_set_attribute];
    let node = parsing::one_of(&parsers).parse(scanner).map_err(|e| {
        let parsing::ParserErrorDetails::BranchingError(_, errs) = &e.details else {
            panic!("This should never happen.")
        };
        if errs
            .iter()
            .all(|err| matches!(err.details, parsing::ParserErrorDetails::Incomplete))
        {
            return parsing::ParserError {
                location: e.location,
                details: parsing::ParserErrorDetails::Incomplete,
            };
        }

        e
    })?;
    Ok(node)
}

pub fn parse_tokens(tokens: Rc<[lex::DecoratedToken]>) -> parsing::ParserResult<Vec<ast::ASTNode>> {
    let raw_tokens: Vec<tokens::Token> = tokens
        .iter()
        .cloned()
        .fold(vec![], fold_tokens)
        .iter()
        .map(|x| x.token().clone())
        .collect();
    let mut scanner = parsing::TokenScanner::from_slice(&raw_tokens);
    let mut nodes: Vec<ast::ASTNode> = vec![];

    parse_initial_whitespace_and_comments(&mut scanner)?;

    loop {
        if let Some(tokens::Token::Eof) = scanner.peek() {
            break;
        }
        let node = parse_node(&mut scanner)?;
        nodes.push(node);
    }

    Ok(nodes)
}

#[cfg(test)]
mod test {
    use super::*;
    use boki::parsing;

    #[test]
    fn test_no_tokens() {
        let mut scanner = parsing::TokenScanner::from_slice(&[]);
        let err = parse_node(&mut scanner).expect_err("Should have failed.");
        assert!(matches!(
            err.details,
            parsing::ParserErrorDetails::Incomplete
        ));
    }

    #[test]
    fn test_single_transaction() {
        let ts = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00").unwrap();
        let mut scanner = parsing::TokenScanner::from_slice(&[
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
        assert!(matches!(node, ast::ASTNode::Transaction(_)));
    }
}
