use crate::input::contracts::ast;
use crate::input::contracts::tokens::Token;
use crate::input::parse::core;
use crate::input::parse::core::Parser;

pub struct TransactionParser;

impl TransactionParser {
    fn parse_transaction_attributes(
        tokens: &[Token],
    ) -> core::ParserResult<'_, serde_yaml::Mapping> {
        core::terminated(core::parse_yaml_matter, core::parse_line_separator).parse(tokens)
    }

    fn parse_header(tokens: &[Token]) -> core::ParserResult<'_, ast::TransactionHeader> {
        let (tokens, timestamp) = core::parse_timestamp(tokens)?;
        let (tokens, _) = core::parse_line_separator(tokens)?;
        let (tokens, attributes) = core::optional(Self::parse_transaction_attributes)
            .parse(tokens)
            .map(|(i, x)| (i, x.unwrap_or_default()))?;
        Ok((
            tokens,
            ast::TransactionHeader {
                timestamp,
                attributes,
            },
        ))
    }

    fn parse_subaccount(tokens: &[Token]) -> core::ParserResult<'_, String> {
        let (tokens, _) = core::parse_account_separator(tokens)?;
        let (tokens, acc) = core::parse_identifier(tokens)?;

        Ok((tokens, acc))
    }

    fn parse_account(tokens: &[Token]) -> core::ParserResult<'_, String> {
        let (tokens, root) = core::parse_identifier(tokens)?;
        let (tokens, rest) = core::many(Self::parse_subaccount).parse(tokens)?;

        let acc = rest.into_iter().fold(root, |a, p| a + "/" + &p);

        Ok((tokens, acc))
    }

    fn parse_commodity(tokens: &[Token]) -> core::ParserResult<'_, String> {
        core::parse_identifier(tokens)
    }

    fn parse_posting(tokens: &[Token]) -> core::ParserResult<'_, ast::Posting> {
        if tokens.is_empty() {
            return Err("No more tokens.".to_string());
        }

        let (tokens, account) = Self::parse_account(tokens)?;
        let (tokens, _) = core::parse_posting_separator(tokens)?;
        let (tokens, commodity) = core::optional(Self::parse_commodity).parse(tokens)?;
        let (tokens, _) = core::parse_posting_separator(tokens)?;
        let (tokens, amount) = core::optional(core::parse_amount).parse(tokens)?;
        let (tokens, _) = core::parse_line_separator(tokens)?;

        Ok((
            tokens,
            ast::Posting {
                account,
                commodity,
                amount,
            },
        ))
    }

    pub fn parse(tokens: &[Token]) -> core::ParserResult<'_, ast::Transaction> {
        let (tokens, header) = Self::parse_header(tokens)?;
        let (tokens, postings) =
            core::many(core::preceded(core::parse_indent, Self::parse_posting)).parse(tokens)?;

        Ok((tokens, ast::Transaction { header, postings }))
    }
}

#[cfg(test)]
mod test {
    use super::TransactionParser;
    use crate::input::contracts::tokens::{Timestamp, Token};

    fn sample_timestamp() -> Timestamp {
        Timestamp::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00").unwrap()
    }

    #[test]
    fn test_header_simple() {
        let ts = sample_timestamp();
        let tokens = [Token::Timestamp(ts), Token::LineSeparator];
        let (rest, result) = TransactionParser::parse_header(&tokens).expect("Failed.");
        assert_eq!(result.timestamp, sample_timestamp());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_header_attributes() {
        let ts = sample_timestamp();
        let tokens = [
            Token::Timestamp(ts),
            Token::LineSeparator,
            Token::YamlMatter(serde_yaml::Mapping::default()),
            Token::LineSeparator,
        ];
        let (rest, result) = TransactionParser::parse_header(&tokens).expect("Failed.");
        assert_eq!(result.timestamp, sample_timestamp());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_posting_simple() {
        let tokens = [
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
        ];
        let (rest, result) = TransactionParser::parse_posting(&tokens).expect("Failed.");
        assert_eq!(result.account, "asset/cce/cash".to_string());
        assert_eq!(result.commodity, Some("JPY".to_string()));
        assert_eq!(result.amount, Some(1000));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_posting_omitted_commodity() {
        let tokens = [
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::PostingSeparator,
            Token::Amount(1000),
            Token::LineSeparator,
        ];
        let (rest, result) = TransactionParser::parse_posting(&tokens).expect("Failed.");
        assert_eq!(result.commodity, None);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_posting_omitted_amount() {
        let tokens = [
            Token::Identifier("asset".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cce".to_string()),
            Token::AccountSeparator,
            Token::Identifier("cash".to_string()),
            Token::PostingSeparator,
            Token::Identifier("JPY".to_string()),
            Token::PostingSeparator,
            Token::LineSeparator,
        ];
        let (rest, result) = TransactionParser::parse_posting(&tokens).expect("Failed.");
        assert_eq!(result.amount, None);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_simple() {
        let ts = sample_timestamp();
        let tokens = [
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
        ];
        let (rest, result) = TransactionParser::parse(&tokens).expect("Failed.");
        assert!(rest.is_empty());
    }
}
