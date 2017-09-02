#![crate_name = "csp"]
#![crate_type = "lib"]
#![warn(missing_docs)]

#[macro_use]
extern crate nom;

pub mod parser;
pub mod syntax;