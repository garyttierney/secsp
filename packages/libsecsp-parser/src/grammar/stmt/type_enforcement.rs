use crate::grammar::atom;
use crate::grammar::expr;
use crate::grammar::expr::ExprRestriction;
use crate::parser::Parser;
use crate::syntax::{KeywordKind, NodeKind, SyntaxKind, TokenKind};
use core::borrow::Borrow;

pub(super) fn te_rule(p: &mut Parser, kind: KeywordKind) -> bool {
    let mut m = p.mark();
    p.bump_as(kind);

    // Parse the source ID
    if !expr::try_expression(
        p,
        ExprRestriction::None,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return false;
    }

    // Parse the target ID.
    if !expr::try_expression(
        p,
        ExprRestriction::None,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return false;
    }

    // Parse the target class and access vector expression
    p.expect(TokenKind::Colon);

    if !expr::expression(p, ExprRestriction::AccessVector) {
        m.abandon(p);
        return false;
    }

    p.expect(TokenKind::Semicolon);
    m.complete(p, SyntaxKind::NODE_TE_RULE);
    true
}
