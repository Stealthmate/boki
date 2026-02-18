use super::core::StringScanner;

use super::core::NomResult;
use nom::bytes::complete::is_a;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::Parser;

/// Consumes one or more consecutive whitespace characters (space, tab)
pub fn whitespace(input: StringScanner) -> NomResult<()> {
    let (input, _) = is_a(" \t").parse(input)?;
    Ok((input, ()))
}

/// Consumes a single newline character.
pub fn newline(input: StringScanner) -> NomResult<()> {
    let (input, _) = is_a("\n\r").parse(input)?;
    Ok((input, ()))
}

/// Consumes one or more consecutive empty lines.
pub fn linespace(input: StringScanner) -> NomResult<()> {
    let (input, _) = many1(terminated(opt(whitespace), newline)).parse(input)?;
    Ok((input, ()))
}
