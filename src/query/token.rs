use crate::query::operator::{Operator,Symbol};
// user Input Decide
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Operator(Operator),
    Symbol(Symbol),
    EOF,
}



