use crate::input::compile::ast;
use crate::output;
use std::collections::HashMap;

pub struct TransactionCompiler;

impl TransactionCompiler {
    fn compute_balances(postings: &[output::Posting]) -> HashMap<String, i64> {
        let mut m = HashMap::new();

        for p in postings.iter() {
            let commodity = &p.commodity;
            let balance = m.get(commodity).cloned().unwrap_or(0);

            m.insert(commodity.clone(), balance + p.amount);
        }

        m
    }

    fn validate_postings(
        postings: &[ast::Posting],
        journal: &output::Journal,
    ) -> ast::CompilationResult<(Vec<output::Posting>, Option<usize>)> {
        use std::iter::repeat;

        let n_postings = postings.len();
        if n_postings < 2 {
            return Err(ast::CompilationError::from_str(
                "Must have 2 or more postings.",
            ));
        }

        let mut out_postings: Vec<output::Posting> = repeat(output::Posting::default())
            .take(n_postings)
            .collect();

        let mut i_empty_amount = None;
        for (i, (p_out, p_in)) in out_postings.iter_mut().zip(postings).enumerate() {
            p_out.account = p_in.account.clone();
            p_out.commodity = p_in
                .commodity
                .clone()
                .unwrap_or(journal.header.default_commodity.clone());
            p_out.amount = p_in.amount.unwrap_or(0);
            if p_in.amount.is_none() {
                if i_empty_amount.is_some() {
                    return Err(ast::CompilationError::from_str(
                        "Only a single posting can have an empty amount.",
                    ));
                }
                i_empty_amount = Some(i);
            }
        }

        Ok((out_postings, i_empty_amount))
    }

    fn find_unbalanced_commodities(postings: &[output::Posting]) -> Vec<(String, i64)> {
        Self::compute_balances(postings)
            .iter()
            .filter_map(|(k, v)| match v {
                0 => None,
                _ => Some((k.clone(), *v)),
            })
            .collect()
    }

    fn ensure_transaction_is_balanced(t: &output::Transaction) -> ast::CompilationResult<()> {
        if Self::compute_balances(&t.postings)
            .iter()
            .any(|(_, a)| *a != 0)
        {
            return Err(ast::CompilationError::from_str("Unbalanced transaction."));
        }

        Ok(())
    }

    pub fn compile(
        t: &ast::Transaction,
        journal: &mut output::Journal,
    ) -> ast::CompilationResult<()> {
        let n_postings = t.postings.len();
        if n_postings < 2 {
            return Err(ast::CompilationError::from_str(
                "Must have 2 or more postings.",
            ));
        }

        let (mut postings, i_empty_amount) = Self::validate_postings(&t.postings, journal)?;
        let unbalanced_commodities = Self::find_unbalanced_commodities(&postings);
        if unbalanced_commodities.len() > 1 {
            return Err(ast::CompilationError::from_str(
                "Only a single commodity can be unbalanced.",
            ));
        }

        let unbalanced_commodity = unbalanced_commodities.first().cloned();

        if let Some(i) = i_empty_amount {
            let posting = &mut postings[i];
            let (commodity, amount) =
                unbalanced_commodity.unwrap_or((posting.commodity.clone(), 0));
            if posting.commodity != commodity {
                return Err(ast::CompilationError::from_str(
                    "Empty posting commodity is different than unbalanced commodity.",
                ));
            }

            posting.amount = -amount;
        }

        let out_t = output::Transaction {
            header: output::TransactionHeader {
                timestamp: t.header.timestamp,
                attributes: t.header.attributes.clone(),
            },
            postings,
        };

        Self::ensure_transaction_is_balanced(&out_t)?;

        journal.transactions.push(out_t);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::ast;
    use crate::output;

    fn compile_transaction(
        t: &ast::Transaction,
        journal: &mut output::Journal,
    ) -> ast::CompilationResult<()> {
        super::TransactionCompiler::compile(&t, journal)
    }

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
    fn test_simple() {
        let t = sample_transaction();
        let mut journal = output::Journal::default();
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

    #[test]
    fn test_substitutes_empty_commodity_for_default() {
        let mut t = sample_transaction();
        t.postings[0].commodity = None;
        t.postings[1].commodity = Some("JPY".to_string());

        let mut journal = output::Journal::default();

        journal.header.default_commodity = "JPY".to_string();

        let result = compile_transaction(&t, &mut journal).expect("Failed.");

        let j_t = journal.transactions.first().expect("Failed.");
        assert_eq!(j_t.postings[0].commodity, "JPY".to_string());
    }

    #[test]
    fn test_auto_balances_single_missing_amount() {
        let mut t = sample_transaction();
        t.postings[0].amount = None;

        let mut journal = output::Journal::default();

        let result = compile_transaction(&t, &mut journal).expect("Failed.");

        let j_t = journal.transactions.first().expect("Failed.");
        assert_eq!(j_t.postings[0].amount, 1000);
    }

    #[test]
    fn test_attributes_are_copied() {
        let mut t = sample_transaction();
        t.header.attributes = serde_yaml::Mapping::default();
        t.header.attributes.insert("foo".into(), "bar".into());

        let mut journal = output::Journal::default();

        let result = compile_transaction(&t, &mut journal).expect("Failed.");

        let j_t = journal.transactions.first().expect("Failed.");
        let attrs = &j_t.header.attributes;
        assert!(attrs.contains_key("foo"));
        assert_eq!(attrs.get("foo").expect("Failed."), "bar");
    }

    #[rstest::rstest]
    #[case::with_0_postings(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![],
        }
    )]
    #[case::with_1_posting(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
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
    #[case::with_net_negative_amounts(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: None,
                    amount: Some(0)
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: None,
                    amount: Some(-1000)
                }
            ],
        })]
    #[case::with_net_positive_amounts(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: None,
                    amount: Some(0)
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: None,
                    amount: Some(1000)
                }
            ],
        })]
    #[case::with_multiple_empty_amounts(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: None,
                    amount: Some(1000)
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: None,
                    amount: None
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: None,
                    amount: None
                }
            ],
        })]
    #[case::with_unbalanced_commodities(
        ast::Transaction {
            header: ast::TransactionHeader {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05.000+09:00")
                    .unwrap(),
                attributes: serde_yaml::Mapping::default(),
            },
            postings: vec![
                ast::Posting {
                    account: "foo".to_string(),
                    commodity: Some("USD".to_string()),
                    amount: Some(1000)
                },
                ast::Posting {
                    account: "bar".to_string(),
                    commodity: Some("JPY".to_string()),
                    amount: Some(-1000)
                },
            ],
        })]
    fn test_rejects(#[case] t: ast::Transaction) {
        let mut journal = output::Journal::default();

        compile_transaction(&t, &mut journal).expect_err("Should have failed.");
    }
}
