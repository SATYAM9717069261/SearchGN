use crate::query::operator::{Operator, OperatorType, StackItem, ParseError};
use crate::query::operator::{Symbol};
use crate::query::token::Token;
use crate::query::ast::AstNode;

use std::collections::VecDeque;

fn reduce( operand_stk: &mut VecDeque<AstNode>, op: Operator,) -> Result<(), ParseError> {
    match op.find_operator_type() {
        OperatorType::Unary => {
            let right = operand_stk .pop_back().ok_or(ParseError::MissingOperand)?;
            operand_stk.push_back(AstNode::Unary {
                operator: op,
                child: Box::new(right),
            });
        }
        OperatorType::Binary => {
            let right = operand_stk.pop_back().ok_or(ParseError::MissingOperand)?;
            let left = operand_stk.pop_back().ok_or(ParseError::MissingOperand)?;
            operand_stk.push_back(AstNode::Binary {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
    }

    Ok(())
}

pub fn parse_token_to_ast(tokens: &[Token]) -> Result<AstNode, ParseError> {
    let mut operand_stk: VecDeque<AstNode> = VecDeque::new();
    let mut operator_stk: VecDeque<StackItem> = VecDeque::new();

    for token in tokens {
        match token {
            Token::Word(word) => {
                operand_stk.push_back(AstNode::Word(word.clone()));
            }
            Token::Operator(op) => {
                while let Some(top) = operator_stk.back() {
                    match top {
                        StackItem::Symbol(_) => break,
                        StackItem::Operator(top_op) => {
                            if top_op.precedence() <= op.precedence() {
                                break;
                            }
                            let StackItem::Operator(stk_op) =
                                operator_stk.pop_back().unwrap()
                            else {
                                unreachable!();
                            };
                            reduce(&mut operand_stk, stk_op)?;
                        }
                    }
                }
                operator_stk.push_back(StackItem::Operator(op.clone()));
            },
            Token::Symbol(Symbol::LeftParen) => {
                operator_stk.push_back(StackItem::Symbol(Symbol::LeftParen.clone()));
            }
            Token::Symbol(Symbol::RightParen) => {
                loop {
                    match operator_stk.pop_back() {
                        Some(StackItem::Operator(op)) => {
                            reduce(&mut operand_stk, op)?;
                        }
                        Some(StackItem::Symbol(Symbol::LeftParen)) => {
                            break;
                        }
                        None => {
                            return Err(ParseError::UnmatchedRightParen);
                        }
                        _ =>{}
                    }
                }
            }
            Token::EOF => {}
        }
    }

    while let Some(item) = operator_stk.pop_back() {
        match item {
            StackItem::Operator(op) => {
                reduce(&mut operand_stk, op)?;
            }

            StackItem::Symbol(Symbol::LeftParen) => {
                return Err(ParseError::UnmatchedLeftParen);
            }
            _ =>{}
        }
    }

    if operand_stk.len() != 1 {
        return Err(ParseError::InvalidExpression);
    }

    Ok(operand_stk.pop_back().unwrap())
}

