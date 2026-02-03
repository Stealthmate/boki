use crate::output::{self};

pub mod ast;
mod set_attribute;
mod transaction;

pub fn compile_node(node: &ast::ASTNode, journal: &mut output::Journal) -> Result<(), String> {
    match node {
        ast::ASTNode::Transaction(t) => transaction::TransactionCompiler::compile(t, journal),
        ast::ASTNode::SetAttribute(name, value) => {
            set_attribute::SetAttributeCompiler::compile(name, value, journal)
        }
    }
}

pub fn compile(nodes: Vec<ast::ASTNode>) -> Result<output::Journal, String> {
    let mut journal = output::Journal::default();

    for node in &nodes {
        compile_node(node, &mut journal)?;
    }

    Ok(journal)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_transaction() -> ast::Transaction {
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: Some("JPY".to_string()),
                    amount: Some(1000),
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: Some("JPY".to_string()),
                    amount: Some(-1000),
                },
            ],
        }
    }

    #[test]
    fn test_compile_simple() {
        let ast = vec![ast::ASTNode::Transaction(sample_transaction())];
        let result = compile(ast).expect("Compilation failed.");
    }

    #[test]
    fn test_compile_node_simple_transaction() {
        let node = ast::ASTNode::Transaction(sample_transaction());
        let mut journal = output::Journal::default();
        let result = compile_node(&node, &mut journal).expect("Compilation failed.");

        assert_eq!(journal.transactions.len(), 1);
    }
}
