use crate::{
    format::_ast,
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

fn parse_misc(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    parse_line(scanner).map(_ast::Node::Misc)
}

fn parse_node(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<_ast::Node> {
    let parsers = [parse_misc];
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
