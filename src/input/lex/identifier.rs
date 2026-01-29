use super::core::LexResult;
use crate::input::parse::Token;
use nom::bytes::complete::is_a;
use nom::combinator::{opt, recognize};
use nom::sequence::pair;
use nom::Parser;

const ALPHA_UNDERSCORE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJQKLMNOPQRSTUVWXYZ_";
const ALPHA_UNDERSCORE_DIGIT: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJQKLMNOPQRSTUVWXYZ_0123456789";

pub fn lex(input: &str) -> LexResult<'_, Token> {
    let (input, x) = recognize(pair(
        is_a(ALPHA_UNDERSCORE),
        opt(is_a(ALPHA_UNDERSCORE_DIGIT)),
    ))
    .parse(input)?;

    Ok((input, Token::Identifier(x.to_string())))
}

#[cfg(test)]
mod test {
    #[rstest::rstest]
    #[case::alpha_only("foo")]
    #[case::alpha_alphanum("f123")]
    #[case::underscore_prefix("_f123")]
    fn test_identifier_succeeds(#[case] input: &str) {
        let (rest, output) = super::lex(input).expect("Failed.");
        let super::Token::Identifier(x) = output else {
            panic!("Should have been an identifier.");
        };
        assert_eq!(x, input);
    }

    #[rstest::rstest]
    #[case::numeric_prefix("1asfasf")]
    fn test_identifier_fails(#[case] input: &str) {
        super::lex(input).expect_err("Failed.");
    }
}
