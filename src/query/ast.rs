use crate::query::operator::Operator;

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



