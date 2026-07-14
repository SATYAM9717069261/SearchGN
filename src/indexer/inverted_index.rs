use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct InvertedIndex{
    map: HashMap<String,Vec<Posting>>
}

impl InvertedIndex{
    pub fn new()->Self{
        InvertedIndex{
            map: HashMap::new()
        }
    }

    pub fn add_term(self:&mut InvertedIndex ,key:String, posting:&mut Posting, line_number:u32, word_start_at:u32){
        match self.map.get_mut(&key){
            Some(details) => {
                let mut post_iter = details.iter_mut();
                let mut fileFound:bool = false;
                while let Some(post) = post_iter.next(){

                    if post.get_document_id() == posting.get_document_id(){ // matched file
                        post.update_frequency();
                        fileFound = true;

                        let idx = post.get_line_no().iter().position(|&line| line == line_number);
                        if let Some(idx) = idx {
                            post.update_position_at(idx, word_start_at);
                        } else {
                            post.push_new_line(line_number);
                            post.push_new_positions(WordStartAt::new(word_start_at));
                        }
                    }
                }
                if fileFound == false{
                    posting.add_occurrence(line_number,word_start_at);
                    details.push(posting.clone());
                }
            },
            None => {

                posting.add_occurrence(line_number,word_start_at);
                self.map.insert(key,vec![posting.clone()]);
            }
        }
    }
}
