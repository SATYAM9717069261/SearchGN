use serde::{Serialize, Deserialize};
use crate::models::word_start_at::WordStartAt;

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Posting{
    document_id:usize,
    frequency:u32,
    line_no:Vec<u32>,
    positions: Vec<WordStartAt>,
    /*
     * Postiong{
     *  fileName,
     *  how many time Occure Word(A) in this File
     *  word Occure in Multiple Lines, each number in this represent entire words in same line
     *   same Line have mutiple time mentioned Word[
     *              start_at:[1,8,10,100]
     *              start_at:[1,8,10,100]
     *      ]
     * }
     */
}
impl Posting{
    pub fn new(index:usize,freq:u32,line_no:Vec<u32>,pos:Vec<WordStartAt>)->Self{
        Posting{
            document_id:index,
            frequency:freq,
            line_no: line_no,
            positions: pos
        }
    }
    pub fn get_document_id(&self)->usize{
        self.document_id
    }
    pub fn get_line_no(&self) -> &Vec<u32>{
        &self.line_no
    }

    pub fn get_frequency(&self) -> u32{
        self.frequency
    }

    pub fn update_frequency(&mut self){
        self.frequency += 1;
    }

    pub fn push_new_line(&mut self, line_number:u32){
        self.line_no.push(line_number);
    }

    pub fn push_new_positions(&mut self, word_start_at:WordStartAt){
        self.positions.push(word_start_at);
    }

    pub fn update_position_at(&mut self,idx:usize, start_at:u32){
        self.frequency += 1;
        self.positions[idx].push(start_at);
    }

    pub fn add_occurrence( &mut self, line: u32, position: u32,) {
        self.frequency += 1;
        self.push_new_line(line);
        self.push_new_positions( WordStartAt::new(position),);
    }
}
