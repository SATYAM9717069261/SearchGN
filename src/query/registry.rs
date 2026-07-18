/*
 * let registry = KeywordRegistry::new();
 * registry.register("AND", Operator::And);
 * registry.register("OR", Operator::Or);
 * registry.register("NOT", Operator::Not);
 * registry.lookup(word)
 *  in lexer
 *          => if word know then create Operator
 *          => otherwiae if Word
 */
use super::operator::Operator;
pub struct KeywordRegistry{
    operator_map:HashMap<&'static str, Operator>
}

impl KeywordRegistry{
    pub fn new() -> Self{
        KeywordRegistry{
            operator_map:HashMap::new()
        }
    }

    pub fn register(&mut self, word:&'static str, op:Operator){
        self.operator_map.insert(word,op);
    }

    pub fn lookup(&self, word:&str) -> Option<&Operator>{
        self.operator_map.get(word)
    }
}






