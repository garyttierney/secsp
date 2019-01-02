use crate::ast::SyntaxKind;
use crate::grammar::atom;
use crate::grammar::block::parse_block;
use crate::grammar::block::BlockType;
use crate::grammar::expr::expression;
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn statement(p: &mut CspParser) -> bool {
    if p.at(TokenType::IfKw) {
        conditional(p);
        return true;
    } else if !atom::is_at_path_start(p, 0) {
        p.error("expected identifier");
        return false;
    }

    let m = atom::path_expr(p).precede(p);

    let (block_type, kind) = match p.current() {
        SyntaxKind::Token(TokenType::OpenParenthesis) => {
            macro_call(p);
            (BlockType::NotBlockLike, SyntaxKind::MacroCall)
        }
        SyntaxKind::Token(TokenType::IfKw) => {
            conditional(p);
            m.abandon(p);
            return true;
        }
        _ => {
            m.complete(p, SyntaxKind::ParseError);
            return false;
        }
    };

    if block_type == BlockType::NotBlockLike {
        p.expect(TokenType::Semicolon);
    } else {
        p.eat(TokenType::Semicolon);
    }

    m.complete(p, kind);
    true
}

fn conditional(p: &mut CspParser) {
    assert!(p.at(TokenType::IfKw));
    let m = p.mark();
    p.bump();

    expression(p);
    parse_block(p, true);

    if p.at(TokenType::ElseKw) {
        p.bump();
        if p.at(TokenType::IfKw) {
            conditional(p);
        } else {
            parse_block(p, true);
        }
    }

    m.complete(p, SyntaxKind::ConditionalStmt);
}

fn macro_call(p: &mut CspParser) {
    assert!(p.at(TokenType::OpenParenthesis));
    p.bump();

    while !p.at(TokenType::CloseParenthesis) {
        expression(p);
        p.eat(TokenType::Comma);
    }

    p.expect(TokenType::CloseParenthesis);
}

#[test]
fn parse_macro_call() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="MacroCall">macro_name(a && b, "test", 123);</marker>
    "#,
    );
}

#[test]
fn parse_conditional() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="conditionalstmt">if a && b {
        }</marker>
      "#,
    );
}

#[test]
fn parse_conditional_with_else() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="conditionalstmt">if a {
        } else {
        }</marker>
        "#
    )
}
