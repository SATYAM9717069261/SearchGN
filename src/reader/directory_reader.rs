use crate::indexer::inverted_index::InvertedIndex;
use crate::reader::file_reader::{insert_token};
use std::{fs,io};

//Result->OK(),Error, Option -> Some(), None

pub fn read_dir(path:String,index:&mut InvertedIndex) -> Result<&'static str,io::Error>{
    let mut dir_iter = fs::read_dir(path).expect("Dir Error");

    while let Some(entry) = dir_iter.next(){
        match entry{
            Ok(file) => {
                match insert_token(&file.path(),index){
                    Ok(()) => {
                        index.add_doument_process_count();
                        println!("SUCCESS :: {:?}",file.path());
                    },
                    Err(e) => {
                        println!("ERROR :: Can't read file {:?} : {:?}",file.path(),e);
                    }
                }
            },
            Err(_) =>{ }
        }
    }
    Ok("Sucessfully Created Inverted Index")
}
