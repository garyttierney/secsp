use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

#[macro_use]
pub(crate) mod rules;

pub(crate) mod atom;
pub(crate) mod block;
pub(crate) mod def;
pub(crate) mod error_recovery;
pub(crate) mod expr;
pub(crate) mod items;
pub(crate) mod stmt;

pub(super) fn root(p: &mut Parser) {
    let (m1, m2) = (p.mark(), p.mark());

    while !p.at(TOK_EOF) {
        match p.current() {
            tok![;] => p.bump(),
            _ => {
                items::parse_item(p);
            }
        };
    }

    m2.complete(p, NODE_BLOCK);
    m1.complete(p, NODE_SOURCE_FILE);
}
