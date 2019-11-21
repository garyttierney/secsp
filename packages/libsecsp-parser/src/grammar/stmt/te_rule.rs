use crate::grammar::expr;
use crate::grammar::expr::ExprContext;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;

pub(crate) fn te_rule(p: &mut ItemParser, kind: KeywordKind) -> Result<(), ItemParseError> {
    p.bump_as(kind.into());

    // Parse the source ID
    if !expr::try_expression(
        p.inner,
        ExprContext::NAMES_ONLY,
        "expected identifier or type expression",
    ) {
        return Ok(());
    }

    // Parse the target ID.
    if !expr::try_expression(
        p.inner,
        ExprContext::NAMES_ONLY,
        "expected identifier or type expression",
    ) {
        return Ok(());
    }

    // Parse the target class and access vector expression
    p.expect(tok![":"])?;

    if !expr::expression(p.inner, ExprContext::IDENTIFIER & ExprContext::NAMED_SET) {
        p.error_check()?;
    }

    p.expect(tok![";"])?;

    Ok(())
}
