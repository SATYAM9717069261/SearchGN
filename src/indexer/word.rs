use std::mem;

pub struct Word{
    _word:String,
}

impl Word{
    pub fn new()->Self{
        Word{
            _word:String::from("")
        }
    }
    pub fn push(&mut self, ch:char){
        // filter pending
        if Self::is_word_char(ch){
            self._word.push(ch.to_ascii_lowercase());
        }
    }

    pub fn get(&mut self)->String{
        mem::take(&mut self._word)
    }
    pub fn clear(&mut self){
        self._word.clear();
    }

    pub fn finish(&mut self) -> Option<String> {
        if self._word.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self._word))
        }
    }

    pub fn is_word_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }
}
