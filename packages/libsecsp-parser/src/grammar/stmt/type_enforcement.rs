use crate::grammar::expr;
use crate::grammar::expr::ExprRestriction;
use crate::parser::Parser;
use crate::syntax::{KeywordKind, SyntaxKind, TokenKind};

pub(crate) fn te_rule(p: &mut Parser, kind: KeywordKind) {
    let m = p.mark();
    p.bump_as(kind);

    // Parse the source ID
    if !expr::try_expression(
        p,
        ExprRestriction::NoContext,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return;
    }

    // Parse the target ID.
    if !expr::try_expression(
        p,
        ExprRestriction::NoContext,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return;
    }

    // Parse the target class and access vector expression
    p.expect(TokenKind::Colon);

    if !expr::expression(p, ExprRestriction::AccessVector) {}

    p.expect(TokenKind::Semicolon);
    m.complete(p, SyntaxKind::NODE_TE_RULE);
}
