use super::operator::StackItem;

use std::collections::HashMap;

pub struct KeywordRegistry{
    operator_map:HashMap<&'static str, StackItem>
}

impl KeywordRegistry{
    pub fn new() -> Self{
        KeywordRegistry{
            operator_map:HashMap::new()
        }
    }

    pub fn register(&mut self, word:&'static str, op:StackItem){
        self.operator_map.insert(word,op);
    }

    pub fn lookup(&self, word:&str) -> Option<&StackItem>{
        self.operator_map.get(word)
    }
}

