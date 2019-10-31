use crate::grammar::expr::{expression, expression_prec, ExprParseRestriction};
use crate::parser::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;
use crate::syntax::{SyntaxKind, TokenKind};

pub(crate) fn path_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();

    if p.at(tok!["."]) {
        p.bump();
    }

    p.expect(TokenKind::Name);

    while p.at(tok!["."]) {
        p.bump();
        p.expect(TokenKind::Name);
    }

    m.complete(p, NODE_PATH_EXPR)
}

pub(crate) fn list_or_paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(tok!["("]));

    let m = p.mark();
    p.bump();

    let mut non_empty = false;
    let mut has_comma = false;

    while !p.at(TOK_EOF) && !p.at(tok![")"]) {
        // TODO: Validate that we're at a valid expression token.
        non_empty = true;
        expression(p, ExprParseRestriction::NO_SECURITY_LITERALS);

        if !p.at(tok![")"]) {
            has_comma = true;
            p.expect(tok![","]);
        }
    }

    p.expect(tok![")"]);
    m.complete(
        p,
        if non_empty && !has_comma {
            NODE_PAREN_EXPR
        } else {
            NODE_LIST_EXPR
        },
    )
}

pub(crate) fn context_expr(p: &mut Parser, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.eat(tok![":"]));

    // The `:category` part of a level expression, or the `:role` part of a context expression.
    if !expression(
        p,
        ExprParseRestriction::NO_CONTEXT | ExprParseRestriction::NO_LEVEL_RANGE,
    ) {
        return lhs;
    }

    if p.eat(tok![":"]) {
        let m = lhs.precede(p);

        // :type
        if !expression(p, ExprParseRestriction::NO_SECURITY_LITERALS) {
            return m.complete(p, NODE_CONTEXT_EXPR);
        }

        // optional (:mls)
        if p.eat(tok![":"]) {
            expression(p, ExprParseRestriction::empty())
        } else {
            return m.complete(p, NODE_CONTEXT_EXPR);
        };

        m.complete(p, NODE_CONTEXT_EXPR)
    } else if p.at(tok!["-"]) {
        // Just parsed a sensitivity:category literal and are at a hyphen,
        // so we must be at the start of a level-range expression.
        range_expr(p, lhs, NODE_LEVEL_RANGE_EXPR)
    } else {
        let m = lhs.precede(p);
        m.complete(p, NODE_LEVEL_EXPR)
    }
}

pub(crate) fn range_expr(
    p: &mut Parser,
    lhs: CompletedMarker,
    kind: SyntaxKind,
) -> CompletedMarker {
    let m = lhs.precede(p);
    let expected = match kind {
        NODE_LEVEL_RANGE_EXPR => TokenKind::Hyphen,
        NODE_CATEGORY_RANGE_EXPR => TokenKind::DotDot,
        _ => unreachable!(),
    };

    p.expect(expected);
    let successful = expression(
        p,
        ExprParseRestriction::NO_LEVEL_RANGE | ExprParseRestriction::NO_CATEGORY_RANGE,
    );

    m.complete(p, kind)
}

pub(crate) fn literal_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();
    p.expect_one_of(vec![TOK_STRING, TOK_INTEGER]);
    m.complete(p, NODE_LITERAL_EXPR)
}

pub(crate) fn is_at_path_start(p: &Parser, offset: usize) -> bool {
    let tok: SyntaxKind = p.nth(offset);

    tok == tok!["."] || tok == TOK_NAME
}

pub(crate) fn set_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();
    assert!(p.eat(tok!["{"]));

    while !p.at_end(tok!["}"]) {
        if !expression(p, ExprParseRestriction::NO_CONTEXT) {
            break;
        }
    }

    p.expect(tok!["}"]);
    m.complete(p, NODE_SET_EXPR)
}

pub(crate) fn parse_extends_list(p: &mut Parser) {
    let m = p.mark();

    assert!(p.eat_keyword(kw!["extends"]));
    path_expr(p);

    while p.eat(tok![","]) {
        path_expr(p);
    }

    m.complete(p, SyntaxKind::NODE_EXTENDS_LIST);
}

pub(crate) fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.mark();
    p.bump();
    expression_prec(p, 255, ExprParseRestriction::empty());
    m.complete(p, NODE_PREFIX_EXPR)
}
