use std::io::{self,BufWriter,Write};
use std::fs::{self,File};
use std::path::PathBuf;

use crate::models::posting::Posting;
use crate::indexer::inverted_index::InvertedIndex;
use crate::models::word_start_at::WordStartAt;
use crate::spimi::format::{MAGIC, VERSION, BLOCK_EXTENSION};
/*
 * Magic Number (8 bytes)
 * Version (u32)
 * Word Count (u32)
 */

pub struct BlockWriter {
    writer: BufWriter<File>,
    word_count: u32,
}

impl BlockWriter {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(MAGIC)?;
        writer.write_all(&VERSION.to_le_bytes())?;

        writer.write_all(&0u32.to_le_bytes())?;

        Ok(Self {
            writer,
            word_count: 0,
        })
    }

    pub fn write_word_entry( &mut self, word: &str, postings: &[Posting]) -> io::Result<()> {
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

    fn write_posting( &mut self, posting: &Posting,) -> io::Result<()> {
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

    fn write_line( &mut self, line_no: u32, positions: &WordStartAt,) -> io::Result<()> {
        writer.write_all(&line_no.to_le_bytes())?; // line number
        let position_count = positions.get_start_at().len() as u32;
        writer.write_all(&position_count.to_le_bytes())?; // lines.len()
        self.write_positions(writer, positions)?;
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        self.writer.flush()?;
        self.writer.seek(SeekFrom::Start(12))?;
        self.writer.write_all(&self.word_count.to_le_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    fn write_positions( &self, writer: &mut BufWriter<File>, positions: &WordStartAt,) -> io::Result<()> {
        for pos in positions.get_start_at() {
            writer.write_all(&pos.to_le_bytes())?;
        }
        Ok(())
    }

}
