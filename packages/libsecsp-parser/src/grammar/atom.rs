use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::{SyntaxKind, TokenKind};

pub(crate) fn path_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();

    if p.at(TokenKind::Dot) {
        p.bump();
    }

    p.expect(TokenKind::Name);

    while p.at(TokenKind::Dot) {
        p.bump();
        p.expect(TokenKind::Name);
    }

    m.complete(p, SyntaxKind::NODE_PATH_EXPR)
}

pub(crate) fn list_or_paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::OpenParenthesis));

    let m = p.mark();
    p.bump();

    let mut non_empty = false;
    let mut has_comma = false;

    while !p.at(TokenKind::Eof) && !p.at(TokenKind::CloseParenthesis) {
        // TODO: Validate that we're at a valid expression token.
        non_empty = true;
        expression(p, ExprRestriction::NoContext);

        if !p.at(TokenKind::CloseParenthesis) {
            has_comma = true;
            p.expect(TokenKind::Comma);
        }
    }

    p.expect(TokenKind::CloseParenthesis);
    m.complete(
        p,
        if non_empty && !has_comma {
            SyntaxKind::NODE_PAREN_EXPR
        } else {
            SyntaxKind::NODE_LIST_EXPR
        },
    )
}

pub(crate) fn context_expr(p: &mut Parser, lhs: CompletedMarker) -> bool {
    assert!(p.at(TokenKind::Colon));
    p.bump();

    // The `:category` part of a level expression, or the `:role` part of a context expression.
    if !expression(p, ExprRestriction::NoContext) {
        return false;
    }

    if p.eat(TokenKind::Colon) {
        let m = lhs.precede(p);

        // :type
        if !expression(p, ExprRestriction::NoContext) {
            m.abandon(p);
            return false;
        }

        // optional (:mls)
        let successful = if p.eat(TokenKind::Colon) {
            expression(p, ExprRestriction::None)
        } else {
            true
        };

        m.complete(p, SyntaxKind::NODE_CONTEXT_EXPR);
        successful
    } else if p.at(TokenKind::Hyphen) {
        // Just parsed a sensitivity:category literal and are at a hyphen,
        // so we must be at the start of a level-range expression.
        range_expr(p, lhs, SyntaxKind::NODE_LEVEL_RANGE_EXPR)
    } else {
        let m = lhs.precede(p);
        m.complete(p, SyntaxKind::NODE_LEVEL_EXPR);
        true
    }
}

pub(crate) fn range_expr(p: &mut Parser, lhs: CompletedMarker, kind: SyntaxKind) -> bool {
    let m = lhs.precede(p);
    let expected = match kind {
        SyntaxKind::NODE_LEVEL_RANGE_EXPR => TokenKind::Hyphen,
        SyntaxKind::NODE_CATEGORY_RANGE_EXPR => TokenKind::DotDot,
        _ => unreachable!(),
    };

    p.expect(expected);
    let successful = expression(p, ExprRestriction::NoRange);

    m.complete(p, kind);
    successful
}

pub(crate) fn literal_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();
    p.expect_one_of(vec![SyntaxKind::TOK_STRING, SyntaxKind::TOK_INTEGER]);
    m.complete(p, SyntaxKind::NODE_LITERAL_EXPR)
}

pub(crate) fn is_at_path_start(p: &Parser, offset: usize) -> bool {
    let tok: SyntaxKind = p.nth(offset);

    tok == SyntaxKind::TOK_DOT || tok == SyntaxKind::TOK_NAME
}
