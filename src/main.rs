use std::env;
use std::fs::read_to_string;

mod config;
use crate::config::Config;

mod lexer;
use crate::lexer::lex;
use crate::lexer::Token;

mod parser;

mod eval;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(args);

    let input = read_to_string("test.lm").unwrap();
    // let input = match *read_to_string(config.path) {
    //     Some(T) => T,
    //     None => panic!("File could not be read."),
    // };

    let tokens = lex(input.as_str());


}

