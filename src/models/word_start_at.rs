#[derive(Debug,Clone)]
pub struct WordStartAt{
    start_at:Vec<u32>
}
impl WordStartAt{
    pub fn new(start_at:u32) -> Self{
        WordStartAt{
            start_at: vec![start_at]
        }
    }
    pub fn push(&mut self,start_at:u32){
        self.start_at.push(start_at);
    }
}


