use super::_ast;
use boki::{
    parsing::{self, Parser},
    tokens,
};

fn non_newline_token(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<tokens::Token> {
    let t = parsing::peek_next(scanner)?.clone();
    if matches!(t, tokens::Token::LineSeparator) {
        return Err(parsing::ParserError {
            location: scanner.tell(),
            details: parsing::ParserErrorDetails::Other("Expected non-newline token.".to_string()),
        });
    }
    scanner.advance(1)?;
    Ok(t)
}

fn parse_line(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<Vec<tokens::Token>> {
    let mut tokens = parsing::terminated(
        parsing::many(non_newline_token),
        parsing::parse_line_separator,
    )
    .parse(scanner)?;
    tokens.push(tokens::Token::LineSeparator);
    Ok(tokens)
}

fn parse_yaml(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    let start_of_line = scanner.tell();
    parsing::parse_indent(scanner)?;
    if !matches!(
        parsing::peek_next(scanner),
        Ok(tokens::Token::YamlMatter(_))
    ) {
        return Err(parsing::ParserError {
            location: scanner.tell(),
            details: parsing::ParserErrorDetails::Other("Expected YAML matter.".to_string()),
        });
    }
    scanner.seek(start_of_line)?;
    parse_line(scanner).map(_ast::Node::Misc)
}

fn parse_misc(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    let location = scanner.tell();
    if parsing::parse_indent(scanner).is_ok() {
        return Err(parsing::ParserError {
            location,
            details: parsing::ParserErrorDetails::Other("Not a misc line.".to_string()),
        });
    }
    scanner.seek(location)?;
    parse_line(scanner).map(_ast::Node::Misc)
}

fn parse_account(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<Vec<tokens::Token>> {
    let mut parts = vec![];
    loop {
        let t = parsing::get_next(scanner)?;
        match &t {
            tokens::Token::Identifier(_) => parts.push(t.clone()),
            tokens::Token::AccountSeparator => parts.push(t.clone()),
            tokens::Token::Whitespace => parts.push(t.clone()),
            tokens::Token::PostingSeparator => {
                break;
            }
            _ => {
                return Err(parsing::ParserError {
                    location: scanner.tell(),
                    details: parsing::ParserErrorDetails::Other("Not an account.".to_string()),
                });
            }
        };
    }

    Ok(parts)
}

fn parse_posting(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    parsing::parse_indent(scanner)?;

    let account = parse_account(scanner)?;

    parsing::optional(parsing::parse_whitespace).parse(scanner)?;
    let commodity = parsing::optional(parsing::parse_identifier).parse(scanner)?;
    parsing::optional(parsing::parse_whitespace).parse(scanner)?;
    parsing::parse_posting_separator(scanner)?;

    parsing::optional(parsing::parse_whitespace).parse(scanner)?;
    let amount = parsing::optional(parsing::parse_amount).parse(scanner)?;

    parsing::optional(parsing::parse_whitespace).parse(scanner)?;
    let comment = parsing::optional(parsing::parse_comment).parse(scanner)?;

    parsing::parse_line_separator(scanner)?;

    Ok(_ast::Node::Posting(Box::new(_ast::Posting {
        account,
        commodity,
        amount,
        comment,
    })))
}

fn parse_node(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    let parsers = [parse_yaml, parse_posting, parse_misc];
    let node = parsing::one_of(&parsers).parse(scanner)?;
    Ok(node)
}

pub(super) fn parse(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<Vec<_ast::Node>> {
    let mut nodes = vec![];
    loop {
        if let Some(tokens::Token::Eof) = scanner.peek() {
            break;
        }
        let node = parse_node(scanner)?;
        nodes.push(node);
    }
    Ok(nodes)
}
