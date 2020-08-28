#[macro_use]
use pest;
use pest::{RuleType, Parser};

use std::fs;
use pest::error::Error;
use pest::iterators::Pairs;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

macro_rules! parse {
    ($example_path:expr, $pest_path:expr) => {
        
        #[derive(self::Parser)]
        #[grammar = $pest_path]
        struct ClashParser;
        let clash_str = self::lexer::read_file_to_str($example_path);
        clash_str

    }
}
fn _read_file(filename: &str) -> Result<String,  std::io::Error> {
    fs::read_to_string(String::from(filename))
}

pub fn read_file_to_str(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    s
}

pub fn read_bytes_to_str(source: Vec<u8>) -> String {
    String::from_utf8(source).expect("Found invalid UTF-8") 
}

