pub type LexResult<'a, T> = nom::IResult<&'a str, T, nom_language::error::VerboseError<&'a str>>;
