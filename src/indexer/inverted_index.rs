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

    pub fn insert(self:&mut InvertedIndex ,key:String, posting:&mut Posting, line_number:u32, word_start_at:u32){
        match self.map.get_mut(&key){
            Some(details) => {
                let mut post_iter = details.iter_mut();
                let mut fileFound:bool = false;
                while let Some(post) = post_iter.next(){
                    post.freq+=1;
                    if post.name == posting.name{ // matched file
                        fileFound = true;
                        let mut line_found = false;
                        for (idx,&line) in post.line_no.iter().enumerate(){
                            if line == line_number{
                                line_found = true;
                                post.word_start_from[idx].push(word_start_at);
                            }
                        }
                        if line_found == false{
                            post.line_no.push(line_number);
                            post.word_start_from.push(WordStartAt::new(word_start_at));
                        }
                    }
                }
                if fileFound == false{
                    posting.line_no.push(line_number);
                    let word_at = WordStartAt::new(word_start_at);
                    posting.word_start_from.push(word_at);
                    details.push(posting.clone());
                }
            },
            None => {
                posting.line_no.push(line_number);
                let word_at = WordStartAt::new(word_start_at);
                posting.word_start_from.push(word_at);
                self.map.insert(key,vec![posting.clone()]);
            }
        }
    }
}
