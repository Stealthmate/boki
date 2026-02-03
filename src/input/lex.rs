use crate::input::parse::{Keyword, Token};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::none_of;
use nom::combinator::{all_consuming, opt, peek};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::Parser;

mod amount;
mod core;
mod identifier;
mod timestamp;
mod whitespace;

use core::LexResult;

fn lex_indent(input: &str) -> LexResult<'_, Token> {
    let (input, _) = tag("  ").parse(input)?;
    let (input, _) = peek(none_of("\n")).parse(input)?;
    Ok((input, Token::Indent))
}

fn lex_account_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag("/")).parse(input)?;
    Ok((input, Token::AccountSeparator))
}

fn lex_posting_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag(";")).parse(input)?;
    Ok((input, Token::PostingSeparator))
}

fn lex_line_separator(input: &str) -> LexResult<'_, Token> {
    let (input, _) = preceded(opt(whitespace::whitespace), tag("\n")).parse(input)?;
    Ok((input, Token::LineSeparator))
}

fn lex_keyword(input: &str) -> LexResult<'_, Token> {
    let (input, kw) = alt([tag("set")]).parse(input)?;

    let the_kw = match kw {
        "set" => Keyword::Set,
        _ => {
            panic!("Unhandled keyword. This is a bug.");
        }
    };

    Ok((input, Token::Keyword(the_kw)))
}

fn lex_yaml_matter(input: &str) -> LexResult<'_, Token> {
    let start = "  ---\n";
    let end = "\n  ---";
    let (input, yamlstr) = delimited(tag(start), take_until(end), tag(end)).parse(input)?;
    let stripped = yamlstr.replace("\n  ", "\n");
    println!("stripped:\n{stripped}");
    let Ok(parsed) = serde_yaml::from_str(&stripped) else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };
    Ok((input, Token::YamlMatter(parsed)))
}

fn lex_single_token(input: &str) -> LexResult<'_, Token> {
    let mut results = vec![];

    if let Ok((rest, t)) = lex_yaml_matter(input) {
        return Ok((rest, t));
    }

    if let Ok((rest, _)) = lex_indent(input) {
        return Ok((rest, Token::Indent));
    }

    for mut lexer in [
        lex_keyword,
        identifier::lex,
        timestamp::lex,
        amount::lex,
        lex_indent,
        lex_account_separator,
        lex_posting_separator,
        lex_line_separator,
    ] {
        if let Ok(x) = lexer.parse(input) {
            results.push(x);
        }
    }

    // Note: we rely on preserving the initial order here.
    results.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
    let Some(result) = results.first().cloned() else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };

    Ok(result)
}

pub fn lex_string(input: &str) -> LexResult<'_, Vec<Token>> {
    let (input, mut tokens) = all_consuming(preceded(
        opt(whitespace::linespace),
        many0(lex_single_token),
    ))
    .parse(input)?;

    if !tokens.is_empty() {
        tokens.push(Token::LineSeparator);
    }

    let folded = tokens
        .into_iter()
        .fold(vec![], |mut a, t| match (a.last(), &t) {
            (Some(Token::LineSeparator), Token::LineSeparator) => a,
            // Indent token is only considered an indent if it follows a new line.
            // Otherwise we treat it as whitespace and skip it altogether.
            // TODO: this is a hack. Need to implement it properly eventually...
            (Some(t), Token::Indent) if !matches!(t, Token::LineSeparator) => a,
            _ => {
                a.push(t);
                a
            }
        });

    Ok((input, folded))
}

#[cfg(test)]
mod test {
    use crate::input::lex::lex_string;
    use crate::input::parse::Token;

    #[test]
    fn test_empty() {
        let input = "";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert!(tokens.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_linespace_only() {
        let input = "   \t  \n   \n\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert!(tokens.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_1_token() {
        let input = "foo";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let tok = tokens.first().expect("Should have lexed at least 1 token.");
        let Token::Identifier(x) = tok else {
            panic!("Should have been an identifier token.");
        };
        assert_eq!(x, "foo");
        assert!(rest.is_empty());
    }

    #[test]
    fn test_2_token_with_space_inbetween() {
        let input = "\n    \nfoo  \n \n \t\t  \n\n\nbar\n\n";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("foo".to_string()),
                Token::LineSeparator,
                Token::Identifier("bar".to_string()),
                Token::LineSeparator,
            ]
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_yaml_matter() {
        let input = "  ---\n  foo: bar\n  ---";
        let (rest, tokens) = lex_string(input).expect("Failed.");
        let mapping: serde_yaml::Mapping =
            serde_yaml::from_str("foo: bar").expect("Invalid test case.");
        assert_eq!(
            tokens,
            vec![Token::YamlMatter(mapping), Token::LineSeparator,]
        );
        assert!(rest.is_empty());
    }
}
