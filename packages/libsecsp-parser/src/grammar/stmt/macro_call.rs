use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn macro_call(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat(tok!["("])?);

    let m = p.mark();
    while !p.at(tok![")"]) && !p.at(TOK_EOF) {
        if !expression(p.inner, ExprContext::all()) {
            p.error_check()?;
            break;
        }

        p.eat(tok![","])?;
    }
    m.complete(p.inner, NODE_MACRO_ARGUMENT_LIST);

    p.expect(tok![")"])?;
    p.expect(tok![";"])?;

    Ok(())
}
