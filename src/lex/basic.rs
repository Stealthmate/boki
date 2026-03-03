use super::core::{error_at, LexerErrorDetails, StringScanner};
use crate::tokens::{Keyword, Token};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::Parser;

use super::core::NomResult;

fn internal_error<T>(loc: usize, msg: &str) -> NomResult<T> {
    Err(error_at(
        loc,
        LexerErrorDetails::InternalError(msg.to_string()),
    ))
}

pub fn lex_whitespace(input: StringScanner) -> NomResult<Token> {
    let t = input.get_last_token();
    match &t {
        None => internal_error(
            input.location(),
            "Whitespace only comes after a non-line separator token.",
        ),
        Some(Token::LineSeparator) => internal_error(
            input.location(),
            "Whitespace only comes after a non-line separator token.",
        ),
        Some(_) => {
            let (input, _) = many1(one_of(" \t")).parse(input)?;
            Ok((input, Token::Whitespace))
        }
    }
}

pub fn lex_indent(input: StringScanner) -> NomResult<Token> {
    let t = input.get_last_token();
    match &t {
        None => {}
        Some(Token::LineSeparator) => {}
        Some(_) => {
            return Err(error_at(
                input.location(),
                LexerErrorDetails::InternalError(
                    "Indent must be preceded by line separator.".to_string(),
                ),
            ))
        }
    }

    let (input, _) = tag("  ").parse(input)?;
    Ok((input, Token::Indent))
}

pub fn lex_account_separator(input: StringScanner) -> NomResult<Token> {
    let (input, _) = tag("/").parse(input)?;
    Ok((input, Token::AccountSeparator))
}

pub fn lex_posting_separator(input: StringScanner) -> NomResult<Token> {
    let (input, _) = tag(";").parse(input)?;
    Ok((input, Token::PostingSeparator))
}

pub fn lex_comment(input: StringScanner) -> NomResult<Token> {
    let (input, _) = tag("//").parse(input)?;
    let (input, content) = take_until("\n").parse(input)?;
    Ok((input, Token::Comment(content.as_str().to_string())))
}

pub fn lex_line_separator(input: StringScanner) -> NomResult<Token> {
    let (input, _) = tag("\n").parse(input)?;
    Ok((input, Token::LineSeparator))
}

pub fn lex_keyword(input: StringScanner) -> NomResult<Token> {
    let (input, kw) = alt([tag("set")]).parse(input)?;

    let the_kw = match kw.as_str() {
        "set" => Keyword::Set,
        _ => {
            panic!("Unhandled keyword. This is a bug.");
        }
    };

    Ok((input, Token::Keyword(the_kw)))
}

pub fn lex_yaml_matter(input: StringScanner) -> NomResult<Token> {
    let start = "  ---\n  ";
    let end = "\n  ---";
    let (input, yamlstr) = delimited(tag(start), take_until(end), tag(end)).parse(input)?;
    let stripped = yamlstr.as_str().replace("\n  ", "\n");
    let Ok(parsed) = serde_yaml::from_str(&stripped) else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };
    Ok((input, Token::YamlMatter(parsed)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        let input = "   \t   ";
        let mut scanner: StringScanner = input.into();
        scanner.set_last_token(Token::PostingSeparator);
        let (rest, t) = lex_whitespace(scanner).expect("Failed.");
        assert!(matches!(t, Token::Whitespace));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_comment() {
        let input = "// mutlibyte 🎉 万歳\n";
        let (rest, t) = lex_comment(input.into()).expect("Failed.");
        assert!(matches!(t, Token::Comment(_)));
        assert_eq!(rest.as_str(), "\n");
    }
}
