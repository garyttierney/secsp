#![crate_name = "secsp"]
#![crate_type = "lib"]

#[macro_use]
extern crate nom;

pub mod parser;
pub mod syntax; 

pub struct ParseResult {
    pub statements: Vec<syntax::Statement>,
}

pub fn parse<R: std::io::Read>(input: &mut R) -> ParseResult {
    let mut buffer = vec![];

    input.read_to_end(&mut buffer).unwrap();
    parse_from_slice(&buffer)
}

fn parse_from_slice(input: &[u8]) -> ParseResult {
    let (_, result) = parser::statement_list(input).unwrap();

    ParseResult { statements: result }
}