extern crate itertools;
extern crate logos;
extern crate num_enum;
extern crate rowan;
extern crate secsp_parser;
extern crate secsp_syntax_derive;
extern crate smol_str;
extern crate text_unit;

pub mod ast;

mod parsing;
mod token;

pub use ast::types::SourceFile;
use rowan::{GreenNode, SyntaxNode, TreeArc};
use secsp_parser::ParseError;

impl SourceFile {
    fn new(green: GreenNode, errors: Vec<ParseError>) -> TreeArc<SourceFile> {
        let root = SyntaxNode::new(green, Some(Box::new(errors)));
        TreeArc::cast(root)
    }

    pub fn parse<T: AsRef<str>>(text: T) -> TreeArc<SourceFile> {
        let (green, errors) = parsing::parse_text(text);
        SourceFile::new(green, errors)
    }
}

#[test]
fn parse_source_file() {
    let _ = SourceFile::parse("block abc { type t; }");
}
