use crate::tokens;

#[derive(Clone, Debug)]
pub struct Transaction {}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Node {
    Misc(Vec<tokens::Token>),
    Transaction(Transaction),
}
