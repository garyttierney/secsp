use crate::grammar::expr::{expression, ExprParseRestriction};
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

pub(crate) fn class_mapping(p: &mut Parser) {
    let m = p.mark();
    assert!(p.eat_keyword(kw!["class_mapping"]));

    p.expect(TOK_NAME);

    if expression(p, ExprParseRestriction::NO_SECURITY_LITERALS) {
        p.expect(tok![";"]);
    }

    m.complete(p, NODE_CLASS_MAPPING);
}
