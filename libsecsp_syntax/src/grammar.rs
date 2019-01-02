use crate::ast::SyntaxKind;
use crate::parser::CspParser;

pub(crate) mod atom;
pub(crate) mod block;
pub(crate) mod container;
pub(crate) mod error_recovery;
pub(crate) mod expr;
pub(crate) mod items;
pub(crate) mod macros;
pub(crate) mod stmt;
pub(crate) mod var;

#[cfg(test)]
mod test;

pub fn root(p: &mut CspParser) {
    let m = p.mark();
    self::block::parse_block(p, false);
    m.complete(p, SyntaxKind::SourceFile);
}
