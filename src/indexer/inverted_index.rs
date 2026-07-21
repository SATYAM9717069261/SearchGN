use serde::{Serialize, Deserialize};

use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;

use std::collections::HashMap;
use std::io::{self};

#[derive(Debug,Serialize, Deserialize)]
pub struct InvertedIndex{
    map: HashMap<String,Vec<Posting>>,
    documents:Vec<String>,
    posting_created: usize,
    words_processed: usize,
}

impl InvertedIndex{
    pub fn new()->Self{
        InvertedIndex{
            map: HashMap::new(),
            documents:Vec::new(),

            //debugging
            posting_created: 0,
            words_processed: 0,
        }
    }
    pub fn print_debugging_details(&self){
        println!("Unique Words Count: {:?}",self.map.len());
        println!("Number of Words Procesed: {:?}",self.words_processed);
        println!("Document Processed: {:?}",self.documents.len());

    }
    pub fn add_document(&mut self, document_id:&str) -> io::Result<usize>{
        if document_id.trim() == ""{
            return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Document id : {}",document_id).to_string(),
            ));
        }
        if let Some(idx) = self.documents.iter().position( |d| d == document_id ){
            return Ok(idx);
        }
        self.documents.push(document_id.to_string());
        return Ok(self.documents.len() - 1);
    }

    pub fn get_docuemnt(&self, idx:usize) -> &str{
        self.documents[idx].as_str()
    }

    pub fn get_all_document(&self) ->&Vec<String>{
        &self.documents
    }
    pub fn get_all_words(&self) -> Vec<String>{
        self.map.clone().into_keys().collect()
    }

    pub fn add_term(self:&mut InvertedIndex ,key:String, f_idx:usize, line_number:u32, word_start_at:u32){
        match self.map.get_mut(&key){
            Some(details) => {
                if let Some(last_document) = details.last_mut(){
                    if last_document.get_document_id() == f_idx{
                        let line_no_vec = last_document.get_line_no();
                        if let Some(&last_line) = line_no_vec.last() {
                            //if every thing match [document_id, line_no] only need to push where you
                            //should start reading
                            if last_line == line_number{
                                let last_line_at_idx = line_no_vec.len() - 1;
                                last_document.update_position_at(last_line_at_idx, word_start_at);
                            }else{
                                last_document.add_occurrence(line_number,word_start_at); // add new line
                            }
                        } else {
                            // if line number not match
                            last_document.add_occurrence(line_number,word_start_at);
                        }
                    }else{
                        let mut line_no_list = vec![];
                        line_no_list.push(line_number);
                        let mut word_start_at_list = vec![];
                        word_start_at_list.push(WordStartAt::new(word_start_at));
                        let posting = Posting::new( f_idx, 1, line_no_list, word_start_at_list);
                        details.push(posting);
                    }
                }else{
                    if key != ""{
                        let mut line_no_list = vec![];
                        line_no_list.push(line_number);
                        let mut word_start_at_list = vec![];
                        word_start_at_list.push(WordStartAt::new(word_start_at));
                        let posting = Posting::new( f_idx, 1, line_no_list, word_start_at_list);
                        self.map.insert(key,vec![posting]);
                    }
                }
            },
            None => {
                // if key not exist in hashMap then insert new key
                let mut line_no_list = vec![];
                line_no_list.push(line_number);
                let mut word_start_at_list = vec![];
                word_start_at_list.push(WordStartAt::new(word_start_at));
                let posting = Posting::new( f_idx, 1, line_no_list, word_start_at_list);
                self.map.insert(key,vec![posting]);
            }
        }
    }

    pub fn get_words_processed(&self) -> usize{
        self.words_processed
    }
    pub fn add_words_processed(&mut self) {
        self.words_processed+=1;
    }
    pub fn search_word(&self, word:&str) -> Result<&Vec<Posting>,&'static str>{
        match self.map.get(word){
            Some(details) => Ok(details),
            None => Err("Not Found")
        }
    }

}
