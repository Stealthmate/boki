use super::core::{NomResult, StringScanner};
use crate::tokens::Token;
use nom::bytes::complete::is_a;
use nom::combinator::{opt, recognize};
use nom::sequence::pair;
use nom::Parser;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJQKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "-_:";

pub fn lex(input: StringScanner) -> NomResult<Token> {
    let first: String = String::new() + ALPHA + SYMBOLS;
    let rest: String = String::new() + ALPHA + NUMBERS + SYMBOLS;

    let (input, x) =
        recognize(pair(is_a(first.as_str()), opt(is_a(rest.as_str())))).parse(input)?;

    Ok((input, Token::Identifier(x.as_str().to_string())))
}

#[cfg(test)]
mod test {
    #[rstest::rstest]
    #[case::alpha_only("foo")]
    #[case::alpha_alphanum("f123")]
    #[case::alpha_symbols("foo:bar-baz")]
    #[case::underscore_prefix("_f123")]
    fn test_identifier_succeeds(#[case] input: &str) {
        let (_, output) = super::lex(input.into()).expect("Failed.");
        let super::Token::Identifier(x) = output else {
            panic!("Should have been an identifier.");
        };
        assert_eq!(x, input);
    }

    #[rstest::rstest]
    #[case::numeric_prefix("1asfasf")]
    fn test_identifier_fails(#[case] input: &str) {
        super::lex(input.into()).expect_err("Failed.");
    }
}
