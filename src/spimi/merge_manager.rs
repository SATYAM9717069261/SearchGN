use std::path::PathBuf;
use std::io::{self,BufWriter,Write};
use std::fs::{self,File};
use std::cmp;

use crate::spimi::format::{BLOCK_EXTENSION,MAGIC, VERSION};
use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;
use crate::spimi::reader::{BlockReader,WordEntry};
use crate::spimi::merge_writer::Merge_Writer;


pub struct MergeManager {
    path: PathBuf,
    output_path: PathBuf,
}

impl MergeManager{
    pub fn new(path:PathBuf, output_path: PathBuf) -> Self{
        if fs::create_dir_all(&output_path).is_ok(){
            return Self{
                path:path,
                output_path: output_path,
            }
        }else{
            panic!("Check Output Path: {:?}",output_path);
        }
    }

    pub fn get_file_name(&self,id:usize) -> PathBuf{
        self.path.join(format!("block_{}.{}", id, BLOCK_EXTENSION))
    }

    pub fn merge_two_blocks( &mut self, left_id: usize, right_id: usize) -> io::Result<()> {
        let mut writer = Merge_Writer::new(&self.output_path,1)?;

        let mut left = BlockReader::new(self.get_file_name(left_id))?;
        let mut right = BlockReader::new(self.get_file_name(right_id))?;

        let mut left_entry =  left.next_word()?; // WordEntry{ word:String, postings:Vec<POSTING> }
        let mut right_entry =  right.next_word()?;

        while left_entry.is_some() && right_entry.is_some(){
            let left_word = left_entry.as_ref().unwrap();
            let right_word = right_entry.as_ref().unwrap();
            if left_word.word < right_word.word {
                writer.write_word_entry(left_word)?;
                left_entry = left.next_word()?;
            } else if left_word.word > right_word.word {
                writer.write_word_entry(right_word)?;
                right_entry = right.next_word()?;
            } else {
                let merged:WordEntry = Self::merge_postings(left_word, right_word);
                writer.write_word_entry(&merged)?;

                left_entry = left.next_word()?;
                right_entry = right.next_word()?;
            }
        }


        while let Some(entry) = left_entry {
            writer.write_word_entry(&entry)?;
            left_entry = left.next_word()?;
        }

        while let Some(entry) = right_entry {
            writer.write_word_entry(&entry)?;
            right_entry = right.next_word()?;
        }

        let size = std::fs::metadata(writer.get_file_path())?.len();
        println!("FILE SIZE After Write = {} bytes {:?}", size, writer.get_file_path());
        writer.finish()?;
        Ok(())
    }


    fn merge_postings<'a>(left:&'a WordEntry, right:&'a WordEntry) ->  WordEntry{
        let left_postings:&[Posting] = &left.postings;
        let right_postings:&[Posting] = &right.postings;
        let mut merged_posting:Vec<Posting> = vec![];

        let mut idx_left:usize = 0;
        let mut idx_right:usize = 0;

        while idx_left < left_postings.len() && idx_right < right_postings.len() {
            let document_id_left = left_postings[idx_left].get_document_id();
            let document_id_right = right_postings[idx_right].get_document_id();
            if document_id_left > document_id_right{
                merged_posting.push(right_postings[idx_right].clone());
                idx_right+=1;
            }else if document_id_left < document_id_right{
                merged_posting.push(left_postings[idx_left].clone());
                idx_left+=1;
            }else{
                let merged = Self::merge_single_posting(&left_postings[idx_left], &right_postings[idx_right]);
                merged_posting.push(merged);
                idx_left += 1;
                idx_right += 1;
            }
        }

        while idx_left < left_postings.len() {
            merged_posting.push(
                left_postings[idx_left].clone()
            );
            idx_left += 1;
        }

        while idx_right < right_postings.len() {
            merged_posting.push(
                right_postings[idx_right].clone()
            );
            idx_right += 1;
        }


        let mut word_entry: WordEntry = WordEntry{
            word: left.word.clone(),
            postings: merged_posting
        };
        return word_entry;
    }

    fn merge_single_posting( left: &Posting, right: &Posting) -> Posting {

        let document_id = left.get_document_id();
        let frequency = left.get_frequency() + right.get_frequency();

        let mut line_no: Vec<u32> = Vec::new();
        let mut positions: Vec<WordStartAt> = Vec::new();

        let left_lines = left.get_line_no();
        let right_lines = right.get_line_no();

        let left_positions = left.get_word_start_at();
        let right_positions = right.get_word_start_at();

        let mut i = 0;
        let mut j = 0;

        while i < left_lines.len() && j < right_lines.len() {
                if left_lines[i] < right_lines[j] {

                    line_no.push(left_lines[i]);
                    positions.push(left_positions[i].clone());

                    i += 1;

                } else if left_lines[i] > right_lines[j] {

                    line_no.push(right_lines[j]);
                    positions.push(right_positions[j].clone());

                    j += 1;

                } else {
                    let line = left_lines[i];

                    let mut merged_positions = left_positions[i].clone();
                    merged_positions.merge(&right_positions[j]);
                    line_no.push(line);
                    positions.push(merged_positions);

                    i += 1;
                    j += 1;
                }
            }

        while i < left_lines.len() {
            line_no.push(left_lines[i]);
            positions.push(left_positions[i].clone());
            i += 1;
        }

        while j < right_lines.len() {
            line_no.push(right_lines[j]);
            positions.push(right_positions[j].clone());
            j += 1;
        }
        return Posting::new( document_id, frequency, line_no, positions);
    }
}

