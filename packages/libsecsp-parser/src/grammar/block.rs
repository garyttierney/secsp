use crate::grammar::error_recovery;
use crate::grammar::items;
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

pub(crate) fn parse_block(p: &mut Parser) {
    let m = p.mark();

    if !p.eat(tok!["{"]) {
        p.error("expected open brace");
        m.abandon(p);
        error_recovery::recover_from_item(p);
        return;
    }

    while !p.at(TOK_EOF) {
        if p.at(tok!["}"]) {
            break;
        }

        items::parse_item(p);
    }

    p.expect(tok!["}"]);
    m.complete(p, NODE_BLOCK);
}
