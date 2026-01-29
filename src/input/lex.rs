use crate::input::parse::Token;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, alphanumeric0, none_of};
use nom::combinator::{opt, peek};
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, terminated};
use nom::Parser;

type LexResult<'a, T> = nom::IResult<&'a str, T, nom_language::error::VerboseError<&'a str>>;

fn lex_whitespace(input: &str) -> LexResult<'_, ()> {
    let (input, _) = is_a(" \t").parse(input)?;
    Ok((input, ()))
}

fn lex_newline(input: &str) -> LexResult<'_, ()> {
    let (input, _) = is_a("\n\r").parse(input)?;
    Ok((input, ()))
}

fn lex_linespace(input: &str) -> LexResult<'_, ()> {
    let (input, _) = many1(terminated(opt(lex_whitespace), lex_newline)).parse(input)?;
    Ok((input, ()))
}

fn lex_indent(input: &str) -> LexResult<'_, Token> {
    let (input, _) = tag("  ").parse(input)?;
    let (input, _) = peek(none_of("\n")).parse(input)?;
    Ok((input, Token::Indent))
}

fn lex_identifier(input: &str) -> LexResult<'_, Token> {
    let (input, x) = pair(alpha1, alphanumeric0).parse(input)?;
    Ok((input, Token::Identifier(x.0.to_string() + x.1)))
}

fn lex_single_token(input: &str) -> LexResult<'_, Token> {
    let mut results = vec![];
    for mut lexer in [lex_identifier, lex_indent] {
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
    preceded(
        opt(lex_linespace),
        many0(terminated(lex_single_token, opt(lex_linespace))),
    )
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
