#![allow(dead_code)]
#![allow(unused_variables)]

mod compile;
mod parse;
// mod lex;
// mod block;
// mod common;
// mod posting;
// mod statement;
// mod timestamp;
// mod transaction;

// use common::{InputParser, ParserResult};

// #[derive(Debug, PartialEq)]
// pub struct Object {
//     transactions: Vec<transaction::Transaction>,
// }

// #[derive(Debug, PartialEq)]
// pub struct JournalAST(Vec<statement::Statement>);

// fn parse_next_statement(input: &str) -> ParserResult<'_, Option<statement::Statement>> {
//     let (input, eof) = common::parse_eols(input)?;
//     if input.is_empty() {
//         return Ok((input, None));
//     }

//     let (input, stmt) = statement::StatementParser::parse(input)?;
//     Ok((input, Some(stmt)))
// }

// pub fn parse_journal<'a>(input: &'a str) -> ParserResult<'a, JournalAST> {
//     let mut statements = vec![];

//     let mut i = input;

//     loop {
//         let (next_i, maybe_stmt) = parse_next_statement(i)?;
//         i = next_i;

//         let Some(stmt) = maybe_stmt else {
//             break;
//         };
//         statements.push(stmt);
//     }

//     Ok(("", JournalAST(statements)))
// }

// mod tests;

// #[cfg(test)]
// mod test {

//     use super::*;

//     fn read_test_case(s: &str) -> String {
//         std::fs::read_to_string(s).unwrap()
//     }

//     fn assert_journal_case_equals(s: &str, journal: JournalAST) -> () {
//         assert_eq!(
//             parse_journal(&read_test_case(&format!("src/input/tests/{s}.input.rj"))),
//             Ok(("", journal))
//         );
//     }

//     #[rstest::rstest]
//     #[case::empty_string("001-empty-string", tests::test_001_empty_string())]
//     // #[case::single_transaction("002-single-transaction", tests::test_002_single_transaction())]
//     fn test_parse_journal(#[case] s: &str, #[case] result: JournalAST) {
//         assert_journal_case_equals(s, result);
//     }

//     #[rstest::rstest]
//     #[case::empty_string("next_statement_001_empty_string", false)]
//     #[case::whitespace_only("next_statement_002_whitespace_only", false)]
//     #[case::whitespace_statement("next_statement_003_whitespace_statement", true)]
//     #[case::whitespace_statement_whitespace(
//         "next_statement_004_whitespace_statement_whitespace",
//         true
//     )]
//     fn test_parse_next_statement(#[case] s: &str, #[case] result: bool) {
//         let input = read_test_case(&format!("src/input/tests/{s}.input"));
//         let (rest, stmt) = parse_next_statement(&input).expect("Could not parse.");
//         assert_eq!(stmt.is_some(), result);
//         assert_eq!(rest, "");
//     }
// }
