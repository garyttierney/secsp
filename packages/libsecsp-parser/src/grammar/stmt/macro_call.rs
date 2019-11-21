use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn macro_call(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat(tok!["("])?);

    while !p.at(tok![")"]) && !p.at(TOK_EOF) {
        if !expression(p.inner, ExprContext::empty()) {
            p.error_check()?;
            break;
        }

        p.eat(tok![","])?;
    }

    p.expect(tok![")"])?;
    p.expect(tok![";"])?;

    Ok(())
}
