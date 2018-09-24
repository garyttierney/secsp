//! This is the parsing backend library for the CSP language.
//!
//! The top level `parse` function can be used to parse a list of
//! statements from an input source implementing `Read`.  The parser combinators
//! that this library is built on are exposed under the `parser` module, and AST types
//! under the `syntax` module.
//!

#![crate_name = "secsp_syntax"]
#![crate_type = "dylib"]
#![feature(nll)]
#![feature(crate_visibility_modifier)]
#![warn(rust_2018_idioms)]
//#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]

extern crate codespan;
extern crate codespan_reporting;
extern crate parking_lot;

pub mod ast;
pub mod codemap;
pub mod diagnostic;
pub mod lex;
pub mod parse;
pub mod session;

mod keywords;
