use crate::parser::Parser;
use crate::syntax::SyntaxKind;

pub(crate) mod atom;
pub(crate) mod block;
pub(crate) mod container;
pub(crate) mod error_recovery;
pub(crate) mod expr;
pub(crate) mod items;
pub(crate) mod macros;
pub(crate) mod stmt;
pub(crate) mod var;

pub(super) fn root(p: &mut Parser) {
    let m = p.mark();
    self::block::parse_block(p, false);
    m.complete(p, SyntaxKind::NODE_SOURCE_FILE);
}
