extern crate drop_bomb;
extern crate itertools;
extern crate logos;
extern crate rowan;
extern crate smol_str;
extern crate text_unit;

pub mod ast;
pub mod parser;

mod grammar;
mod lexer;
mod token;

pub(crate) const TOK_START: u16 = 0;
pub(crate) const KW_KIND_START: u16 = 1_000;
pub(crate) const NODE_KIND_START: u16 = 10_000;
