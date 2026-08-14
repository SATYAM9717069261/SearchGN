use std::io::{self,BufWriter,Write,Seek,SeekFrom};
use std::fs::{self,File};
use std::path::PathBuf;

use crate::spimi::format::{MAGIC, VERSION, BLOCK_EXTENSION};
use crate::models::posting::Posting;
use crate::spimi::reader::{WordEntry};
use crate::models::word_start_at::WordStartAt;

pub struct Merge_Writer{
    write_buffer: BufWriter<File>,
    word_count: u32,
    file_path: PathBuf
}

impl Merge_Writer{
    pub fn new(output_path:&PathBuf, id:u32) ->io::Result<Self>{
        let path = output_path.join(format!("merge_block_{}.{}",id,BLOCK_EXTENSION));
        let file = File::create(&path)?;
        let mut write =  BufWriter::new(file);

        write.write_all(MAGIC)?;
        write.write_all(&VERSION.to_le_bytes())?;
        write.write_all(&0u32.to_le_bytes())?;

        Ok(Merge_Writer{ file_path:path, write_buffer: write, word_count: 0 })
    }
    pub fn get_file_path(&self) -> PathBuf{
        self.file_path.clone()
    }
    pub fn write_word_entry(&mut self, word_entry:&WordEntry) -> io::Result<()>{
        let word: &str = &word_entry.word;
        let postings: &[Posting] = &word_entry.postings;
        let word_len = word.len() as u32;

        self.write_buffer.write_all(&word_len.to_le_bytes())?;
        self.write_buffer.write_all(word.as_bytes())?;

        let posting_count = postings.len() as u32;
        self.write_buffer.write_all(&posting_count.to_le_bytes())?;
        for posting in postings{
            self.write_posting(posting)?;
        }
        self.word_count+=1;
        Ok(())
    }

    fn write_posting(&mut self, posting: &Posting,) -> io::Result<()> {
        self.write_buffer.write_all(&posting.get_document_id().to_le_bytes())?;
        self.write_buffer.write_all(&posting.get_frequency().to_le_bytes())?;

        let line_count = posting.get_line_no().len() as u32;
        self.write_buffer.write_all(&line_count.to_le_bytes())?;

        for (line_no, positions) in posting.get_line_no().iter()
            .zip(posting.get_word_start_at()){
                self.write_line(*line_no, positions)?;
            }
        Ok(())
    }


    fn write_line(&mut self, line_no: u32, positions: &WordStartAt) -> io::Result<()>{
        self.write_buffer.write_all(&line_no.to_le_bytes())?; // line number
        let position_count = positions.get_start_at().len() as u32;
        self.write_buffer.write_all(&position_count.to_le_bytes())?; // lines.len()
        self.write_positions(positions)?;
        Ok(())
    }

    fn write_positions(&mut self, positions: &WordStartAt,) -> io::Result<()> {
        for pos in positions.get_start_at() {
            self.write_buffer.write_all(&pos.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        self.write_buffer.flush()?;
        let mut file = self.write_buffer.into_inner().map_err(|e| e.into_error())?;
        file.seek(SeekFrom::Start(12))?; // skip 12 Bytes
        file.write_all(&self.word_count.to_le_bytes())?;
        Ok(())
    }

}
