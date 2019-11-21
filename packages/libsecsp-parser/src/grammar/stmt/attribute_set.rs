//! The attribute set statement is responsible for modifying existing sets of permissions,
//! types, users, and roles. It can be used to append a simple list of names to the set, or
//! append the result of a complex expression.

use crate::grammar::atom;
use crate::grammar::expr::expression;
use crate::grammar::expr::ExprContext;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::parser::Parser;
use crate::syntax::KeywordKind;

pub(crate) fn attribute_set(p: &mut ItemParser, kind: KeywordKind) -> Result<(), ItemParseError> {
    assert!(p.eat_keyword(kind)?);

    if !atom::is_at_path_start(p.inner, 0) {
        p.error_check()?;
    }

    atom::path_expr(p.inner);
    if !expression(p.inner, ExprContext::BIN_EXPR & ExprContext::NAMES_ONLY) {
        p.error_check()?;
    } // ExprContext::BIN_EXPR | ExprContext::NAMES

    p.expect(tok![";"])?;
    Ok(())
}
