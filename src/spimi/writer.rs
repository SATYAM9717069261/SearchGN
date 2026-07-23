use std::io::{self,BufWriter,Write};
use std::fs::{self,File};
use std::path::PathBuf;

use crate::models::posting::Posting;
use crate::indexer::inverted_index::InvertedIndex;
use crate::models::word_start_at::WordStartAt;

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

    fn write_word_entry(&mut self, writer:&mut BufWriter<File>, word: &str, postings: &[Posting]) -> io::Result<()>{
        let word_len = word.len() as u32;

        writer.write_all(&word_len.to_le_bytes())?;
        writer.write_all(word.as_bytes())?;

        let posting_count = postings.len() as u32;
        writer.write_all(&posting_count.to_le_bytes())?;
        for posting in postings{
            self.write_posting(writer,posting)?;
        }
        Ok(())
    }

    fn write_posting(&self, writer: &mut BufWriter<File>, posting: &Posting,) -> io::Result<()> {
        writer.write_all(&posting.get_document_id().to_le_bytes())?;
        writer.write_all(&posting.get_frequency().to_le_bytes())?;

        let line_count = posting.get_line_no().len() as u32;
        writer.write_all(&line_count.to_le_bytes())?;
        for (line_no, positions) in posting.get_line_no().iter()
            .zip(posting.get_word_start_at()){
                self.write_line(writer, *line_no, positions)?;
            }

        Ok(())
    }

    fn write_line( &self, writer: &mut BufWriter<File>, line_no: u32, positions: &WordStartAt,) -> io::Result<()>{
        writer.write_all(&line_no.to_le_bytes())?;
        let position_count = positions.get_start_at().len() as u32;
        writer.write_all(&position_count.to_le_bytes())?;
        self.write_positions(writer, positions)?;
        Ok(())
    }

    fn write_positions( &self, writer: &mut BufWriter<File>, positions: &WordStartAt,) -> io::Result<()> {
        for pos in positions.get_start_at() {
            writer.write_all(&pos.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn write_block(&mut self, index: &mut InvertedIndex, block_id:usize) -> io::Result<()>{
        let path = self.output_path.join(format!("block_{}.idx",block_id));
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.write_header( &mut writer, index.get_map_len() as u32)?;

        for (word,posting) in index.iterator(){
            self.write_word_entry(&mut writer,word,posting);
        }

        Ok(())
    }
}
