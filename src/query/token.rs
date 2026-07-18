use crate::query::Operator;
// user Input Decide
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Operator(Operator),
    LeftParen,
    RightParen,
    EOF,
}


