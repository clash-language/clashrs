mod io;
pub use self::io::{read_file_to_str,read_bytes_to_str};

extern crate pest;

use pest::{RuleType, Parser};
use std::fs;
use pest::error::Error;
use pest::iterators::Pairs;



#[macro_use]
macro_rules! parse {
    ($example_path:expr, $pest_path:expr) => {
        #[derive(Parser)]
        #[grammar = $pest_path]
        pub struct TestParser;
        
        fs::read_to_string("$example_path")
            .expect("Cannot read $example_path to file");
        
        
        let clash_file = TestParser::parse(Rule::file, &unparsed_file)
            .expect("$example_path could not be parsed")
            .next()
            .unwrap();
        clash_file
        
    }
}

