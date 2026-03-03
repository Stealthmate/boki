//! # The Lexer
//!
//! The lexer takes as input a string of characters and produces a list of
//! decorated tokens, i.e. [DecoratedToken]. It does only that, without transforming
//! the list of tokens in _any_ way, except for inserting a [Token::Eof] token at the end.

use nom::combinator::opt;
use nom::Parser;

use crate::tokens::Token;

mod amount;
mod basic;
mod core;
mod error;
mod identifier;
mod timestamp;
mod whitespace;

use core::{LexerResult, NomResult, StringScanner};

pub use error::{LexerError, LexerErrorDetails};

pub type TokenLocation = usize;

#[derive(Clone, Debug)]
pub struct DecoratedToken(Token, TokenLocation);

impl DecoratedToken {
    pub fn new(t: Token, i: TokenLocation) -> Self {
        Self(t, i)
    }

    pub fn token(&self) -> &Token {
        &self.0
    }

    pub fn location(&self) -> usize {
        self.1
    }
}

fn compare_match_length(
    a: &(StringScanner, Token),
    b: &(StringScanner, Token),
) -> std::cmp::Ordering {
    a.0.as_str().len().cmp(&b.0.as_str().len())
}

fn lex_single_token(input: StringScanner) -> NomResult<Token> {
    // First we lexer in order of priority
    for mut lexer in [basic::lex_yaml_matter, basic::lex_indent] {
        if let Ok(x) = lexer.parse(input.clone()) {
            return Ok(x);
        }
    }

    // If none of the priority lexers succeed,
    // we take the longest match of the remaining ones.

    let mut results = vec![];
    for mut lexer in [
        basic::lex_whitespace,
        basic::lex_comment,
        basic::lex_keyword,
        identifier::lex,
        timestamp::lex,
        amount::lex,
        basic::lex_account_separator,
        basic::lex_posting_separator,
        basic::lex_line_separator,
    ] {
        if let Ok(x) = lexer.parse(input.clone()) {
            results.push(x);
        }
    }

    // Note: we rely on preserving the initial order here.
    results.sort_by(compare_match_length);

    let Some(result) = results.first().cloned() else {
        return Err(core::error_at(
            &input,
            input.location(),
            LexerErrorDetails::NothingMatched,
        ));
    };

    Ok(result)
}

fn nom_lex_string(input: StringScanner) -> LexerResult<Vec<DecoratedToken>> {
    let content = input.content.clone();
    let (input, _) = opt(whitespace::linespace)
        .parse(input)
        .map_err(|e| match e {
            nom::Err::Incomplete(_) => LexerError {
                content: content.clone(),
                location: 0,
                details: LexerErrorDetails::InternalError("incomplete".to_string()),
                previous_tokens: vec![],
            },
            nom::Err::Error(e1) => e1,
            nom::Err::Failure(e1) => e1,
        })?;

    let mut remaining = input;

    let mut tokens = vec![];
    loop {
        if remaining.is_empty() {
            break;
        }

        let loc = remaining.location();
        let (rest, t) = lex_single_token(remaining).map_err(|e| match e {
            nom::Err::Incomplete(_) => LexerError {
                content: content.clone(),
                location: loc,
                details: LexerErrorDetails::InternalError("incomplete".to_string()),
                previous_tokens: tokens.clone(),
            },
            nom::Err::Error(e1) => LexerError {
                content: content.clone(),
                location: loc,
                details: e1.details,
                previous_tokens: tokens.clone(),
            },
            nom::Err::Failure(e1) => LexerError {
                content: content.clone(),
                location: loc,
                details: e1.details,
                previous_tokens: tokens.clone(),
            },
        })?;
        remaining = rest;

        tokens.push(DecoratedToken::new(t.clone(), loc));
        remaining.set_last_token(t);
    }

    tokens.push(DecoratedToken::new(Token::Eof, remaining.eof_idx()));

    Ok(tokens)
}

pub fn lex_string(content: &str) -> LexerResult<Vec<DecoratedToken>> {
    let scanner = StringScanner::from(content);
    let result = nom_lex_string(scanner)?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Token;

    #[test]
    fn test_inserts_eof_token_at_end() {
        let input = "";
        let tokens = lex_string(input).expect("Failed.");
        let t = tokens.first().map(|t| t.token());
        assert!(matches!(t, Some(Token::Eof)));
    }

    #[test]
    fn test_skips_initial_linespace() {
        let input = "   \t  \n   \n\n\n";
        let tokens = lex_string(input).expect("Failed.");
        let t = tokens.first().map(|t| t.token());
        assert!(matches!(t, Some(Token::Eof)));
    }

    #[test]
    fn test_lexes_1_token() {
        let input = "foo";
        let tokens = lex_string(input).expect("Failed.");
        let tok = tokens.first().expect("Should have lexed at least 1 token.");
        let Token::Identifier(x) = &tok.token() else {
            panic!("Should have been an identifier token.");
        };
        assert_eq!(x, "foo");
    }

    #[test]
    fn test_lexes_2_tokens_with_space_inbetween() {
        let input = "\n    \nfoo  \n\nbar\n\n";
        let tokens = lex_string(input).expect("Failed.");
        let the_tokens: Vec<Token> = tokens.iter().map(|x| x.token().clone()).collect();
        assert_eq!(
            the_tokens,
            vec![
                Token::Identifier("foo".to_string()),
                Token::Whitespace,
                Token::LineSeparator,
                Token::LineSeparator,
                Token::Identifier("bar".to_string()),
                Token::LineSeparator,
                Token::LineSeparator,
                Token::Eof
            ]
        );
    }
}
