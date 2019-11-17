//! Conditional statements are structured imperatively, and allow building conditional policy
//! based on tunables and booleans. Parenthesis in conditional expressions are optional.

use crate::grammar::block::parse_block;
use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind;

pub(crate) fn conditional(p: &mut ItemParser) -> Result<(), ItemParseError> {
    assert!(p.eat(tok!["if"])?);

    expression(p.inner, ExprContext::BIN_EXPR);
    parse_block(p.inner);

    if p.eat(tok!["else"])? {
        if p.at(tok!["if"]) {
            p.try_parse(SyntaxKind::NODE_CONDITIONAL_STMT, conditional);
        } else {
            parse_block(p.inner);
        }
    }

    Ok(())
}
