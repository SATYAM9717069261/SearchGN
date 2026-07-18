use crate::query::operator::Operator;
use crate::query::operator::OperatorType::{Unary,Binary};

#[derive(Debug, Clone)]
pub enum AstNode{
    Word(String),
    Unary{
        operator: Operator,
        child:Box<AstNode>
    },
    Binary{
        operator: Operator,
        left: Box<AstNode>,
        right: Box<AstNode>
    },
}



