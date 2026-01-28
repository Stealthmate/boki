use super::ast;

use crate::output;

pub fn compile_node(node: &ast::ASTNode, journal: &mut output::Object) -> Result<(), String> {
    Ok(())
}

pub fn compile(nodes: Vec<ast::ASTNode>) -> Result<output::Object, String> {
    let mut journal = output::Object {
        transactions: vec![],
    };

    for node in &nodes {
        compile_node(node, &mut journal)?;
    }

    Ok(journal)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let ast = vec![ast::ASTNode::Transaction(ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000+09:00")
                    .unwrap(),
            },
            postings: vec![],
        })];

        let result = compile(ast).expect("Compilation failed.");
    }
}
