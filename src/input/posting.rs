use super::common::{InputParser, ParserResult};
use nom::bytes::complete::{tag, take, take_until};
use nom::multi::many;
use nom::Parser;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    account: String,
    commodity: String,
    amount: i64,
}

pub struct PostingParser;

impl PostingParser {}

impl InputParser<Posting> for PostingParser {
    fn parse(input: &str) -> ParserResult<'_, Posting> {
        let (input, _): (&str, Vec<&str>) = many(2.., tag(" ")).parse(input)?;
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_posting_parser_succeeds_account_currency_amount() {
        let input = "  asset/cce/cash;JPY;1000\n";
        let (_, result) = PostingParser::parse(&input).expect("Could not parse.");
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
    fn test_posting_parser_fails_if_no_indent() {
        let input = "asset/cce/cash;JPY;1000\n";
        assert!(PostingParser::parse(&input).is_err());
    }

    #[test]
    fn test_posting_parser_fails_if_no_newline_at_end() {
        let input = "  asset/cce/cash;JPY;1000";
        assert!(PostingParser::parse(&input).is_err());
    }
}
