mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::parser::parse;


fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut file_str = "test.json";
    let file;
    if arg.len() >= 2 {
        file_str = &arg[1];
    }
    let extension = &file_str[file_str.len()-5..file_str.len()];
    if extension != ".json" {
        panic!("`{}` must be in .json file format", file_str);
    }
    match File::open(&file_str) {
        Ok(r) => file = r,
        Err(e) => panic!("Error opening file `{}`: {}", file_str, e),
    }
    let mut br = BufReader::new(file);
    let mut contents = "".to_owned();
    if let Err(e) = br.read_to_string(&mut contents) {
        panic!("Error reading contents of file `{}`: {}", file_str, e);
    }
    let strcontents = contents.as_str();
    parse(strcontents);
}
