use std::io::{self,BufWriter,Write};
use std::fs::{self,File};
use std::path::PathBuf;

use crate::models::posting::Posting;
use crate::indexer::inverted_index::InvertedIndex;

/*
 * Magic Number (8 bytes)
 * Version (u32)
 * Word Count (u32)
 */

const MAGIC: &[u8; 8] = b"SEARCHGN";
const VERSION: u32 = 1;

pub struct BlockWriter {
    output_path: PathBuf,
}

impl BlockWriter {
    pub fn new(output_path: PathBuf) ->io::Result<Self>{
        fs::create_dir_all(&output_path)?;
        Ok(Self{ output_path })
    }

    fn write_header(&mut self,writer: &mut BufWriter<File>, word_count: u32) -> io::Result<()>{
        writer.write_all(MAGIC)?;
        writer.write_all(&VERSION.to_le_bytes())?;
        writer.write_all(&word_count.to_le_bytes())?;
        Ok(())
    }

    fn write_entry( &mut self, word: &str, postings: &Vec<Posting>,) -> io::Result<()>{
        todo!();
    }

    pub fn write_block( &mut self, index: &InvertedIndex, block_id:usize) -> io::Result<()>{
        let path = self.output_path.join(format!("block_{}.idx",block_id));
        println!("indx Path : {:?}",path);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.write_header( &mut writer,
            index.get_map_len() as u32,
        )?;
        Ok(())
    }
}
