use crate::models::word_start_at::WordStartAt;

#[derive(Debug,Clone)]
pub struct Posting{
    name:String,
    freq:u32,
    line_no:Vec<u32>,
    word_start_from: Vec<WordStartAt>,
    /*
     * Postiong{
     *  fileName,
     *  how many time Occure Word(A) in this File
     *  word Occure in Multiple Lines
     *   same Line have mutiple time mentioned Word[
     *              start_at:[1,8,10,100]
     *              start_at:[1,8,10,100]
     *      ]
     * }
     */
}
impl Posting{
    pub fn new(name:String,freq:u32)->Self{
        Posting{
            name:name,
            freq:freq,
            line_no: Vec::new(),
            word_start_from: Vec::new()
        }
    }
}
