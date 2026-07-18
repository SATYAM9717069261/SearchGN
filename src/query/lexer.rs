use crate::query::token::Token;
use crate::query::registry::KeywordRegistry;
use crate::query::operator::{StackItem,Symbol,Operator};

pub struct Lexer{
    registry: KeywordRegistry,
}

impl Lexer{
    pub fn new() -> Self{
        let mut reg = KeywordRegistry::new();
        reg.register("and",StackItem::Operator(Operator::And));
        reg.register("or", StackItem::Operator(Operator::Or));
        reg.register("not", StackItem::Operator(Operator::Not));
        reg.register("(", StackItem::Symbol(Symbol::LeftParen));
        reg.register(")", StackItem::Symbol(Symbol::RightParen));
        Self{
            registry:reg
        }
    }

    pub fn tokenizer(&self, input:&str) -> Vec<Token>{
        let inp_iter = input.trim().split_whitespace();
        let mut token_list:Vec<Token> = Vec::new();

        for word in inp_iter {
            if let Some(op) = self.registry.lookup(&word.to_ascii_lowercase()){
                match op{
                    StackItem::Symbol(op) => {
                        token_list.push(Token::Symbol(*op));
                    },
                    StackItem::Operator(op) => {
                        token_list.push(Token::Operator(*op));
                    }
                }
            }else{
                token_list.push(Token::Word(word.to_string()));
            }
        }
        token_list
    }
}
