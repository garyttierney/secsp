use crate::grammar::block::parse_block;
use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::Parser;
use crate::syntax::SyntaxKind;

pub(crate) fn conditional(p: &mut Parser) {
    let m = p.mark();
    assert!(p.eat(tok!["if"]));

    expression(p, ExprRestriction::NoContext);
    parse_block(p);

    if p.eat(tok!["else"]) {
        if p.at(tok!["if"]) {
            conditional(p);
        } else {
            parse_block(p);
        }
    }

    m.complete(p, SyntaxKind::NODE_CONDITIONAL_STMT);
}
