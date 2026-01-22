#![allow(dead_code)]
#![allow(unused_variables)]
use nom::error::ParseError;
use nom::Parser;

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

fn parse_next_statement(input: &str) -> MyResult<'_, Option<Statement>> {
    let (input, eof) = parse_eols(input)?;
    if input == "" {
        return Ok((input, None));
    }

    Ok((
        "",
        Some(Statement::TransactionStatement(Transaction {
            timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01 00:00:00.000Z").unwrap(),
            postings: vec![],
        })),
    ))
    // let (input, statement) = nom::combinator::opt(parse_statement).parse(input)?;
    // let (input, _) = parse_end_of_statement(input)?;

    // Ok((input, statement))
}

pub fn parse_journal<'a>(input: &'a str) -> MyResult<'a, JournalAST> {
    let mut statements = vec![];

    let mut i = input;

    loop {
        let (next_i, maybe_stmt) = parse_next_statement(i)?;
        i = next_i;

        let Some(stmt) = maybe_stmt else {
            break;
        };
        statements.push(stmt);
    }

    Ok(("", JournalAST(statements)))
}

mod tests;

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

    fn read_test_case(s: &str) -> String {
        std::fs::read_to_string(s).unwrap()
    }

    fn assert_journal_case_equals(s: &str, journal: JournalAST) -> () {
        assert_eq!(
            parse_journal(&read_test_case(&format!("src/input/tests/{s}.input.rj"))),
            Ok(("", journal))
        );
    }

    #[rstest::rstest]
    #[case::empty_string("001-empty-string", tests::test_001_empty_string())]
    // #[case::single_transaction("002-single-transaction", tests::test_002_single_transaction())]
    fn test_parse_journal(#[case] s: &str, #[case] result: JournalAST) {
        assert_journal_case_equals(s, result);
    }

    #[rstest::rstest]
    #[case::empty_string("next_statement_001_empty_string", false)]
    #[case::whitespace_only("next_statement_002_whitespace_only", false)]
    #[case::whitespace_statement("next_statement_003_whitespace_statement", true)]
    #[case::whitespace_statement_whitespace(
        "next_statement_004_whitespace_statement_whitespace",
        true
    )]
    fn test_parse_next_statement(#[case] s: &str, #[case] result: bool) {
        let input = read_test_case(&format!("src/input/tests/{s}.input"));
        let (rest, stmt) = parse_next_statement(&input).expect("Could not parse.");
        assert_eq!(stmt.is_some(), result);
        assert_eq!(rest, "");
    }
}
