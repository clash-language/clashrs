use std::fs;
// use std::io::prelude::*;
/*
   expected struct `std::string::String`, found enum `std::result::Result`
    found enum `std::result::Result<std::string::String, std::io::Error>`
 */

fn _read_file(filename: &str) -> Result<String,  std::io::Error> {
    fs::read_to_string(String::from(filename))
}

pub fn read_file_to_str(filename: &str) -> String {
    match _read_file(filename) {
        Ok(file_string) => file_string,
        _ => {
            println!("Failed to read the file.");
            String::from("fail")
        }
    }
}

pub fn read_bytes_to_str(source: Vec<u8>) -> String {
    String::from_utf8(source).expect("Found invalid UTF-8") 
}
