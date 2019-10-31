use crate::grammar::expr;
use crate::grammar::expr::ExprParseRestriction;
use crate::parser::Parser;
use crate::syntax::{KeywordKind, SyntaxKind};

pub(crate) fn te_rule(p: &mut Parser, kind: KeywordKind) {
    let m = p.mark();
    p.bump_as(kind);

    // Parse the source ID
    if !expr::try_expression(
        p,
        ExprParseRestriction::NO_SECURITY_LITERALS,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return;
    }

    // Parse the target ID.
    if !expr::try_expression(
        p,
        ExprParseRestriction::NO_SECURITY_LITERALS,
        "expected identifier or type expression",
    ) {
        m.abandon(p);
        return;
    }

    // Parse the target class and access vector expression
    p.expect(tok![":"]);

    expr::expression(p, ExprParseRestriction::NAMES_ONLY);

    if !p.at(tok![";"]) {
        expr::expression(p, ExprParseRestriction::NO_SECURITY_LITERALS);
    }

    p.expect(tok![";"]);
    m.complete(p, SyntaxKind::NODE_TE_RULE);
}

pub(crate) fn te_transition(p: &mut Parser, kind: KeywordKind) {
    let m = p.mark();
    p.bump_as(kind);

    let found_src =
        expr::try_expression(p, ExprParseRestriction::NAMES_ONLY, "expected identifier");

    // Only attempt to parse the target if a source expression was found
    if found_src {
        expr::try_expression(p, ExprParseRestriction::NAMES_ONLY, "expected identifier");
    }

    p.expect(tok![":"]);

    expr::expression(p, ExprParseRestriction::NAMES_ONLY);
    expr::expression(p, ExprParseRestriction::NO_SECURITY_LITERALS);

    if !p.at(tok![";"]) && kind == KeywordKind::TypeTransition {
        expr::expression(p, ExprParseRestriction::LITERAL_ONLY);
    }

    p.expect(tok![";"]);
    m.complete(p, SyntaxKind::NODE_TE_TRANSITION);
}
