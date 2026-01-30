use boki;

fn compile(fp: &str) -> Result<boki::output::Journal, String> {
    let content = std::fs::read_to_string(fp).expect("Could not open file.");
    boki::compile_string(&content)
}

#[rstest::rstest]
#[case::example("001-example")]
fn test_successful(#[case] case: &str) {
    compile(&format!("tests/cases/{case}.boki")).expect("Failed.");
}
