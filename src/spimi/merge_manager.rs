use std::path::PathBuf;
use std::io::{self,BufWriter,Write};
use std::fs::{self,File};

use crate::models::posting::Posting;
use crate::models::word_start_at::WordStartAt;
use crate::spimi::reader::{BlockReader,WordEntry};
use crate::spimi::merge_writer::Merge_Writer;
use crate::spimi::thread_pool::ThreadPool;

use std::sync::{Arc, Mutex};

pub struct MergeManager {
    block_files: Arc<Mutex<Vec<PathBuf>>>,
    thread_pool: ThreadPool,
    output_path: PathBuf,
    next_output_id: Arc<Mutex<u32>>,
}

impl MergeManager{
    pub fn new( path: PathBuf, output_path: PathBuf, thread_count: usize,) -> Self {
        let mut block_files = Vec::new();
        let dir_iter = std::fs::read_dir(&path).expect("Failed to read block directory");
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    block_files.push(entry.path());
                }
                Err(err) => {
                    println!("Block file processing error: {:?}", err);
                }
            }
        }

        std::fs::create_dir_all(&output_path).expect("Failed to create output directory");
        Self {
            block_files: Arc::new(Mutex::new(block_files)),
            thread_pool: ThreadPool::new(thread_count),
            output_path,
            next_output_id: Arc::new(Mutex::new(0)),
        }
    }

    fn next_output_id(&self) -> u32 {
        let mut id = self.next_output_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }

    pub fn start_process(&mut self) {
        loop {

           let is_done = {
                let block_files = self.block_files.lock().unwrap();
                self.thread_pool.get_active_worker_count() == 0 && block_files.len() == 1
            };

            if is_done {
                break;
            }

            let files = {
                let mut block_files = self.block_files.lock().unwrap();
                if block_files.len() >= 2 {
                    let file1 = block_files.pop().unwrap();
                    let file2 = block_files.pop().unwrap();
                    Some((file1, file2))
                } else {
                    None
                }
            };
            let Some((file1, file2)) = files else {
                 std::thread::yield_now();
                continue;
            };

            let output_id = self.next_output_id();
            let output_path = self.output_path.clone();
            let block_files = Arc::clone(&self.block_files);

            if self.thread_pool.submit( file1.clone(), file2.clone(),
                move |file1, file2,p_id| {
                    match Self::merge_two_blocks( file1, file2, output_path, output_id, p_id) {
                        Ok(path) => Some(path),
                        Err(err) => {
                            eprintln!("Worker {p_id} failed: {err}");
                            None
                        }
                    }
                },
                move |output_path| {
                    let mut block_files = block_files.lock().unwrap();
                    block_files.push(output_path); // again push back to blockfile
                },
            ) == false{
                //somthing goes wrong so again send back
                let mut block_files = self.block_files.lock().unwrap();
                block_files.push(file1);
                block_files.push(file2);
            };

        }
    }

    pub fn merge_two_blocks(left_path: PathBuf, right_path: PathBuf, output_path: PathBuf, output_id: u32, p_id:u32) -> io::Result<PathBuf> {
        let mut writer = Merge_Writer::new(&output_path,output_id as usize,p_id)?;

        let mut left = BlockReader::new(&left_path)?;
        let mut right = BlockReader::new(&right_path)?;

        let mut left_entry =  left.next_word()?;
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

        let output_file = writer.finish()?;
        let size = std::fs::metadata(&output_path)?.len();
        std::fs::remove_file(&left_path)?;
        std::fs::remove_file(&right_path)?;
        println!("FILE SIZE After Write = {} bytes {:?}", size, output_path);
        Ok(output_file)
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

