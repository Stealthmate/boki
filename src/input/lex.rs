use crate::input::parse::Token;

type LexResult<'a, T> = nom::IResult<&'a str, T, nom_language::error::VerboseError<&'a str>>;

pub fn lex_string(string: &str) -> LexResult<'_, Vec<Token>> {
    Ok(("", vec![]))
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
}
