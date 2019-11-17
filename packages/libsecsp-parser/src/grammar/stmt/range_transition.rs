use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn range_transition(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kw!["range_transition"])?);

    Ok(())
}
