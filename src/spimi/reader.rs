use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use crate::spimi::format::{MAGIC, VERSION,BLOCK_EXTENSION};
use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;

#[derive(Debug)]
pub struct BlockReader {
    reader: BufReader<File>,
    remaining_words: u32,
}

#[derive(Debug)]
pub struct WordEntry {
    pub word: String,
    pub postings: Vec<Posting>,
}

impl BlockReader {

    pub fn new(input_path: &PathBuf) -> io::Result<BlockReader> {
        let file = File::open(input_path)?;
        let mut reader = BufReader::new(file);
        let remaining_words = Self::read_header(&mut reader)?;
        Ok(Self { reader, remaining_words })
    }

    fn read_u32(&mut self) -> io::Result<u32>{
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        let value = u32::from_le_bytes(buf);
        Ok(value)
    }

    fn read_header(reader: &mut BufReader<File>) -> io::Result<u32>{
        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid block file",
            ));
        }

        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;

        let version = u32::from_le_bytes(version);

        if version != VERSION {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unsupported block version",
            ));
        }

        let mut word_count = [0u8; 4];
        reader.read_exact(&mut word_count)?;

        Ok(u32::from_le_bytes(word_count))
    }

    fn read_word_entry(&mut self) -> io::Result<WordEntry>{
        let word_len = self.read_u32()?;
        let mut bytes = vec![0u8; word_len as usize];

        self.reader.read_exact(&mut bytes)?;

        let word = String::from_utf8(bytes).map_err(|_| io::Error::new( io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        let mut posting_count_bytes = [0u8; 4];

        self.reader.read_exact(&mut posting_count_bytes)?;
        let posting_count = u32::from_le_bytes(posting_count_bytes);

        let mut postings = Vec::new();

        for _ in 0..posting_count {
            let posting = self.read_posting()?;
            postings.push(posting);
        }

        Ok(WordEntry{ word, postings })
    }

    fn read_posting(&mut self) -> io::Result<Posting>{
        let document_id = self.read_u32()?;
        let frequency = self.read_u32()?;
        let line_count = self.read_u32()?;

        let mut line_no:Vec<u32> = vec![];
        let mut positions:Vec<WordStartAt>  = vec![];
        for _ in 0..line_count {
            let (line,word_start_at) = self.read_line()?;
            line_no.push(line);
            positions.push(word_start_at);
        }
        Ok(Posting::new( document_id, frequency, line_no, positions))
    }

    fn read_line(&mut self) -> io::Result<(u32,WordStartAt)> {
        let line_no = self.read_u32()?;
        let position_count = self.read_u32()?;
        let mut word_start_at = WordStartAt::new();

        for _ in 0..position_count {
            let rtn_pos = self.read_position()?;
            word_start_at.push(rtn_pos);
        }

        Ok((line_no,word_start_at))
    }

    fn read_position(&mut self) -> io::Result<u32> {
        let position = self.read_u32()?;
        Ok(position)
    }

    pub fn next_word(&mut self) -> io::Result<Option<WordEntry>>{
        if self.remaining_words == 0 {
            return Ok(None);
        }
        let entry = self.read_word_entry()?;
        self.remaining_words -= 1;
        Ok(Some(entry))
    }
}
