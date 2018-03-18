//! This is the parsing backend library for the CSP language.
//!
//! The top level `parse` function can be used to parse a list of
//! statements from an input source implementing `Read`.  The parser combinators
//! that this library is built on are exposed under the `parser` module, and AST types
//! under the `syntax` module.
//!

#![crate_name = "secsp_syntax"]
#![crate_type = "lib"]

/// A structure representing the start and end byte positions of a token
/// or span of code.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// The byte position of the starting character in this `Span`.
    pub start: usize,

    /// The byte position of the ending character in this `Span`.
    pub end: usize,
}

pub mod ast;
pub mod codemap;
pub mod lex;

// #[cfg(test)]
// mod testing {
//     use super::*;

//     pub fn parse<O, P>(input: &str, parser: P) -> O
//     where
//         P: Fn(&[u8]) -> nom::IResult<&[u8], O>,
//     {
//         let bytes = input.as_bytes();
//         let result = parser(bytes);

//         match result {
//             IResult::Done(remaining, output) => output,
//             IResult::Incomplete(e) => panic!("{:?}", e),
//             IResult::Error(e) => panic!("{}", e),
//             _ => panic!("Invalid"),
//         }
//     }
// }
