use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use crate::spimi::format::{MAGIC, VERSION,BLOCK_EXTENSION};
use crate::indexer::inverted_index::InvertedIndex;

pub struct BlockReader {
    input_path: PathBuf,
}

fn read_u32(reader: &mut BufReader<File>) -> io::Result<u32>{
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let value = u32::from_le_bytes(buf);
    Ok(value)
}


impl BlockReader {
    pub fn new(input_path: PathBuf) -> io::Result<Self> {
        Ok(Self { input_path })
    }


    fn read_header( &self, reader: &mut BufReader<File>,) -> io::Result<u32>{
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

    fn read_word_entry( &self, reader: &mut BufReader<File>, index: &mut InvertedIndex) -> io::Result<()> {
        let word_len = read_u32(reader)?;

        let mut bytes = vec![0u8; word_len as usize];
        reader.read_exact(&mut bytes)?;

        let word = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8",
            ))?;
        let mut posting_count_bytes = [0u8; 4];
        reader.read_exact(&mut posting_count_bytes)?;
        let posting_count = u32::from_le_bytes(posting_count_bytes);

        for _ in 0..posting_count {
            self.read_posting(reader,&word,index)?;
        }
        Ok(())
    }


    fn read_posting( &self, reader: &mut BufReader<File>,word:&str,index: &mut InvertedIndex) -> io::Result<()> {
        let document_id = read_u32(reader)?;
        let frequency = read_u32(reader)?;
        let line_count = read_u32(reader)?;

        for _ in 0..line_count {
            self.read_line(reader,word,document_id,index)?;
        }
        Ok(())
    }

    fn read_line( &self, reader: &mut BufReader<File>,word:&str,document_id:u32,index: &mut InvertedIndex) -> io::Result<()> {
        let line_no = read_u32(reader)?;
        let position_count = read_u32(reader)?;
        for _ in 0..position_count {
            let rtn_pos = self.read_position(reader)?;
            index.add_term(word.to_owned(),document_id,line_no,rtn_pos);
        }
        Ok(())
    }

    fn read_position( &self, reader: &mut BufReader<File>) -> io::Result<u32> {
        let position = read_u32(reader)?;
        Ok(position)
    }


    pub fn read_block( &self, block_id: usize,) -> io::Result<InvertedIndex> {
        let path = self.input_path.join(format!("block_{}.{}", block_id,BLOCK_EXTENSION));
        let size = std::fs::metadata(&path)?.len();
        println!("FILE SIZE READ = {} {:?}", size, path);

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut index = InvertedIndex::new();

        let word_count = self.read_header(&mut reader)?;
        for _ in 0..word_count {
            self.read_word_entry(&mut reader,&mut index)?;
        }

        Ok(index)
    }
}
