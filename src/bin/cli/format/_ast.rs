use boki::tokens;

#[derive(Clone, Debug)]
pub struct Posting {
    pub account: Vec<tokens::Token>,
    pub commodity: Option<String>,
    pub amount: Option<i64>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Node {
    Misc(Vec<tokens::Token>),
    Posting(Box<Posting>),
}
