use super::ast;

use crate::output::{self, Journal};

pub fn compile_transaction(
    t: &ast::Transaction,
    journal: &mut output::Journal,
) -> Result<(), String> {
    if t.postings.len() < 2 {
        return Err("Must have 2 or more postings.".to_string());
    }

    journal.transactions.push(output::Transaction {
        header: output::TransactionHeader {
            timestamp: t.header.timestamp,
        },
        postings: t
            .postings
            .iter()
            .map(|p| output::Posting {
                account: p.account.clone(),
                commodity: p.commodity.clone().unwrap(),
                amount: p.amount.unwrap(),
            })
            .collect(),
    });

    Ok(())
}

pub fn compile_node(node: &ast::ASTNode, journal: &mut output::Journal) -> Result<(), String> {
    match node {
        ast::ASTNode::Transaction(t) => compile_transaction(t, journal),
    }
}

pub fn compile(nodes: Vec<ast::ASTNode>) -> Result<output::Journal, String> {
    let mut journal = Journal::default();

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
        let mut journal = Journal::default();
        let result = compile_node(&node, &mut journal).expect("Compilation failed.");

        assert_eq!(journal.transactions.len(), 1);
    }

    #[test]
    fn test_compile_transaction_all_literals() {
        let t = sample_transaction();
        let mut journal = Journal::default();
        let result = compile_transaction(&t, &mut journal).expect("Failed.");

        let j_t = journal.transactions.first().expect("Failed.");
        assert_eq!(j_t.header.timestamp, t.header.timestamp);
        assert_eq!(j_t.postings.len(), t.postings.len());
        for (p_out, p_in) in j_t.postings.iter().zip(t.postings.iter()) {
            assert_eq!(p_out.account, p_in.account);
            assert_eq!(p_out.commodity, p_in.commodity.clone().unwrap());
            assert_eq!(p_out.amount, p_in.amount.clone().unwrap());
        }
    }

    #[rstest::rstest]
    #[case::with_0_postings(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
            },
            postings: vec![],
        }
    )]
    #[case::with_1_posting(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: None,
                    amount: None
                }
            ],
        }
    )]
    fn test_compile_transaction_rejects(#[case] t: ast::Transaction) {
        let mut journal = Journal::default();

        compile_transaction(&t, &mut journal).expect_err("Should have failed.");
    }
}
