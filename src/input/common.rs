use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::Err;
use nom_language::error::VerboseError;

pub type ParserResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

pub trait InputParser<T> {
    fn parse(input: &str) -> ParserResult<'_, T>;
}

pub fn error_unexpected<'a>(input: &'a str, expected: &'a str) -> Err<VerboseError<&'a str>> {
    nom::Err::Error(nom::error::make_error(input, ErrorKind::IsNot))
}

pub fn parse_whitespace(input: &str) -> ParserResult<'_, &str> {
    use nom::Parser;
    let (input, _) = nom::multi::many0(nom::bytes::complete::tag(" ")).parse(input)?;
    Ok((input, ""))
}

pub fn eol(input: &str) -> ParserResult<'_, ()> {
    use nom::Parser;

    if input.is_empty() {
        return Err(nom::Err::Error(
            nom_language::error::VerboseError::from_error_kind(input, nom::error::ErrorKind::Eof),
        ));
    }

    let (input, _) = parse_whitespace(input)?;
    if input.is_empty() {
        return Err(nom::Err::Error(
            nom_language::error::VerboseError::from_error_kind(input, nom::error::ErrorKind::Eof),
        ));
    }

    let (input, _) = nom::bytes::complete::tag("\n").parse(input)?;

    Ok((input, ()))
}

pub fn parse_eols(input: &str) -> ParserResult<'_, ()> {
    use nom::Parser;
    let (next_input, _) = nom::multi::many0(eol).parse(input)?;

    Ok((next_input, ()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eol_parses_empty_line() {
        assert_eq!(eol("\n"), Ok(("", ())))
    }

    #[test]
    fn eol_parses_empty_line_with_spaces() {
        assert_eq!(eol("   \n"), Ok(("", ())))
    }

    #[test]
    fn eol_fails_on_empty_string() {
        assert!(eol("").is_err())
    }

    #[test]
    fn eol_fails_on_non_empty_line() {
        assert!(eol("  asdasd").is_err());
        assert!(eol("2026-01-01").is_err())
    }

    #[test]
    fn parse_eols_parses_empty_string() {
        assert_eq!(parse_eols(""), Ok(("", ())));
    }

    #[test]
    fn parse_eols_parses_one_line() {
        assert_eq!(parse_eols("  \n"), Ok(("", ())));
    }

    #[test]
    fn parse_eols_parses_two_lines() {
        assert!(parse_eols("\n    \n").is_ok());
    }
}
