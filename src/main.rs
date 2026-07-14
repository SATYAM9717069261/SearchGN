mod reader;
mod indexer;
mod models;
mod error;



use reader::directory_reader::{read_dir};
use indexer::inverted_index::{InvertedIndex};
use std::process::ExitCode;

const PATH:&str = "./test/";

fn main() -> ExitCode {
    let mut inverted_idx:InvertedIndex = InvertedIndex::new();
    if let Err(err) = read_dir(PATH.to_string(),&mut inverted_idx){
        println!("{:?}",err);
        return ExitCode::FAILURE;
    }
    let json = serde_json::to_string(&inverted_idx).unwrap();
    println!("{}", json);
    ExitCode::SUCCESS
}
