use super::core::LexResult;
use crate::input::lex::whitespace;
use crate::input::parse::Token;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::Parser;

pub fn lex(input: &str) -> LexResult<'_, Token> {
    let original_input = input;

    let (input, _) = opt(whitespace::whitespace).parse(input)?;

    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, initial_digits) = digit1.parse(input)?;
    let (input, rest_digits) = many0(preceded(tag(","), digit1)).parse(input)?;

    let numstr = format!(
        "{}{}{}",
        sign.unwrap_or('+'),
        initial_digits,
        rest_digits.join("")
    );
    let amount: i64 = str::parse(&numstr).map_err(|e| {
        nom::Err::Error(nom::error::make_error(
            original_input,
            nom::error::ErrorKind::IsNot,
        ))
    })?;

    Ok((input, Token::Amount(amount)))
}

#[cfg(test)]
mod test {
    #[rstest::rstest]
    #[case::integer("1000", 1000)]
    #[case::positive_integer("+1000", 1000)]
    #[case::negative_integer("-1000", -1000)]
    #[case::integer_with_thousands_separators("1,000,000", 1_000_000)]
    fn test_identifier_succeeds(#[case] input: &str, #[case] result: i64) {
        let (rest, output) = super::lex(input).expect("Failed.");
        let super::Token::Amount(x) = output else {
            panic!("Should have been an identifier.");
        };
        assert_eq!(x, result);
    }

    #[rstest::rstest]
    #[case::non_numeric("asfasf")]
    fn test_identifier_fails(#[case] input: &str) {
        super::lex(input).expect_err("Failed.");
    }
}
