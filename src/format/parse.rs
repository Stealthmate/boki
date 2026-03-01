use crate::{format::_ast, parsing};

pub(super) fn parse(scanner: &mut parsing::TokenScanner) -> parsing::ParserResult<Vec<_ast::Node>> {
    let nodes = vec![_ast::Node::Misc(scanner.tokens().to_vec())];
    Ok(nodes)
}
