use crate::input::contracts::tokens::{self, Token};
use nom::combinator::{all_consuming, opt};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{Input, Parser};

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
    let mut tokens = vec![];
    let mut remaining = input;
    loop {
        let (rest, _) = opt(whitespace::linespace).parse(remaining)?;
        let Ok((rest, t)) = lex_single_token(rest) else {
            break;
        };
        remaining = rest;
        let loc = input.len() - remaining.len();
        tokens.push(core::DecoratedToken::new(t, loc));
    }

    if !tokens.is_empty() {
        tokens.push(core::DecoratedToken::new(Token::LineSeparator, 0));
    }

    let folded = tokens.into_iter().fold(vec![], fold_tokens);

    Ok((input, folded))
}

#[cfg(test)]
mod test {
    use crate::input::contracts::tokens::Token;
    use crate::input::lex::lex_string;

    #[test]
    fn test_empty() {
        let input = "";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert!(tokens.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_linespace_only() {
        let input = "   \t  \n   \n\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert!(tokens.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_1_token() {
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
    fn test_2_token_with_space_inbetween() {
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
        assert_eq!(
            the_tokens,
            vec![Token::YamlMatter(mapping), Token::LineSeparator,]
        );
        assert!(rest.is_empty());
    }
}
