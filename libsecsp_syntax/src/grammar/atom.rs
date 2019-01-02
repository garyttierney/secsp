use crate::ast::SyntaxKind;
use crate::grammar::expr::expression;
use crate::parser::CompletedMarker;
use crate::parser::CspParser;
use crate::token::Token;
use crate::token::TokenType;

pub fn path_expr(p: &mut CspParser) -> CompletedMarker<SyntaxKind, Token> {
    let m = p.mark();
    p.expect_one_of(vec![TokenType::Dot, TokenType::Name]);

    while p.at(TokenType::Dot) {
        p.bump();
        p.expect(TokenType::Name);
    }

    m.complete(p, SyntaxKind::PathExpr)
}

pub fn list_or_paren_expr(p: &mut CspParser) -> CompletedMarker<SyntaxKind, Token> {
    assert!(p.at(TokenType::OpenParenthesis));

    let m = p.mark();
    p.bump();

    let mut non_empty = false;
    let mut has_comma = false;

    while !p.at(TokenType::Eof) && !p.at(TokenType::CloseParenthesis) {
        // TODO: Validate that we're at a valid expression token.
        non_empty = true;
        expression(p);

        if !p.at(TokenType::CloseParenthesis) {
            has_comma = true;
            p.expect(TokenType::Comma);
        }
    }

    p.expect(TokenType::CloseParenthesis);
    m.complete(
        p,
        if non_empty && !has_comma {
            SyntaxKind::ParenExpr
        } else {
            SyntaxKind::ListExpr
        },
    )
}

pub fn literal_expr(p: &mut CspParser) -> CompletedMarker<SyntaxKind, Token> {
    let m = p.mark();
    p.expect_one_of(vec![TokenType::String, TokenType::Integer]);
    m.complete(p, SyntaxKind::LiteralExpr)
}

pub fn is_at_path_start(p: &CspParser, offset: usize) -> bool {
    let tok = p.nth(offset);
    tok == SyntaxKind::Token(TokenType::Dot) || tok == SyntaxKind::Token(TokenType::Name)
}
