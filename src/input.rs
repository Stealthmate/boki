#![allow(dead_code)]
#![allow(unused_variables)]

use nom::error::ParseError;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    account: String,
    commodity: String,
    amount: i64,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    timestamp: chrono::DateTime<chrono::FixedOffset>,
    postings: Vec<Posting>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Object {
    transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq)]
enum Statement {
    TransactionStatement(Transaction),
}

#[derive(Debug, PartialEq)]
pub struct JournalAST(Vec<Statement>);

type MyResult<'a, T> = nom::IResult<&'a str, T, nom_language::error::VerboseError<&'a str>>;

fn parse_whitespace(input: &str) -> MyResult<'_, &str> {
    use nom::Parser;
    let (input, _) = nom::multi::many0(nom::bytes::complete::tag(" ")).parse(input)?;
    Ok((input, ""))
}

fn eol(input: &str) -> MyResult<'_, ()> {
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

fn parse_eols(input: &str) -> MyResult<'_, ()> {
    use nom::Parser;
    let (next_input, _) = nom::multi::many0(eol).parse(input)?;

    Ok((next_input, ()))
}

fn parse_timestamp(input: &str) -> nom::IResult<&str, chrono::DateTime<chrono::FixedOffset>> {
    match chrono::DateTime::parse_from_rfc3339(input) {
        Ok(x) => Ok(("", x)),
        Err(_) => Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        ))),
    }
}

fn parse_posting(input: &str) -> MyResult<'_, Posting> {
    todo!()
}

fn parse_statement_transaction(input: &str) -> MyResult<'_, Statement> {
    // let (input, timestamp) = parse_timestamp(input)?;
    // let (input, _) = eol(input)?;
    // let (input, postings) = nom::multi::many(2.., parse_posting).parse(input)?;

    let (input, _) = nom::combinator::rest(input)?;

    Ok((
        input,
        Statement::TransactionStatement(Transaction {
            timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000Z").unwrap(),
            postings: vec![
                Posting {
                    account: "assets/cce/cash".to_string(),
                    commodity: "JPY".to_string(),
                    amount: -1000,
                },
                Posting {
                    account: "expense".to_string(),
                    commodity: "JPY".to_string(),
                    amount: 1000,
                },
            ],
        }),
    ))
}

fn parse_statement(input: &str) -> MyResult<'_, Statement> {
    use nom::Parser;

    nom::branch::alt((parse_statement_transaction,)).parse(input)
}

fn parse_end_of_statement(input: &str) -> MyResult<'_, ()> {
    todo!()
}

fn parse_next_statement(input: &str) -> MyResult<'_, Statement> {
    use nom::Parser;
    let (input, _) = parse_eols(input)?;
    let (input, statement) = nom::combinator::opt(parse_statement).parse(input)?;
    let (input, _) = parse_end_of_statement(input)?;

    todo!()
}

pub fn parse_journal<'a>(input: &'a str) -> MyResult<'a, JournalAST> {
    Ok(("", JournalAST(vec![])))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eol_parses_empty_line() {
        assert_eq!(super::eol("\n"), Ok(("", ())))
    }

    #[test]
    fn eol_parses_empty_line_with_spaces() {
        assert_eq!(super::eol("   \n"), Ok(("", ())))
    }

    #[test]
    fn eol_fails_on_empty_string() {
        assert!(super::eol("").is_err())
    }

    #[test]
    fn eol_fails_on_non_empty_line() {
        assert!(super::eol("  asdasd").is_err());
        assert!(super::eol("2026-01-01").is_err())
    }

    #[test]
    fn parse_eols_parses_empty_string() {
        assert_eq!(super::parse_eols(""), Ok(("", ())));
    }

    #[test]
    fn parse_eols_parses_one_line() {
        assert_eq!(super::parse_eols("  \n"), Ok(("", ())));
    }

    #[test]
    fn parse_eols_parses_two_lines() {
        assert!(super::parse_eols("\n    \n").is_ok());
    }

    #[test]
    fn parse_journal_parses_empty_string() {
        assert_eq!(parse_journal(""), Ok(("", JournalAST(vec![]))))
    }
}
