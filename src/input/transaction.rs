use super::common::{eol, parse_eols};
use super::common::{InputParser, ParserResult};
use super::posting;
use super::timestamp;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct TransactionHeader {
    pub timestamp: timestamp::Timestamp,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub postings: Vec<posting::Posting>,
}

pub struct TransactionParser;

impl TransactionParser {
    fn parse_column(input: &str) -> ParserResult<'_, String> {
        let (input, col) = take_until(";").parse(input)?;
        let (input, _) = take(1usize).parse(input)?;
        Ok((input, col.to_string()))
    }

    fn parse_last_column(input: &str) -> ParserResult<'_, String> {
        let (input, col) = alt((terminated(take_until("\n"), tag("\n")), rest)).parse(input)?;
        Ok((input, col.to_string()))
    }

    fn empty_str_is_none((i, r): (&str, String)) -> (&str, Option<String>) {
        match r.as_str() {
            "" => (i, None),
            x => (i, Some(x.to_string())),
        }
    }

    fn parse_amount((i, r): (&str, String)) -> ParserResult<'_, Option<i64>> {
        match r.as_str() {
            "" => Ok((i, None)),
            x => Ok((i, Some(1000))),
        }
    }
}

fn parse_transaction_header(input: &str) -> ParserResult<'_, TransactionHeader> {
    let (input, timestamp) = timestamp::TimestampParser::parse(input)?;
    let (input, _) = eol(input)?;

    Ok((input, TransactionHeader { timestamp }))
}

fn peek_next_line(input: &str) -> ParserResult<'_, String> {
    alt((terminated(take_until("\n"), tag("\n")), rest))
        .parse(input)
        .map(|(i, r)| (i, r.to_string()))
}

fn parse_next_posting(input: &str) -> ParserResult<'_, Option<posting::Posting>> {
    let (input, _) = parse_eols(input)?;
    if input.is_empty() {
        return Ok((input, None));
    }

    let (input, posting) = posting::PostingParser::parse(input)?;
    Ok((input, Some(posting)))
}

impl InputParser<Transaction> for TransactionParser {
    fn parse(input: &str) -> ParserResult<'_, Transaction> {
        let (input, header) = parse_transaction_header(input)?;

        let mut postings = vec![];
        let mut i = input;

        loop {
            let (_, next_line) = peek_next_line(i)?;
            println!("Next line: {next_line}");
            if !next_line.starts_with("  ") {
                break;
            }

            let (next_i, maybe_posting) = parse_next_posting(i)?;
            i = next_i;

            let Some(post) = maybe_posting else {
                break;
            };
            postings.push(post);
        }
        let tx = Transaction { header, postings };

        Ok((i, tx))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_transaction_simple() {
        let input = indoc! {"
            2025-01-01
              asset/cce/cash;JPY;-1000
              expense;JPY;1000
        "};

        let (_, result) = TransactionParser::parse(input).expect("Could not parse.");
    }

    #[test]
    fn test_parse_transaction_stops_on_new_transaction() {
        let input = indoc! {"
            2025-01-01
              asset/cce/cash;JPY;-1000
              expense;JPY;1000
            2025-01-02
        "};

        let (rest, result) = TransactionParser::parse(input).expect("Could not parse.");
        assert!(rest.starts_with("2025-01-02"), "Rest: {rest}");
    }
}
