use crate::models::posting::Posting;
use crate::indexer::word::Word;

use crate::indexer::inverted_index::InvertedIndex;

use std::io::{self,Read};
use std::fs::{File};

pub fn insert_token(file_path:&std::path::Path ,index: &mut InvertedIndex)->io::Result<()>{
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 1024];

    if let Some(f_name)= file_path.to_str(){
        let mut word:Word = Word::new();
        let mut word_count = 0;
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
                        index.add_term(term,f_name,line,position);
                        word_count+=1;
                        word.clear();
                        position = bytes_read_count+1;
                    },
                    b'\n' => {
                        let term = word.get();
                        index.add_term(term,f_name,line,position);
                        word_count+=1;
                        word.clear();
                        line+=1;
                        bytes_read_count= 0;
                        position = 0;
                    },
                    _ => {
                        word.push(*byte as char);
                    }
                }
            }

            if (word_count % 10000) == 0{
                println!(
                    "Processed {} words | Unique terms: {}",
                    word_count,
                    index.get_len()
                )
            }
        }
    }
    Ok(())
}
