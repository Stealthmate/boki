use super::core::LexResult;
use nom::bytes::complete::is_a;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::Parser;

/// Consumes one or more consecutive whitespace characters (space, tab)
pub fn whitespace(input: &str) -> LexResult<'_, ()> {
    let (input, _) = is_a(" \t").parse(input)?;
    Ok((input, ()))
}

/// Consumes a single newline character.
pub fn newline(input: &str) -> LexResult<'_, ()> {
    let (input, _) = is_a("\n\r").parse(input)?;
    Ok((input, ()))
}

/// Consumes one or more consecutive empty lines.
pub fn linespace(input: &str) -> LexResult<'_, ()> {
    let (input, _) = many1(terminated(opt(whitespace), newline)).parse(input)?;
    Ok((input, ()))
}
