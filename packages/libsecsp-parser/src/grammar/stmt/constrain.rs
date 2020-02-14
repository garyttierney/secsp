use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn constrain(p: &mut ItemParser, kw: KeywordKind) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw)?);

    if !expression(p.inner, ExprContext::NAMED_SET | ExprContext::IDENTIFIER) {
        p.error_check()?;
    }

    p.expect(tok!["("])?;

    if !expression(p.inner, ExprContext::BIN_EXPR) {
        p.error_check()?;
    }

    p.expect(tok![")"])?;
    p.expect(tok![";"])?;

    Ok(())
}
