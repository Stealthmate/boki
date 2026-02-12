use boki;

#[rstest::rstest]
#[case::books_journal("docs/examples/01-books-journal/books.boki")]
fn test_docs(#[case] case: &str) {
    boki::evaluate::evaluate_file(case).unwrap_or_else(|e| panic!("Failed:\n{}", e));
}
