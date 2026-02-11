use crate::input::contracts::ast;
use crate::input::parse_v2::{basic, combinators, core};

use crate::input::parse_v2::core::{Parser, ParserError};

pub struct TransactionParser;

impl TransactionParser {
    fn parse_transaction_attributes(
        scanner: &mut core::TokenScanner,
    ) -> core::ParserResult<serde_yaml::Mapping> {
        combinators::terminated(basic::parse_yaml_matter, basic::parse_line_separator)
            .parse(scanner)
    }

    fn parse_header(
        scanner: &mut core::TokenScanner,
    ) -> core::ParserResult<ast::TransactionHeader> {
        let timestamp = basic::parse_timestamp(scanner)?;
        basic::parse_line_separator(scanner)?;
        let attributes = combinators::optional(Self::parse_transaction_attributes)
            .parse(scanner)
            .map(|x| x.unwrap_or_default())?;
        Ok(ast::TransactionHeader {
            timestamp,
            attributes,
        })
    }

    fn parse_subaccount(scanner: &mut core::TokenScanner) -> core::ParserResult<String> {
        combinators::preceded(basic::parse_account_separator, basic::parse_identifier)
            .parse(scanner)
    }

    fn parse_account(scanner: &mut core::TokenScanner) -> core::ParserResult<String> {
        let root = basic::parse_identifier(scanner)?;
        let rest = combinators::many(Self::parse_subaccount).parse(scanner)?;

        let acc = rest.into_iter().fold(root, |a, p| a + "/" + &p);

        Ok(acc)
    }

    fn parse_commodity(scanner: &mut core::TokenScanner) -> core::ParserResult<String> {
        basic::parse_identifier(scanner)
    }

    fn parse_posting(scanner: &mut core::TokenScanner) -> core::ParserResult<ast::Posting> {
        let account = Self::parse_account(scanner)?;
        basic::parse_posting_separator(scanner)?;
        let commodity = combinators::optional(Self::parse_commodity).parse(scanner)?;
        basic::parse_posting_separator(scanner)?;
        let amount = combinators::optional(basic::parse_amount).parse(scanner)?;
        basic::parse_line_separator(scanner)?;

        Ok(ast::Posting {
            account,
            commodity,
            amount,
        })
    }

    pub fn parse(scanner: &mut core::TokenScanner) -> core::ParserResult<ast::Transaction> {
        let header = Self::parse_header(scanner)?;

        let mut postings = vec![];
        loop {
            let i = scanner.tell();
            if basic::parse_indent(scanner).is_err() {
                scanner.seek(i)?;
                break;
            }
            let p = Self::parse_posting(scanner).map_err(|e| ParserError {
                location: i,
                details: core::ParserErrorDetails::Nested(
                    "Encountered invalid posting".to_string(),
                    Box::new(e),
                ),
            })?;
            postings.push(p);
        }

        Ok(ast::Transaction { header, postings })
    }
}

#[cfg(test)]
mod test {
    use super::TransactionParser;
    use crate::input::contracts::tokens::{Timestamp, Token};
    use crate::input::parse_v2::core::TokenScanner;

    fn sample_timestamp() -> Timestamp {
        Timestamp::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00").unwrap()
    }

    #[test]
    fn test_header_simple() {
        let ts = sample_timestamp();
        let mut scanner = TokenScanner::from_slice(&[Token::Timestamp(ts), Token::LineSeparator]);
        let result = TransactionParser::parse_header(&mut scanner).expect("Failed.");
        assert_eq!(result.timestamp, sample_timestamp());
    }

    #[test]
    fn test_header_attributes() {
        let ts = sample_timestamp();
        let mut scanner = TokenScanner::from_slice(&[
            Token::Timestamp(ts),
            Token::LineSeparator,
            Token::YamlMatter(serde_yaml::Mapping::default()),
            Token::LineSeparator,
        ]);
        let result = TransactionParser::parse_header(&mut scanner).expect("Failed.");
        assert_eq!(result.timestamp, sample_timestamp());
    }

    #[test]
    fn test_posting_simple() {
        let mut scanner = TokenScanner::from_slice(&[
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
        ]);
        let result = TransactionParser::parse_posting(&mut scanner).expect("Failed.");
        assert_eq!(result.account, "asset/cce/cash".to_string());
        assert_eq!(result.commodity, Some("JPY".to_string()));
        assert_eq!(result.amount, Some(1000));
    }

    #[test]
    fn test_posting_omitted_commodity() {
        let mut scanner = TokenScanner::from_slice(&[
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
        ]);
        let result = TransactionParser::parse_posting(&mut scanner).expect("Failed.");
        assert_eq!(result.commodity, None);
    }

    #[test]
    fn test_posting_omitted_amount() {
        let mut scanner = TokenScanner::from_slice(&[
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::LineSeparator,
        ]);
        let result = TransactionParser::parse_posting(&mut scanner).expect("Failed.");
        assert_eq!(result.amount, None);
    }

    #[test]
    fn test_simple() {
        let ts = sample_timestamp();
        let mut scanner = TokenScanner::from_slice(&[
            Token::Timestamp(ts),
            Token::LineSeparator,
            Token::Indent,
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
            Token::Indent,
            Token::Identifier("expense".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
        ]);
        let result = TransactionParser::parse(&mut scanner).expect("Failed.");
    }

    #[test]
    fn test_invalid_posting() {
        use crate::input::parse_v2::core::ParserErrorDetails;

        let ts = sample_timestamp();
        let mut scanner = TokenScanner::from_slice(&[
            Token::Timestamp(ts),
            Token::LineSeparator,
            Token::Indent,
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            // Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
            Token::Indent,
            Token::Identifier("expense".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
        ]);
        let err = TransactionParser::parse(&mut scanner).expect_err("Should have failed.");
        assert_eq!(err.location, 2);
        let ParserErrorDetails::Nested(_, nested) = err.details else {
            panic!("Invalid error.");
        };
        assert_eq!(nested.location, 10);
        assert!(matches!(
            nested.details,
            ParserErrorDetails::ExpectedSomethingElse(_1, _2)
        ));
    }
}
