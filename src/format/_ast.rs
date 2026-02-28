use crate::tokens;

#[derive(Clone, Debug)]
pub enum Node {
    Misc(Vec<tokens::Token>),
}
