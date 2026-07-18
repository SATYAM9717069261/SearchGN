#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackItem{
    Operator(Operator),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator{
    And,
    Or,
    Not
}
pub enum OperatorType{
    Unary,
    Binary
}


impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Or => 1,
            Operator::And => 2,
            Operator::Not => 3,
        }
    }
    pub fn find_operator_type(&self) -> OperatorType{
        match self {
            Operator::Or => OperatorType::Binary,
            Operator::And => OperatorType::Binary,
            Operator::Not => OperatorType::Unary,
        }
    }
}


#[derive(Debug)]
pub enum ParseError {
    MissingOperand,
    UnmatchedLeftParen,
    UnmatchedRightParen,
    InvalidExpression,
}

