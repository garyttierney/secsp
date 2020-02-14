use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn netifcon(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["netifcon"])?);

    p.expect(TOK_NAME)?;

    if !expression(p.inner, ExprContext::CONTEXT | ExprContext::IDENTIFIER) {
        p.error_check()?;
    }

    if !expression(p.inner, ExprContext::CONTEXT | ExprContext::IDENTIFIER) {
        p.error_check()?;
    }

    p.expect(tok![";"])?;

    Ok(())
}
