pub mod search;

use std::{env, fs};

use practice_task3::cli::arguments::{ArgParser, ParseArg};
use search::{WordSearch, SearchWord};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg_parser=ArgParser::new();
    for i in args  {
        let parseResult=arg_parser.add_argument(i);
        match parseResult {
            Ok(_)=>continue,
            Err(err)=>
            {
                panic!("Wrong Arg")
        }
        }
    }   

    let config=arg_parser.parse();

    let content=fs::read_to_string(&config.input_file)
    .expect("read file error");

    let mut search_word=WordSearch::new();

    search_word.add_content(&content);

    let count=search_word.get_count(&config.word);

}
