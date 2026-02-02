use boki;

fn compile(fp: &str) -> Result<boki::output::Journal, String> {
    let content = std::fs::read_to_string(fp).expect("Could not open file.");
    boki::compile_string(&content)
}

#[rstest::rstest]
#[case::example("001-example")]
fn test_successful(#[case] case: &str) {
    let result = compile(&format!("tests/cases/{case}.boki")).expect("Failed.");
    let expected_str = std::fs::read_to_string(&format!("tests/cases/{case}.output.json"))
        .expect("Could not read output file.");
    let expected: boki::output::Journal =
        serde_json::from_str(&expected_str).expect("Could not parse output file.");
    assert_eq!(result, expected);
}
