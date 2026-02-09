//! The compiler stage.
//!
//! This module is responsible for processing a list of AST nodes
//! and producing an output Journal.
//!
//! The compiler depends only on the AST, and thus does not assume _anything_
//! with regards to the way the AST gets produced. In particular, the compiler does not care
//! if parsing occurs in a streaming or complete fashion, nor does it consider
//! how to handle errors which happen while generating the AST.

use crate::input::contracts::ast;
use crate::output::{self};

mod set_attribute;
mod transaction;

#[derive(Debug)]
pub enum CompilationError {
    GeneralError(String),
}

impl CompilationError {
    pub fn from_str(s: &str) -> Self {
        CompilationError::GeneralError(s.to_string())
    }
}

pub type CompilationResult<T> = Result<T, CompilationError>;

pub fn compile_node(node: &ast::ASTNode, journal: &mut output::Journal) -> CompilationResult<()> {
    match node {
        ast::ASTNode::Transaction(t) => transaction::TransactionCompiler::compile(t, journal),
        ast::ASTNode::SetAttribute(name, value) => {
            set_attribute::SetAttributeCompiler::compile(name, value, journal)
        }
    }
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
        let node = ast::ASTNode::Transaction(sample_transaction());
        let mut journal = output::Journal::default();
        let result = compile_node(&node, &mut journal).expect("Compilation failed.");
    }

    #[test]
    fn test_compile_node_simple_transaction() {
        let node = ast::ASTNode::Transaction(sample_transaction());
        let mut journal = output::Journal::default();
        let result = compile_node(&node, &mut journal).expect("Compilation failed.");

        assert_eq!(journal.transactions.len(), 1);
    }
}
