use std::str::FromStr;

use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::Parser;
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn variable(p: &mut Parser) {
    let m = p.mark();

    let kw = KeywordKind::from_str(p.current_text()).expect("should be at var type keyword");
    assert!(kw.is_var_type());

    p.bump_as(kw);
    p.expect(TOK_NAME);

    if p.eat(tok!["="]) {
        expression(p, ExprRestriction::None);
    }

    p.expect(tok![";"]);
    m.complete(p, NODE_VARIABLE_DEF);
}
