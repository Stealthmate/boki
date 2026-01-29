use crate::input::parse::Token;
use nom::bytes::complete::is_a;
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
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

fn lex_single_token(input: &str) -> LexResult<'_, Token> {
    Ok(("", Token::Indent))
}

pub fn lex_string(input: &str) -> LexResult<'_, Vec<Token>> {
    preceded(
        opt(lex_linespace),
        many0(terminated(lex_single_token, lex_linespace)),
    )
    .parse(input)
}

#[cfg(test)]
mod test {
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

    // #[test]
    // fn test_single_token() {
    //     let input = "example";
    //     let (rest, tokens) = lex_string(input).expect("Failed.");
    //     assert_eq!(tokens.len(), 1);
    //     assert!(rest.is_empty());
    // }
}
