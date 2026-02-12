use crate::lexparse::contracts::tokens::{Keyword, Token};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::none_of;
use nom::combinator::{opt, peek};
use nom::sequence::{delimited, preceded};
use nom::Parser;

use crate::lexparse::lex::{core::LexResult, whitespace};

pub fn lex_indent(input: &str) -> LexResult<'_, Token> {
    let (input, _) = tag("  ").parse(input)?;
    let (input, _) = peek(none_of("\n")).parse(input)?;
    Ok((input, Token::Indent))
}

pub fn lex_account_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag("/")).parse(input)?;
    Ok((input, Token::AccountSeparator))
}

pub fn lex_posting_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag(";")).parse(input)?;
    Ok((input, Token::PostingSeparator))
}

fn lex_comment(input: &str) -> LexResult<'_, ()> {
    let (input, _) = tag("//").parse(input)?;
    let (input, _) = take_until("\n").parse(input)?;
    Ok((input, ()))
}

pub fn lex_line_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = opt(whitespace::whitespace).parse(input)?;
    let (input, _) = opt(lex_comment).parse(input)?;
    let (input, _) = tag("\n").parse(input)?;
    Ok((input, Token::LineSeparator))
}

pub fn lex_keyword(input: &str) -> LexResult<'_, Token> {
    let (input, kw) = alt([tag("set")]).parse(input)?;

    let the_kw = match kw {
        "set" => Keyword::Set,
        _ => {
            panic!("Unhandled keyword. This is a bug.");
        }
    };

    Ok((input, Token::Keyword(the_kw)))
}

pub fn lex_yaml_matter(input: &str) -> LexResult<'_, Token> {
    let start = "  ---\n  ";
    let end = "\n  ---";
    let (input, yamlstr) = delimited(tag(start), take_until(end), tag(end)).parse(input)?;
    let stripped = yamlstr.replace("\n  ", "\n");
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
    fn test_line_with_comment() {
        let input = "   // foobar\n";
        let (rest, t) = lex_line_separator(input).expect("Failed.");
        assert!(matches!(t, Token::LineSeparator));
        assert!(rest.is_empty());
    }
}
