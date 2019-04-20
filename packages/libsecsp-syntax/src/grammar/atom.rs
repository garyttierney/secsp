use std::convert::TryFrom;

use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::syntax::{NodeKind, TokenKind};
use crate::parser::CompletedMarker;
use crate::parser::CspParser;
use crate::token::Token;

pub fn path_expr(p: &mut CspParser) -> CompletedMarker<Token> {
    let m = p.mark();

    if p.at(TokenKind::Dot) {
        p.bump();
    }

    p.expect(TokenKind::Name);

    while p.at(TokenKind::Dot) {
        p.bump();
        p.expect(TokenKind::Name);
    }

    m.complete(p, NodeKind::PathExpr)
}

pub fn list_or_paren_expr(p: &mut CspParser) -> CompletedMarker<Token> {
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
            NodeKind::ParenExpr
        } else {
            NodeKind::ListExpr
        },
    )
}

pub fn context_expr(p: &mut CspParser, lhs: CompletedMarker<Token>) -> bool {
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

        m.complete(p, NodeKind::ContextExpr);
        successful
    } else if p.at(TokenKind::Hyphen) {
        // Just parsed a sensitivity:category literal and are at a hyphen,
        // so we must be at the start of a level-range expression.
        range_expr(p, lhs, NodeKind::LevelRangeExpr)
    } else {
        let m = lhs.precede(p);
        m.complete(p, NodeKind::LevelExpr);
        true
    }
}

pub fn range_expr(p: &mut CspParser, lhs: CompletedMarker<Token>, kind: NodeKind) -> bool {
    let m = lhs.precede(p);
    let expected = match kind {
        NodeKind::LevelRangeExpr => TokenKind::Hyphen,
        NodeKind::CategoryRangeExpr => TokenKind::DotDot,
        _ => unreachable!(),
    };

    p.expect(expected);
    let successful = expression(p, ExprRestriction::NoRange);

    m.complete(p, kind);
    successful
}

pub fn literal_expr(p: &mut CspParser) -> CompletedMarker<Token> {
    let m = p.mark();
    p.expect_one_of(vec![TokenKind::String, TokenKind::Integer]);
    m.complete(p, NodeKind::LiteralExpr)
}

pub fn is_at_path_start(p: &CspParser, offset: usize) -> bool {
    let tok = TokenKind::try_from(p.nth(offset)).ok();

    tok == Some(TokenKind::Dot) || tok == Some(TokenKind::Name)
}

#[test]
fn parse_global_path() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="pathexpr">.global.item</marker>);
    "#,
    )
}

#[test]
fn parse_nested_path() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="pathexpr">nested1.nested2.nested3</marker>);
    "#,
    )
}

#[test]
fn parse_list_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="listexpr">(item1, item2, item3)</marker>);
    "#,
    )
}

#[test]
fn parse_paren_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="parenexpr">(a && b)</marker>);
    "#,
    )
}

#[test]
fn parse_context_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type</marker>);
    "#,
    )
}

#[test]
fn parse_mls_context_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type:<marker type="levelrangeexpr">s1-s2</marker></marker>);
    "#,
    )
}

#[test]
fn parse_mls_mcs_context_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type:s1:c2..c5-s10:c1</marker>);
    "#,
    )
}

#[test]
fn parse_level_expr() {
    crate::grammar::test::test_parser(
        r#"
        callstub(<marker type="levelexpr">sensitivity:category</marker>);
    "#,
    )
}

#[test]
fn parse_level_range_expr() {
    crate::grammar::test::test_parser(
        r#"
            callstub(<marker type="levelrangeexpr">sensitivity:category-sensitivity2</marker>);
        "#,
    )
}
