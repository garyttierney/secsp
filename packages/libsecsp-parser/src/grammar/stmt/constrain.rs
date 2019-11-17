use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn constrain(p: &mut ItemParser, kw: KeywordKind) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw)?);

    expression(p.inner, ExprContext::NO_SECURITY_LITERALS);
    p.expect(tok!["("]);
    expression(p.inner, ExprContext::NO_SECURITY_LITERALS);
    p.expect(tok![")"]);
    p.expect(tok![";"])?;

    Ok(())
}
