mod reader;
mod indexer;
mod models;
mod query;


use reader::directory_reader::{read_dir};
use indexer::inverted_index::{InvertedIndex};

use query::lexer::Lexer;
use query::parser::parse_token_to_ast;
use query::evaluator::{evaluate, print_posting_list};

use std::process::ExitCode;
use std::io::{stdin};
use std::env;

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/");

fn main() -> ExitCode {
    let mut inverted_idx:InvertedIndex = InvertedIndex::new();
    if let Err(err) = read_dir(PATH.to_string(),&mut inverted_idx){
        println!("{:?}",err);
        return ExitCode::FAILURE;
    }
    //let json = serde_json::to_string(&inverted_idx).unwrap();
    inverted_idx.print_debugging_details();

    loop{
        let mut inp:String = String::new();
        stdin().read_line(&mut inp).expect("IO Error");
        let query = inp.trim().parse::<String>().expect("Parsing Error");
        if query == "exit"{
            break;
        }else{
            let lexer = Lexer::new();
            if let Ok(rtn) = parse_token_to_ast(&lexer.tokenizer(&query)){
                let eval_rtn = evaluate(&rtn,&inverted_idx);
                print_posting_list(&eval_rtn,&inverted_idx);
            }
        }
    }
    ExitCode::SUCCESS
}
