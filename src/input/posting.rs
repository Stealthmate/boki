use super::common::{InputParser, ParserResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::Parser;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    pub account: String,
    pub commodity: Option<String>,
    pub amount: Option<i64>,
}

pub struct PostingParser;

impl PostingParser {
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

impl InputParser<Posting> for PostingParser {
    fn parse(input: &str) -> ParserResult<'_, Posting> {
        let (input, _) = tag("  ")(input)?;
        let (input, account) = Self::parse_column(input)?;
        let (input, commodity) = Self::parse_column(input).map(Self::empty_str_is_none)?;
        let (input, amount) = Self::parse_last_column(input).and_then(Self::parse_amount)?;

        Ok((
            input,
            Posting {
                account,
                commodity,
                amount,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_posting_parser_succeeds_account_currency_amount() {
        let input = "  asset/cce/cash;JPY;1000";
        let (_, result) = PostingParser::parse(&input).expect("Could not parse.");
        assert_eq!(
            result,
            Posting {
                account: "asset/cce/cash".to_string(),
                commodity: Some("JPY".to_string()),
                amount: Some(1000)
            }
        )
    }

    #[test]
    fn test_posting_parser_succeeds_account_amount() {
        let input = "  asset/cce/cash;;1000";
        let (_, result) = PostingParser::parse(&input).expect("Could not parse.");
        assert_eq!(
            result,
            Posting {
                account: "asset/cce/cash".to_string(),
                commodity: None,
                amount: Some(1000)
            }
        )
    }

    #[test]
    fn test_posting_parser_succeeds_account_commodity() {
        let input = "  asset/cce/cash;JPY;";
        let (_, result) = PostingParser::parse(&input).expect("Could not parse.");
        assert_eq!(
            result,
            Posting {
                account: "asset/cce/cash".to_string(),
                commodity: Some("JPY".to_string()),
                amount: None
            }
        )
    }

    #[test]
    fn test_posting_parser_stops_after_newline() {
        let input = "  asset/cce/cash;JPY;\nfoobar";
        let (rest, result) = PostingParser::parse(&input).expect("Could not parse.");
        assert_eq!(rest, "foobar")
    }

    #[test]
    fn test_posting_parser_rejects_eof() {
        let input = "";
        PostingParser::parse(&input).expect_err("Should have failed.");
    }

    #[test]
    fn test_posting_parser_rejects_empty_line() {
        let input = "       \n";
        PostingParser::parse(&input).expect_err("Should have failed.");
    }
}
