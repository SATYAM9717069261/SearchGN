use crate::indexer::word::Word;
use crate::indexer::inverted_index::InvertedIndex;

use std::io::{self,Read};
use std::fs::{File};

pub fn insert_token(file_path:&std::path::Path ,index: &mut InvertedIndex)->io::Result<()>{
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 64*1024]; // 64 Kbytes

    if let Some(f_name)= file_path.to_str(){
        println!("::::::::::::::: {} :::::::::::::::",f_name);
        let doc_idx = index.add_document(f_name)?;
        let mut word:Word = Word::new();
        let mut line:u32 = 0;
        let mut bytes_read_count:u32 = 0;
        let mut position = 0;

        while let Ok(n) = file.read(&mut buffer){
            if n == 0{
                break;
            }
            let content = String::from_utf8_lossy(&buffer[..n]);
            for byte in content.chars(){
                bytes_read_count+=1;
                match byte{
                    ' ' => {
                        let term = word.get();
                        index.add_term(term,doc_idx,line,position);
                        word.clear();
                        position = bytes_read_count+1;
                    },
                    '\n' => {
                        let term = word.get();
                        index.add_term(term,doc_idx,line,position);
                        word.clear();
                        line+=1;
                        bytes_read_count= 0;
                        position = 0;
                    },
                    _ => {
                       word.push(byte);
                    }
                }
            }

        }
    }
    Ok(())
}
