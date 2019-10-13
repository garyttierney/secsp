use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::{CompletedMarker, Parser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn macro_call(p: &mut Parser, lhs: CompletedMarker) {
    let m = lhs.precede(p);

    assert!(p.eat(tok!["("]));

    while !p.at(tok![")"]) && !p.at(TOK_EOF) {
        if !expression(p, ExprRestriction::None) {
            break;
        }

        p.eat(tok![","]);
    }

    p.expect(tok![")"]);
    p.expect(tok![";"]);

    m.complete(p, NODE_MACRO_CALL);
}
