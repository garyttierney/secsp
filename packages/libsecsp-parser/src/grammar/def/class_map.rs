use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

pub(crate) fn class_mapping(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["class_mapping"])?);

    p.expect(TOK_NAME);
    expression(p.inner, ExprContext::NO_SECURITY_LITERALS);
    p.expect(tok![";"])?;

    Ok(())
}
