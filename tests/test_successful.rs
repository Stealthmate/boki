// #[rstest::rstest]
// #[case::example("001-example")]
// // #[case::multiple_transactions("002-multiple-transactions")]
// fn test_successful_compilation(#[case] case: &str) {
//     let result = hledger_clone::compile_journal(&format!("tests/cases/{case}.input.rj"));
//     let expected = hledger_clone::read_object(&format!("tests/cases/{case}.output.json"));
//     assert_eq!(result, expected);
// }
