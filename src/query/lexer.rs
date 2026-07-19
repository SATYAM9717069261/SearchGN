use crate::query::token::Token;
use crate::query::registry::KeywordRegistry;
use crate::query::operator::{StackItem,Symbol,Operator};
use crate::indexer::word::Word;

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

    pub fn tokenizer(&self, input: &str) -> Vec<Token> {
        let mut token_list = Vec::new();
        let mut word = Word::new();

        for ch in input.chars().chain(std::iter::once(' ')) {
            if Word::is_word_char(ch) {
                word.push(ch);
                continue;
            }
            if let Some(text) = word.finish() {
                if let Some(item) = self.registry.lookup(&text) {
                    match item {
                        StackItem::Operator(op) => {
                            token_list.push(Token::Operator(*op));
                        }
                        StackItem::Symbol(sym) => {
                            token_list.push(Token::Symbol(*sym));
                        }
                    }
                } else {
                    token_list.push(Token::Word(text));
                }
            }
            match ch {
                '(' => token_list.push(Token::Symbol(Symbol::LeftParen)),
                ')' => token_list.push(Token:: Symbol(Symbol::RightParen)),
                _ => {} // Ignore spaces, tabs, commas, etc. for now
            }
        }
        token_list.push(Token::EOF);
        token_list
    }
}

