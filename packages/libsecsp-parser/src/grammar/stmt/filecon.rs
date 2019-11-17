use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn filecon(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["filecon"])?);

    if p.at(tok!["{"]) {
        filecon_list(p)?;
    } else {
        let m = p.mark();

        match filecon_fragment(p) {
            Err(e) => {
                m.abandon(p.inner);
                return Err(e);
            }
            Ok(_) => {}
        };

        m.complete(p.inner, NODE_FILE_CONTEXT_FRAGMENT);
    }

    Ok(())
}

fn filecon_list(p: &mut ItemParser) -> Result<(), ItemParseError> {
    p.expect(tok!["{"])?;

    while !p.at(tok!["}"]) {
        let m = p.mark();

        match filecon_fragment(p) {
            Err(e) => {
                m.abandon(p.inner);
                return Err(e);
            }
            Ok(_) => {}
        };

        m.complete(p.inner, NODE_FILE_CONTEXT_FRAGMENT);
    }

    p.expect(tok!["}"])?;

    Ok(())
}

fn filecon_fragment(p: &mut ItemParser) -> Result<(), ItemParseError> {
    if !expression(p.inner, ExprContext::LITERAL) {
        p.error_check()?;
    }

    if !expression(p.inner, ExprContext::LITERAL & ExprContext::CONTEXT) {
        p.error_check()?;
    }

    if !expression(p.inner, ExprContext::CONTEXT) {
        p.error_check()?;
    }

    if !p.at(tok![";"]) {
        expression(p.inner, ExprContext::CONTEXT);
    }

    p.expect(tok![";"])?;

    Ok(())
}
