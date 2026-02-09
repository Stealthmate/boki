use crate::input::contracts::tokens;

use crate::input::parse_v2::core::{
    get_next, ParserError, ParserErrorDetails, ParserResult, TokenScanner,
};

macro_rules! parse_token {
    ($name:ident, $return_type:ty, $tname:expr, $expansion:pat, $return_value:expr) => {
        pub fn $name(scanner: &mut TokenScanner) -> ParserResult<$return_type> {
            let i = scanner.tell();
            let t = get_next(scanner)?;
            match t {
                $expansion => Ok($return_value),
                _ => {
                    return Err(ParserError {
                        location: i,
                        details: ParserErrorDetails::ExpectedSomethingElse(
                            $tname.to_string(),
                            t.clone(),
                        ),
                    })
                }
            }
        }
    };
}

parse_token!(
    parse_timestamp,
    tokens::Timestamp,
    tokens::TOKEN_NAME_TIMESTAMP,
    tokens::Token::Timestamp(ts),
    *ts
);
parse_token!(
    parse_amount,
    i64,
    tokens::TOKEN_NAME_AMOUNT,
    tokens::Token::Amount(x),
    *x
);
parse_token!(
    parse_identifier,
    String,
    tokens::TOKEN_NAME_IDENTIFIER,
    tokens::Token::Identifier(x),
    x.clone()
);
parse_token!(
    parse_account_separator,
    (),
    tokens::TOKEN_NAME_ACCOUNT_SEPARATOR,
    tokens::Token::AccountSeparator,
    ()
);
parse_token!(
    parse_posting_separator,
    (),
    tokens::TOKEN_NAME_POSTING_SEPARATOR,
    tokens::Token::PostingSeparator,
    ()
);
parse_token!(
    parse_line_separator,
    (),
    tokens::TOKEN_NAME_LINE_SEPARATOR,
    tokens::Token::LineSeparator,
    ()
);
parse_token!(
    parse_yaml_matter,
    serde_yaml::Mapping,
    tokens::TOKEN_NAME_YAML_MATTER,
    tokens::Token::YamlMatter(x),
    x.clone()
);
parse_token!(
    parse_indent,
    (),
    tokens::TOKEN_NAME_INDENT,
    tokens::Token::Indent,
    ()
);

pub fn parse_keyword(scanner: &mut TokenScanner, kw: tokens::Keyword) -> ParserResult<()> {
    let i = scanner.tell();
    let t = get_next(scanner)?;
    match t {
        tokens::Token::Keyword(x) if *x == kw => Ok(()),
        _ => Err(ParserError {
            location: i,
            details: ParserErrorDetails::ExpectedSomethingElse(
                tokens::TOKEN_NAME_KEYWORD.to_string(),
                t.clone(),
            ),
        }),
    }
}
