use std::io::{self,Read};
use std::fs::{self,File};
use std::process::ExitCode;
use std::collections::HashMap;

const path:&str = "./dataSet/";

#[derive(Debug,Clone)]
struct WordStartAt{
    start_at:Vec<u32>
}
impl WordStartAt{
    fn new() -> Self{
        WordStartAt{
            start_at: Vec::new()
        }
    }
}

#[derive(Debug,Clone)]
struct Posting{
    name:String,
    freq:u32,
    line_no:Vec<u32>,
    start_at: Vec<WordStartAt>,
    /*
     * Postiong{
     *  fileName,
     *  how many time Occure Word(A) in this File
     *  word Occure in Multiple Lines
     *   same Line have mutiple time mentioned Word[
     *              start_at:[1,8,10,100]
     *              start_at:[1,8,10,100]
     *      ]
     * }
     */
}
impl Posting{
    fn new(name:String,freq:u32)->Self{
        Posting{
            name:name,
            freq:freq,
            line_no: Vec::new(),
            start_at: Vec::new()
        }
    }
}
#[derive(Debug)]
struct InvertedIndex{
    map: HashMap<String,Vec<Posting>>
}

impl InvertedIndex{
    fn new()->Self{
        InvertedIndex{
            map: HashMap::new()
        }
    }
    fn insert(&mut self,key:String, posting:&mut Posting, line_number:u32, word_start_at:u32){
        match self.map.get_mut(&key){
            Some(details) => {
             //   todo!();
            },
            None => {
                posting.line_no.push(line_number);
                let mut word_start = WordStartAt::new();
                word_start.start_at = vec![word_start_at];
                posting.start_at.push(word_start);
                self.map.insert(key,vec![posting.clone()]);
            }
        }
    }
}

fn main() -> ExitCode {
    let mut inverted_idx:InvertedIndex = InvertedIndex::new();
    let mut dir_iter = fs::read_dir(path).expect("Dir Error");

    while let Some(entry) = dir_iter.next(){
        match entry{
            Ok(file) => {
                match insert_token(&file.path(),&mut inverted_idx){
                    Ok(()) => {
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
    println!(" {:?}",inverted_idx);
    ExitCode::SUCCESS
}


fn insert_token(file_path:&std::path::Path ,inverted_idx: &mut InvertedIndex)->io::Result<()>{

    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 1024];

    if let Some(f_name)= file_path.to_str(){
        let mut posting = Posting::new(f_name.to_string(),0);
        let mut word:Word = Word::new();

        let mut line_number:u32 = 0;
        let mut bytes_read_count:u32 = 0;
        let mut word_start_at = 0;

        while let Ok(n) = file.read(&mut buffer){
            if n == 0{
                break;
            }

            for byte in &buffer[..n]{
                bytes_read_count+=1;
                match byte{
                    b' ' => {
                        let key = word.get();
                        inverted_idx.insert(key,&mut posting,line_number,word_start_at);
                        word.clear();
                        word_start_at = bytes_read_count+1;
                    },
                    b'\n' => {
                        line_number+=1;
                        // push in hash
                    },
                    _ => {
                        word.push(*byte as char);
                    }
                }
            }
        }
    }
    Ok(())
}



struct Word{
    _word:String
}

impl Word{
    fn new()->Self{
        Word{
            _word:String::from("")
        }
    }
    fn push(&mut self, ch:char){
        // filter pending
        self._word.push(ch);
    }
    fn get(&mut self)->String{
        let tmp = self._word.clone();
        self._word.clear();
        return tmp;
    }
    fn clear(&mut self){
        self._word.clear();
    }
}
