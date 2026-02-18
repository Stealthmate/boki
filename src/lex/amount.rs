use super::core::{NomResult, StringScanner};
use super::whitespace;
use crate::tokens::Token;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::Parser;

pub fn lex(input: StringScanner) -> NomResult<Token> {
    let original_input = input.clone();

    let (input, _) = opt(whitespace::whitespace).parse(input)?;

    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, initial_digits) = digit1.parse(input)?;
    let (input, rest_digits) = many0(preceded(tag(","), digit1)).parse(input)?;

    let numstr = format!(
        "{}{}{}",
        sign.unwrap_or('+'),
        initial_digits.as_str(),
        rest_digits
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .join("")
    );

    let amount: i64 = str::parse(&numstr).map_err(|_| {
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
    fn test_amount_succeeds(#[case] input: &str, #[case] result: i64) {
        let (_, output) = super::lex(input.into()).expect("Failed.");
        let super::Token::Amount(x) = output else {
            panic!("Should have been an identifier.");
        };
        assert_eq!(x, result);
    }

    #[rstest::rstest]
    #[case::non_numeric("asfasf")]
    fn test_amount_fails(#[case] input: &str) {
        super::lex(input.into()).expect_err("Failed.");
    }
}
