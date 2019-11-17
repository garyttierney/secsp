use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn portcon(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["portcon"])?);

    p.expect(TOK_NAME)?;

    if !expression(p.inner, ExprContext::INT_RANGE) {
        p.error_check()?;
    }

    if !expression(p.inner, ExprContext::CONTEXT) {
        p.error_check()?;
    }

    p.expect(tok![";"])?;

    Ok(())
}
