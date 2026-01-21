#[rstest::rstest]
#[case::example("001-example")]
fn test_successful_compilation(#[case] case: &str) {
    let result = hledger_clone::compile_journal(&format!("cases/{case}.input.rj"));
    let expected = hledger_clone::read_object(&format!("tests/cases/{case}.output.json"));
    assert_eq!(result, expected);
}
