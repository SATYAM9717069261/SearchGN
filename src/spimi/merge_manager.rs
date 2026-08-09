use crate::spimi::format::{BLOCK_EXTENSION};
use crate::spimi::reader::BlockReader;
use std::io;

use std::path::PathBuf;

pub struct MergeManager {
    output_path: PathBuf,
    path: PathBuf,
}

impl MergeManager{
    pub fn new(path:PathBuf, output_path: PathBuf) -> Self{
        Self{
            path,
            output_path,
        }
    }

    pub fn get_file_name(&self,id:usize) -> PathBuf{
        self.path.join(format!("block_{}.{}", id, BLOCK_EXTENSION))
    }

    pub fn merge_two_blocks( &mut self, left_id: usize, right_id: usize,) -> io::Result<()> {
        let mut left = BlockReader::new(self.get_file_name(left_id))?;
        let mut right = BlockReader::new(self.get_file_name(right_id))?;

        let mut left_entry =  left.next_word()?;
        let mut right_entry =  right.next_word()?;

        while left_entry.is_some() && right_entry.is_some(){
            let left_word = left_entry.as_ref().unwrap();
            let right_word = right_entry.as_ref().unwrap();
            if left_word.word < right_word.word {
                write(left);
                left_entry = left.next_word()?;
            } else if left_word.word > right_word.word {
                write(right);
                right_entry = right.next_word()?;
            } else {
                // merge postings
                let merged = merge_postings(left, right);
                write(merged);
                left_entry = left.next_word()?;
                right_entry = right.next_word()?;
            }

            println!("last =>
            {:?} {:?} ", left_entry.as_ref().map(|e| &e.word),
            right_entry.as_ref().map(|e| &e.word)
            );
        }


        while let Some(entry) = left_entry {
            write(left);
            left_entry = left.next_word()?;
        }

        while let Some(entry) = right_entry {
            write(right);
            right_entry = right.next_word()?;
        }

        Ok(())
    }
}
