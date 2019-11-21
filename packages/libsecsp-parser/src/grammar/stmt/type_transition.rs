use crate::grammar::expr;
use crate::grammar::expr::ExprContext;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;

pub(crate) fn type_transition(p: &mut ItemParser, kind: KeywordKind) -> Result<(), ItemParseError> {
    p.bump_as(kind.into());

    let found_src = expr::try_expression(p.inner, ExprContext::NAMES_ONLY, "expected identifier");

    // Only attempt to parse the target if a source expression was found
    if found_src {
        expr::try_expression(p.inner, ExprContext::NAMES_ONLY, "expected identifier");
    }

    p.expect(tok![":"])?;

    if !expr::expression(p.inner, ExprContext::IDENTIFIER) {
        p.error_check()?;
    }

    if !expr::expression(p.inner, ExprContext::IDENTIFIER) {
        p.error_check()?;
    }

    if !p.at(tok![";"]) && kind == kw!["type_transition"] {
        expr::expression(p.inner, ExprContext::LITERAL_ONLY);
    }

    p.expect(tok![";"])?;

    Ok(())
}
