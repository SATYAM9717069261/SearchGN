use crate::models::posting::Posting;
use crate::indexer::word::Word;

use crate::indexer::inverted_index::InvertedIndex;

use std::io::{self,Read};
use std::fs::{File};

pub fn insert_token(file_path:&std::path::Path ,index: &mut InvertedIndex)->io::Result<()>{
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 1024];

    if let Some(f_name)= file_path.to_str(){
        let mut document = Posting::new(f_name.to_string(),0);
        let mut word:Word = Word::new();

        let mut line:u32 = 0;
        let mut bytes_read_count:u32 = 0;
        let mut position = 0;

        while let Ok(n) = file.read(&mut buffer){
            if n == 0{
                break;
            }

            for byte in &buffer[..n]{
                bytes_read_count+=1;
                match byte{
                    b' ' => {
                        let term = word.get();
                        index.add_term(term,&mut document,line,position);
                        word.clear();
                        position = bytes_read_count+1;
                    },
                    b'\n' => {
                        line+=1;
                        // push in hash
                    },
                    _ => {
                        word.push(*byte as char);
                    }
                }
            }
        }
    }
    Ok(())
}
