use std::mem;

pub struct Word{
    _word:String
}

impl Word{
    pub fn new()->Self{
        Word{
            _word:String::from("")
        }
    }
    pub fn push(&mut self, ch:char){
        // filter pending
        self._word.push(ch);
    }
    pub fn get(&mut self)->String{
        mem::take(&mut self._word)
    }
    pub fn clear(&mut self){
        self._word.clear();
    }
}
