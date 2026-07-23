mod reader;
mod indexer;
mod models;
mod query;
mod spimi;

use reader::directory_reader::{read_dir};
use spimi::manager::SPIMIManager;

use query::lexer::Lexer;
use query::parser::parse_token_to_ast;
use query::evaluator::{evaluate, print_posting_list};

use std::process::ExitCode;
use std::io::{stdin};
use std::env;

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/dataSet/");

fn main() -> ExitCode {
    match SPIMIManager::new(
            std::env::temp_dir().join("SearchGN"),
            1000000
        ){
        Ok(mut manager) =>{
            if let Err(err) = read_dir(PATH.to_string(),&mut manager){
                println!("{:?}",err);
                return ExitCode::FAILURE;
            }
        },
        Err(er) => {
            println!("Error: {:?}",er);
        }
    }

    //let json = serde_json::to_string(&inverted_idx).unwrap();
    //inverted_idx.print_debugging_details();

 // loop{
 //     let mut inp:String = String::new();
 //     stdin().read_line(&mut inp).expect("IO Error");
 //     let query = inp.trim().parse::<String>().expect("Parsing Error");
 //     if query == "exit"{
 //         break;
 //     }else{
 //         let lexer = Lexer::new();
 //         if let Ok(rtn) = parse_token_to_ast(&lexer.tokenizer(&query)){
 //             let eval_rtn = evaluate(&rtn,&inverted_idx);
 //             print_posting_list(&eval_rtn,&inverted_idx);
 //         }
 //     }
 // }
    ExitCode::SUCCESS
}
