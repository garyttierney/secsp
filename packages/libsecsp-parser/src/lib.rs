#![allow(unused)]
#![deny(unused_imports)]
extern crate rowan;

use crate::parser::event;
use crate::parser::Parser;
use crate::syntax::SyntaxKind;

mod grammar;
mod parser;
pub mod syntax;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(String);

pub trait TokenSource {
    fn kind(&self, idx: usize) -> SyntaxKind;

    fn text(&self, idx: usize) -> &str;
}

pub trait TreeSink {
    fn error(&mut self, error: ParseError);

    fn start_node(&mut self, ty: SyntaxKind);

    fn finish_node(&mut self);

    fn token(&mut self, ty: SyntaxKind);
}

fn parse_with<P>(source: &dyn TokenSource, sink: &mut dyn TreeSink, parse_fn: P)
where
    P: FnOnce(&mut Parser),
{
    let mut parser = Parser::new(source);
    parse_fn(&mut parser);

    let events = parser.finish();
    event::process(sink, events);
}

pub fn parse_file(source: &dyn TokenSource, sink: &mut dyn TreeSink) {
    parse_with(source, sink, grammar::root)
}
