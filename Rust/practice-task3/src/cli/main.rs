pub mod search;

use std::{env, fs};

use practice_task3::cli::arguments::{ArgParser, ParseArg};
use search::{WordSearch, SearchWord};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg_parser=ArgParser::new();
    match arg_parser.add_argument(args[1].clone()){
        Err(err)=>
        panic!("error"),
        Ok(_)=>{}
    }

    match arg_parser.add_argument(args[2].clone()){
        Err(err)=>
        panic!("error"),
        Ok(_)=>{}
    }

    match arg_parser.add_argument(args[3].clone()){
        Err(err)=>
        panic!("error"),
        Ok(_)=>{}
    }

    let config=arg_parser.parse();

    println!("{}", config.input_file);

    let content=fs::read_to_string(&config.input_file)
    .expect("read file error");

    let mut search_word=WordSearch::new();

    search_word.add_content(&content);

    let count=search_word.get_count(&config.word);

    fs::write(&config.output_file, count.to_string())
    .expect("write to file error");

}
