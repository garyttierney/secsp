//! This is the parsing backend library for the CSP language.
//!
//! The top level `parse` function can be used to parse a list of
//! statements from an input source implementing `Read`.  The parser combinators
//! that this library is built on are exposed under the `parser` module, and AST types
//! under the `syntax` module.
//!
//! # Examples
//! ```rust
//! use secsp::*;
//!
//! let input = b"block abc{}";
//! match parse_from_slice(&input[..]) {
//!     ParseResult::Ok(statements) => {},
//!     _ => {}
//! }
//! ```

#![crate_name = "secsp"]
#![crate_type = "lib"]

#[macro_use]
extern crate nom;

pub mod ast;
pub use ast::*;

mod expr;
mod name;
mod security_attributes;
mod parser;

use nom::{Err as NomErr, ErrorKind, IResult, Needed};
use std::str::from_utf8_unchecked;

/// A parse error. It contains an `ErrorKind` along with a `String` giving information on the reason
/// why the parser failed.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    pub kind: ErrorKind,
    pub info: String,
}

/// Parse result. It can either be parsed, incomplete or errored.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseResult {
    /// The source was successfully parsed.
    Ok(Vec<Statement>),
    /// The parser failed with a `ParseError`.
    Err(ParseError),
    /// More data is required to go on.
    Incomplete(Needed),
}


pub fn parse<R: AsMut<std::io::Read>>(input: &mut R) -> ParseResult {
    let source = input.as_mut();
    let mut buffer = vec![];

    source.read_to_end(&mut buffer).unwrap();
    parse_from_slice(&buffer)
}

pub fn parse_from_slice(input: &[u8]) -> ParseResult {
    match parser::statement_list(input) {
        IResult::Done(i, x) => {
            if i.is_empty() {
                ParseResult::Ok(x)
            } else {
                let kind = ErrorKind::Custom(0); // FIXME: use our own error kind
                let msg = unsafe { from_utf8_unchecked(i).to_owned() };
                let info = msg.lines().next().unwrap_or("").to_owned();
                ParseResult::Err(ParseError { kind, info })
            }
        }
        IResult::Error(err) => {
            match err {
                NomErr::Code(k) => ParseResult::Err(ParseError {
                    kind: k,
                    info: String::new(),
                }),
                NomErr::Node(kind, trace) => {
                    let info = format!("{:#?}", trace);
                    ParseResult::Err(ParseError { kind, info })
                }
                NomErr::Position(kind, p) => {
                    let msg = unsafe { from_utf8_unchecked(p).to_owned() };
                    let info = msg.lines().next().unwrap_or("").to_owned();

                    ParseResult::Err(ParseError { kind, info })
                }
                NomErr::NodePosition(kind, p, trace) => {
                    let p_msg = unsafe { from_utf8_unchecked(p) };
                    let info = format!("{}: {:#?}", p_msg, trace);

                    ParseResult::Err(ParseError { kind, info })
                }
            }
        }
        IResult::Incomplete(n) => ParseResult::Incomplete(n),
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    pub fn parse<O, P>(input: &str, parser: P) -> O
    where
        P: Fn(&[u8]) -> nom::IResult<&[u8], O>,
    {
        let bytes = input.as_bytes();
        let result = parser(bytes);

        match result {
           IResult::Done(remaining, output) => {
               output
           },
           IResult::Incomplete(e) => panic!("{:?}", e),
           IResult::Error(e) => panic!("{}", e),
            _ => panic!("Invalid")
        }
    }
}