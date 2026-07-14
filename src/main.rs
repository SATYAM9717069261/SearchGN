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
    if let Err(err) = read_dir(path.to_string(),&mut inverted_idx){
        println!("{:?}",err);
        return ExitCode::FAILURE;
    }
    println!(" {:?}",inverted_idx);
    ExitCode::SUCCESS
}
