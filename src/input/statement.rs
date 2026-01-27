use super::common::{InputParser, ParserResult};
use super::transaction;
use nom::branch::alt;

#[derive(Debug, PartialEq)]
pub enum Statement {
    TransactionStatement(transaction::Transaction),
}

pub struct StatementParser {}

impl InputParser<Statement> for StatementParser {
    fn parse(input: &str) -> ParserResult<'_, Statement> {
        use nom::Parser;
        alt([transaction::TransactionParser::parse.map(Statement::TransactionStatement)])
            .parse(input)
    }
}
