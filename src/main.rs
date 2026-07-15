mod reader;
mod indexer;
mod models;
mod error;

use reader::directory_reader::{read_dir};
use indexer::inverted_index::{InvertedIndex};
use std::process::ExitCode;
use std::io::{stdin};

const PATH:&str = "./test/";

fn main() -> ExitCode {
    let mut inverted_idx:InvertedIndex = InvertedIndex::new();
    if let Err(err) = read_dir(PATH.to_string(),&mut inverted_idx){
        println!("{:?}",err);
        return ExitCode::FAILURE;
    }
    let json = serde_json::to_string(&inverted_idx).unwrap();
    println!("{}", json);
    loop{
        let mut inp:String = String::new();
        stdin().read_line(&mut inp).expect("IO Error");
        let search_word = inp.trim().parse::<String>().expect("Parsing Error");
        if search_word == "exit"{
            break;
        }else{
            if let Ok(details) = inverted_idx.search_word(&search_word){
                for detail  in details.iter(){
                    let idx = detail.get_document_id();
                    println!(" => {:?}",inverted_idx.get_docuemnt(idx));
                }
            }else{
                println!("{} not found in database",search_word);
            }
        }
    }
    ExitCode::SUCCESS
}
