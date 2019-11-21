use crate::grammar::atom;
use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

pub(crate) fn class_mapping(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["class_mapping"])?);

    atom::path_expr(p.inner);
    p.expect(TOK_NAME)?;

    expression(p.inner, ExprContext::NAMED_SET & ExprContext::IDENTIFIER);
    p.expect(tok![";"])?;

    Ok(())
}
