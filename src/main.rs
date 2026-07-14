mod reader;
mod indexer;
mod models;
mod error;



use reader::directory_reader::{read_dir};
use indexer::inverted_index::{InvertedIndex};
use std::process::ExitCode;

const path:&str = "./dataSet/";

fn main() -> ExitCode {
    let mut inverted_idx:InvertedIndex = InvertedIndex::new();
    read_dir(path.to_string(),&mut inverted_idx);
    println!(" {:?}",inverted_idx);
    ExitCode::SUCCESS
}
