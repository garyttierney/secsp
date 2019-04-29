use std::str::FromStr;

use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::Parser;
use crate::syntax::{KeywordKind, TokenKind};

pub(crate) fn parse_var(p: &mut Parser) {
    let kw = KeywordKind::from_str(p.current_text()).expect("should be at var type keyword");
    assert!(kw.is_var_type());

    p.bump_as(kw);
    p.expect(TokenKind::Name);

    if p.eat(TokenKind::Equals) {
        expression(p, ExprRestriction::None);
    }
}
