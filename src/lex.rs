//! The lexer stage.
//!
//! This module is responsible for producing Tokens from
//! a string of characters.
//!
//! The lexer depends only on the Token type.
//!
//! TODO: actual contract of the lexer

use nom::combinator::opt;
use nom::Parser;

use crate::tokens::{self, Token};

mod amount;
mod basic;
mod core;
mod identifier;
mod timestamp;
mod whitespace;

use core::{LexerResult, NomResult, StringScanner};

pub use core::{LexerError, LexerErrorDetails};

pub type TokenLocation = usize;

#[derive(Clone, Debug)]
pub struct DecoratedToken(Token, TokenLocation);

impl DecoratedToken {
    fn new(t: Token, i: TokenLocation) -> Self {
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
            input.location(),
            core::LexerErrorDetails::NothingMatched,
        ));
    };

    Ok(result)
}

fn fold_tokens(mut a: Vec<DecoratedToken>, t: DecoratedToken) -> Vec<DecoratedToken> {
    let Some(last) = a.last() else {
        a.push(t);
        return a;
    };

    match (last.token().name(), t.token().name()) {
        // consecutive newlines are combined into one
        (tokens::TOKEN_NAME_LINE_SEPARATOR, tokens::TOKEN_NAME_LINE_SEPARATOR) => a,
        // indent followed by newline is considered as a single newline
        (tokens::TOKEN_NAME_INDENT, tokens::TOKEN_NAME_LINE_SEPARATOR) => {
            let mut i = a.pop().unwrap().location();
            // The second-to-last token could be a newline. In that case, we pop that as well.
            if a.last()
                .map(|t| matches!(t.token(), Token::LineSeparator))
                .unwrap_or(false)
            {
                i = a.pop().unwrap().location();
            }

            // Finally we put a newline at the end.
            a.push(DecoratedToken::new(Token::LineSeparator, i));
            a
        }
        // Indent not following a newline is skipped
        (n, tokens::TOKEN_NAME_INDENT) if n != tokens::TOKEN_NAME_LINE_SEPARATOR => a,
        _ => {
            a.push(t);
            a
        }
    }
}

fn nom_lex_string(input: StringScanner) -> NomResult<Vec<DecoratedToken>> {
    let (input, _) = opt(whitespace::linespace).parse(input)?;

    let mut remaining = input;

    let mut tokens = vec![];
    loop {
        if remaining.is_empty() {
            break;
        }

        let loc = remaining.location();
        let (rest, t) = lex_single_token(remaining).map_err(|e| {
            e.map(|mut e1| {
                let end = tokens.len();
                let start = end.saturating_sub(3);
                e1.location = loc;
                e1.previous_tokens = tokens[start..end].to_vec();
                e1
            })
        })?;
        remaining = rest;

        if let Token::LineSeparator = &t {
            let (rest, _) = opt(whitespace::linespace).parse(remaining)?;
            remaining = rest;
        };
        tokens.push(DecoratedToken::new(t, loc));
    }

    let mut folded = tokens.into_iter().fold(vec![], fold_tokens);

    folded.push(DecoratedToken::new(Token::Eof, remaining.eof_idx()));

    Ok((remaining, folded))
}

pub fn lex_string(content: &str) -> LexerResult<Vec<DecoratedToken>> {
    let scanner = StringScanner::from(content);
    let (_, result) = nom_lex_string(scanner)?;
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
        let input = "\n    \nfoo  \n \n \t\t  \n\n\nbar\n\n";
        let tokens = lex_string(input).expect("Failed.");
        let the_tokens: Vec<Token> = tokens.iter().map(|x| x.token().clone()).collect();
        assert_eq!(
            the_tokens,
            vec![
                Token::Identifier("foo".to_string()),
                Token::LineSeparator,
                Token::Identifier("bar".to_string()),
                Token::LineSeparator,
                Token::Eof
            ]
        );
    }
}
