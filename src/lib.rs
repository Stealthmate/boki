mod input;
mod output;

// pub use output::{read_object, Object};

// fn run(_: input::JournalAST) -> Object {
//     use output::{Object, Posting, Transaction};
//     Object {
//         transactions: vec![Transaction {
//             timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000Z").unwrap(),
//             postings: vec![
//                 Posting {
//                     account: "assets/cce/cash".to_string(),
//                     commodity: "JPY".to_string(),
//                     amount: -1000,
//                 },
//                 Posting {
//                     account: "expense".to_string(),
//                     commodity: "JPY".to_string(),
//                     amount: 1000,
//                 },
//             ],
//         }],
//     }
// }

// pub fn compile_journal(path: &str) -> Object {
//     let content = std::fs::read_to_string(path).unwrap();
//     let (_, ast) = input::parse_journal(&content).unwrap_or_else(|e| {
//         let errstr = match e {
//             nom::Err::Error(e1) => nom_language::error::convert_error(content.as_str(), e1),
//             nom::Err::Incomplete(e1) => panic!("Incomplete. {e1:#?}"),
//             nom::Err::Failure(e1) => nom_language::error::convert_error(content.as_str(), e1),
//         };
//         panic!("Could not parse\n{errstr}");
//     });

//     run(ast)
// }
