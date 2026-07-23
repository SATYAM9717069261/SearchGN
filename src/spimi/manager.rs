use crate::indexer::inverted_index::InvertedIndex;
use crate::spimi::writer::BlockWriter;

use std::path::PathBuf;
use std::io::{self,Read};

/**
 * store Current Inverted Index
 * count how many words processed
 * is limit reached
 */
pub struct SPIMIManager {
    current_index: InvertedIndex,
    block_id: usize,
    words_processed: usize,
    document_processed:usize,
    flush_limit: usize,

    block_writer:BlockWriter
}

impl SPIMIManager{
    pub fn new(output_path:PathBuf, flush_limit:usize) -> io::Result<Self>{
        Ok(SPIMIManager{
            current_index: InvertedIndex::new(),
            block_id: 0,
            words_processed: 0,
            document_processed: 0,
            flush_limit: flush_limit,

            block_writer: BlockWriter::new(output_path)?,
        })
    }

    pub fn get_current_index(&mut self) -> &mut InvertedIndex{
        return &mut self.current_index;
    }

    pub fn add_doument_process_count(&mut self) {
        self.document_processed+=1;
    }

    fn flush(&mut self)-> io::Result<()>{
        self.block_writer.write_block(&self.current_index, self.block_id);
        self.block_id+=1;
        self.words_processed = 0;
        self.current_index.clear();
        Ok(())
    }

    pub fn add_term(&mut self ,key:String, f_idx:usize, line_number:u32, word_start_at:u32)-> io::Result<()> {
        self.current_index.add_term(key, f_idx, line_number, word_start_at);
        self.words_processed += 1;
        if self.words_processed >= self.flush_limit {
            self.flush()?;
        }
        Ok(())
    }

}
