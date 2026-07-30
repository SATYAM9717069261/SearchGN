use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct WordStartAt{
    start_at:Vec<u32>
}
impl WordStartAt{
    pub fn new() -> Self{
        WordStartAt{
            start_at: vec![]
        }
    }
    pub fn push(&mut self,start_at:u32){
        self.start_at.push(start_at);
    }
    pub fn get_start_at(&self)->&[u32]{
        &self.start_at
    }
}


