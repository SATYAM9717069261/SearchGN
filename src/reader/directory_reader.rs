use crate::spimi::manager::SPIMIManager;
use crate::reader::file_reader::{insert_token};
use std::{fs,io};

//Result->OK(),Error, Option -> Some(), None

pub fn read_dir(path:String,spimi_manager:&mut SPIMIManager) -> Result<&'static str,io::Error>{
    let mut dir_iter = fs::read_dir(path).expect("Dir Error");

    while let Some(entry) = dir_iter.next(){
        match entry{
            Ok(file) => {
                match insert_token(&file.path(),spimi_manager){
                    Ok(()) => {
                        spimi_manager.add_doument_process_count();
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
