mod lexer;

use std::env;
use std::process;
use std::fs::read_to_string;
use clap::{ App, Arg };
use crate::lexer::lex;
use crate::lexer::Token;

pub const VERSION: &str = "0.0.0-alpha.1";
pub const AUTHOR: &str = "Aidan Heller <asfheller@gmail.com>";
pub const DESCRIPTION: &str = "The meta programming language";

fn main() {
    let matches = App::new("lemon")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .takes_value(true)
            .about("File to run")
        )
        .get_matches();

    let path = matches.value_of("file").unwrap_or_else(|| {
        println!("The '--file' argument must be specified.");
        process::exit(1);
    });
    let input = read_to_string(path).unwrap_or_else(|e| {
        println!("The specified file could not be read.");
        process::exit(1);
    });

    let tokens = lex(input.as_str());
    for token in tokens {
        println!("{:?}", token);
    }
}
