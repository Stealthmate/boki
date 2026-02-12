//! The lexer stage.
//!
//! This module is responsible for producing Tokens from
//! a string of characters.
//!
//! The lexer depends only on the Token type.
//!
//! TODO: actual contract of the lexer

use crate::lexparse::contracts::tokens::{self, Token};
use nom::combinator::opt;
use nom::Parser;

mod amount;
mod basic;
pub mod core;
mod identifier;
mod timestamp;
mod whitespace;

use core::LexResult;

fn compare_match_length(a: &(&str, Token), b: &(&str, Token)) -> std::cmp::Ordering {
    a.0.len().cmp(&b.0.len())
}

fn lex_single_token(input: &str) -> LexResult<'_, Token> {
    // First we lexer in order of priority
    for mut lexer in [basic::lex_yaml_matter, basic::lex_indent] {
        if let Ok(x) = lexer.parse(input) {
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
        if let Ok(x) = lexer.parse(input) {
            results.push(x);
        }
    }

    // Note: we rely on preserving the initial order here.
    results.sort_by(compare_match_length);

    let Some(result) = results.first().cloned() else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };

    Ok(result)
}

fn fold_tokens(
    mut a: Vec<core::DecoratedToken>,
    t: core::DecoratedToken,
) -> Vec<core::DecoratedToken> {
    let Some(last) = a.last() else {
        a.push(t);
        return a;
    };

    match (last.token().name(), t.token().name()) {
        // consecutive newlines are combined into one
        (tokens::TOKEN_NAME_LINE_SEPARATOR, tokens::TOKEN_NAME_LINE_SEPARATOR) => a,
        // Indent not following a newline is skipped
        (n, tokens::TOKEN_NAME_INDENT) if n != tokens::TOKEN_NAME_LINE_SEPARATOR => a,
        _ => {
            a.push(t);
            a
        }
    }
}

pub fn lex_string(input: &str) -> LexResult<'_, Vec<core::DecoratedToken>> {
    let (input, _) = opt(whitespace::linespace).parse(input)?;

    let mut tokens = vec![];
    let mut remaining = input;
    loop {
        if remaining.is_empty() {
            break;
        }

        let Ok((rest, t)) = lex_single_token(remaining) else {
            break;
        };
        remaining = rest;
        let loc = input.len() - remaining.len();
        if let Token::LineSeparator = &t {
            let (rest, _) = opt(whitespace::linespace).parse(rest)?;
            remaining = rest;
        };
        tokens.push(core::DecoratedToken::new(t, loc));
    }

    let mut folded = tokens.into_iter().fold(vec![], fold_tokens);

    folded.push(core::DecoratedToken::new(Token::Eof, input.len()));

    Ok((remaining, folded))
}

#[cfg(test)]
mod test {
    use crate::lexparse::contracts::tokens::Token;
    use crate::lexparse::lex::lex_string;

    #[test]
    fn test_inserts_eof_token_at_end() {
        let input = "";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let t = tokens.first().map(|t| t.token());
        assert!(matches!(t, Some(Token::Eof)));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_skips_initial_linespace() {
        let input = "   \t  \n   \n\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let t = tokens.first().map(|t| t.token());
        assert!(matches!(t, Some(Token::Eof)));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_lexes_1_token() {
        let input = "foo";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let tok = tokens.first().expect("Should have lexed at least 1 token.");
        let Token::Identifier(x) = &tok.token() else {
            panic!("Should have been an identifier token.");
        };
        assert_eq!(x, "foo");
        assert!(rest.is_empty());
    }

    #[test]
    fn test_lexes_2_tokens_with_space_inbetween() {
        let input = "\n    \nfoo  \n \n \t\t  \n\n\nbar\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
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
        assert!(rest.is_empty());
    }

    #[test]
    fn test_yaml_matter() {
        let input = "  ---\n  foo: bar\n  ---";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let the_tokens: Vec<Token> = tokens.iter().map(|x| x.token().clone()).collect();
        let mapping: serde_yaml::Mapping =
            serde_yaml::from_str("foo: bar").expect("Invalid test case.");
        assert_eq!(the_tokens, vec![Token::YamlMatter(mapping), Token::Eof]);
        assert!(rest.is_empty());
    }
}
