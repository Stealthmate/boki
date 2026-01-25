#![allow(dead_code)]
#![allow(unused_variables)]

use nom::bytes::streaming::tag;
use nom::Parser;
use nom::{
    bytes::complete::{take, take_until},
    error::ParseError,
};

mod common;
mod posting;
mod timestamp;

use common::{InputParser, ParserResult};

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    account: String,
    commodity: String,
    amount: i64,
}

#[derive(Debug, PartialEq)]
pub struct TransactionHeader {
    timestamp: timestamp::Timestamp,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    header: TransactionHeader,
    postings: Vec<Posting>,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq)]
enum Statement {
    TransactionStatement(Transaction),
}

#[derive(Debug, PartialEq)]
pub struct JournalAST(Vec<Statement>);

fn parse_whitespace(input: &str) -> ParserResult<'_, &str> {
    use nom::Parser;
    let (input, _) = nom::multi::many0(nom::bytes::complete::tag(" ")).parse(input)?;
    Ok((input, ""))
}

fn eol(input: &str) -> ParserResult<'_, ()> {
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

fn parse_eols(input: &str) -> ParserResult<'_, ()> {
    use nom::Parser;
    let (next_input, _) = nom::multi::many0(eol).parse(input)?;

    Ok((next_input, ()))
}

fn parse_posting(input: &str) -> ParserResult<'_, Posting> {
    let (input, _): (&str, Vec<&str>) = nom::multi::many(2.., tag(" ")).parse(input)?;
    let (input, _) = take_until("\n").parse(input)?;
    let (input, _) = take(1usize).parse(input)?;

    Ok((
        input,
        Posting {
            account: "asset/cce/cash".to_string(),
            commodity: "JPY".to_string(),
            amount: 1000,
        },
    ))
}

fn parse_timestamp(input: &str) -> ParserResult<'_, timestamp::Timestamp> {
    timestamp::TimestampParser::parse(input)
}

fn parse_transaction_header(input: &str) -> ParserResult<'_, TransactionHeader> {
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = eol(input)?;

    Ok((input, TransactionHeader { timestamp }))
}

fn parse_next_posting(input: &str) -> ParserResult<'_, Option<Posting>> {
    Ok((input, None))
}

fn parse_transaction(input: &str) -> ParserResult<'_, Transaction> {
    let (input, header) = parse_transaction_header(input)?;

    let mut postings = vec![];
    let mut i = input;

    loop {
        let (next_i, maybe_posting) = parse_next_posting(i)?;
        i = next_i;

        let Some(post) = maybe_posting else {
            break;
        };
        postings.push(post);
    }
    let tx = Transaction { header, postings };

    Ok(("", tx))
}

fn parse_statement(input: &str) -> ParserResult<'_, Statement> {
    use nom::Parser;
    nom::branch::alt([parse_transaction.map(Statement::TransactionStatement)]).parse(input)
}

fn parse_end_of_statement(input: &str) -> ParserResult<'_, ()> {
    todo!()
}

fn parse_next_statement(input: &str) -> ParserResult<'_, Option<Statement>> {
    let (input, eof) = parse_eols(input)?;
    if input.is_empty() {
        return Ok((input, None));
    }

    Ok((
        "",
        Some(Statement::TransactionStatement(Transaction {
            header: TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01 00:00:00.000Z")
                    .unwrap(),
            },
            postings: vec![],
        })),
    ))
    // let (input, statement) = nom::combinator::opt(parse_statement).parse(input)?;
    // let (input, _) = parse_end_of_statement(input)?;

    // Ok((input, statement))
}

pub fn parse_journal<'a>(input: &'a str) -> ParserResult<'a, JournalAST> {
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

    #[test]
    fn test_parse_statement_001_transaction() {
        let (input, result) = parse_statement(&read_test_case(&format!(
            "src/input/tests/statement_001_transaction.input"
        )))
        .expect("Could not parse.");

        match result {
            Statement::TransactionStatement(_) => (),
            _ => panic!("Not a transaction."),
        }
    }

    // #[test]
    // fn test_parse_transaction_001_simple() {
    //     let input = read_test_case(&format!("src/input/tests/transaction_001_simple.input"));
    //     let (rest, tn) = parse_transaction(&input).expect("Could not parse.");
    //     assert_eq!(tn.postings.len(), 2);
    // }

    #[test]
    fn test_parse_posting_account_currency_amount() {
        let input = "  asset/cce/cash;JPY;1000\n";
        let (_, result) = parse_posting(&input).expect("Could not parse.");
        assert_eq!(
            result,
            Posting {
                account: "asset/cce/cash".to_string(),
                commodity: "JPY".to_string(),
                amount: 1000
            }
        )
    }

    #[test]
    fn test_parse_posting_fails_if_no_indent() {
        let input = "asset/cce/cash;JPY;1000\n";
        assert!(parse_posting(&input).is_err());
    }

    #[test]
    fn test_parse_posting_fails_if_no_newline_at_end() {
        let input = "  asset/cce/cash;JPY;1000";
        assert!(parse_posting(&input).is_err());
    }
}
