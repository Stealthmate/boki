#[derive(Debug, PartialEq)]
pub struct Object {}

fn compile_journal(path: &str) -> Object {
    Object {}
}

fn read_object(path: &str) -> Object {
    Object {}
}

#[test]
fn test_single() {
    let result = compile_journal("cases/001-example.input.rj");
    assert_eq!(result, read_object("cases/001-example-output.json"));
}
