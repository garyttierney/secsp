//! This is the parsing backend library for the CSP language.
//!
//! The top level `parse` function can be used to parse a list of
//! statements from an input source implementing `Read`.  The parser combinators
//! that this library is built on are exposed under the `parser` module, and AST types
//! under the `syntax` module.
//!
//! # Examples
//! ```rust
//! use secsp::syntax::*;
//!
//! let input = b"block abc{}";
//! let parse_result = secsp::parse_from_slice(&input[..]);
//! match parse_result.statements[0] {
//!     Statement::Declaration(Declaration::Block { is_abstract, ref name, ref qualifier, ref statements }) => println!("Parsed block"),
//!     _ => panic!("Didn't find a block!")
//! }
//! ```

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

pub fn parse_from_slice(input: &[u8]) -> ParseResult {
    let (_, result) = parser::statement_list(input).unwrap();

    ParseResult { statements: result }
}