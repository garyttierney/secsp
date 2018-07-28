extern crate symbolic_expressions;

use self::symbolic_expressions::parser::parse_file;
use self::symbolic_expressions::Sexp;

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut first: Sexp;
    if let Some(first_file) = std::env::args().nth(1) {
        first =
            parse_file(&first_file).unwrap_or_else(|e| panic!("Unable to parse first file: {}", e));
    } else {
        panic!("Expected 2 files");
    }

    let mut second: Sexp;
    if let Some(second_file) = std::env::args().nth(2) {
        second = parse_file(&second_file)
            .unwrap_or_else(|e| panic!("Unable to parse second file: {}", e));
    } else {
        panic!("Expected 2 files");
    }

    if first == second {
        std::process::exit(0);
    } else {
        println!("First: {}", first);
        println!("Second: {}", second);
        std::process::exit(1);
    }
}
