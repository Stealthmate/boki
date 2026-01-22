use nom::error::ErrorKind;
use nom::Err;
use nom_language::error::VerboseError;

pub type ParserResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

pub trait InputParser<T> {
    fn parse(input: &str) -> ParserResult<'_, T>;
}

pub fn error_unexpected<'a>(input: &'a str, expected: &'a str) -> Err<VerboseError<&'a str>> {
    nom::Err::Error(nom::error::make_error(input, ErrorKind::IsNot))
}
