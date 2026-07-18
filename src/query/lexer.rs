use crate::query::token::TOKEN;
use crate::query::operator::Operator;

pub struct Lexer{
    registry: KeywordRegistry,
}

impl Lexer{
    pub fn new() -> Self{
        let mut reg = KeywordRegistory::new();
        reg.register("and",Operator::And);
        reg.register("or",Operator::Or);
        reg.register("not",Operator::Not);
        reg.register("(",OperatorToken::LeftParen);
        reg.register(")",OperatorToken::RightParen);
        Self{
            registry:reg
        }
    }

    pub fn tokenizer(&self, input:&str) -> Vec<Token>{
        let inp_iter = input.tim().split_whitespace();
        let mut token_list:Vec<Token> = Vec::new();

        for word in inp_iter {
            if let Some(op) = self.registry.lookup(&word.to_ascii_lowercase()){
                token_list.push(Token::Operator(*op));
            }else{
                token_list.push(Token::Word(word.to_string()));
            }
        }
        token_list
    }
}
