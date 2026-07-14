use serde::{Serialize, Deserialize};

use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;

use std::collections::HashMap;

#[derive(Debug,Serialize, Deserialize)]
pub struct InvertedIndex{
    map: HashMap<String,Vec<Posting>>
}

impl InvertedIndex{
    pub fn new()->Self{
        InvertedIndex{
            map: HashMap::new()
        }
    }
    pub fn get_len(&self) ->usize{
        self.map.len()
    }
    pub fn add_term(self:&mut InvertedIndex ,key:String, f_name:&str, line_number:u32, word_start_at:u32){
        match self.map.get_mut(&key){
            Some(details) => {
                if let Some(post) = details.iter_mut().find(|document| document.get_document_id() == f_name){
                    let idx = post.get_line_no().iter().position(|&line| line == line_number);
                    if let Some(idx) = idx {
                        //if every thing match [document_id, line_no] only need to push where you
                        //should start reading
                        post.update_position_at(idx, word_start_at);
                    } else {
                        // if line number not match
                        post.add_occurrence(line_number,word_start_at);
                    }
                }else{
                    //if document_id not match
                    let mut line_no_list = vec![];
                    line_no_list.push(line_number);
                    let mut word_start_at_list = vec![];
                    word_start_at_list.push(WordStartAt::new(word_start_at));
                    let mut posting = Posting::new(
                        f_name.to_string(),
                        1,
                        line_no_list,
                        word_start_at_list
                    );
                    details.push(posting);
                }
            },
            None => {
                // if key not exist in hashMap then insert new key
                if key != ""{
                    let mut line_no_list = vec![];
                    line_no_list.push(line_number);
                    let mut word_start_at_list = vec![];
                    word_start_at_list.push(WordStartAt::new(word_start_at));
                    let mut posting = Posting::new(
                        f_name.to_string(),
                        1,
                        line_no_list,
                        word_start_at_list
                    );
                    self.map.insert(key,vec![posting]);
                }
            }
        }
    }
}
