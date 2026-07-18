pub mod parser;

pub mod operator;
pub mod token;
pub mod ast;
pub mod lexer;
pub mod registry;
pub mod evaluator;


#[cfg(test)]
mod evaluator_test;

#[cfg(test)]
mod parser_test;
