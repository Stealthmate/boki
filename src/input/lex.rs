use crate::input::parse::Token;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::combinator::{all_consuming, opt, peek};
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::Parser;

mod core;
mod identifier;
mod timestamp;
mod whitespace;

use core::LexResult;

fn lex_indent(input: &str) -> LexResult<'_, Token> {
    let (input, _) = tag("  ").parse(input)?;
    let (input, _) = peek(none_of("\n")).parse(input)?;
    Ok((input, Token::Indent))
}

fn lex_account_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag("/")).parse(input)?;
    Ok((input, Token::AccountSeparator))
}

fn lex_posting_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag(";")).parse(input)?;
    Ok((input, Token::PostingSeparator))
}

fn lex_single_token(input: &str) -> LexResult<'_, Token> {
    let mut results = vec![];
    for mut lexer in [
        identifier::lex,
        timestamp::lex,
        lex_indent,
        lex_account_separator,
        lex_posting_separator,
    ] {
        if let Ok(x) = lexer.parse(input) {
            results.push(x);
        }
    }

    results.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
    let Some(result) = results.first().cloned() else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };

    Ok(result)
}

pub fn lex_string(input: &str) -> LexResult<'_, Vec<Token>> {
    all_consuming(preceded(
        opt(whitespace::linespace),
        many0(terminated(lex_single_token, opt(whitespace::linespace))),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::input::lex::lex_string;
    use crate::input::parse::Token;

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
        let Token::Identifier(x) = tok else {
            panic!("Should have been an identifier token.");
        };
        assert_eq!(x, "foo");
        assert!(rest.is_empty());
    }

    #[test]
    fn test_2_token_with_space_inbetween() {
        let input = "\n    \nfoo  \n \n \t\t  \n\n\nbar\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert_eq!(tokens.len(), 2);
        assert!(rest.is_empty());
    }
}
