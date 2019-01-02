use crate::ast::SyntaxKind;
use crate::grammar::atom;
use crate::grammar::block::BlockType;
use crate::parser::CspParser;
use crate::token::TokenType;
use crate::grammar::expr::expression;

pub fn parse_stmt(p: &mut CspParser) -> bool {
    if !atom::is_at_path_start(p, 0) {
        p.error("expected identifier");
        return false;
    }

    let m = atom::path_expr(p).precede(p);

    let (block_type, kind) = match p.current() {
        SyntaxKind::Token(TokenType::OpenParenthesis) => {
            parse_macro_call(p);
            (BlockType::NotBlockLike, SyntaxKind::MacroCall)
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

pub fn parse_macro_call(p: &mut CspParser) {
    assert!(p.at(TokenType::OpenParenthesis));
    p.bump();

    while !p.at(TokenType::CloseParenthesis) {
        expression(p);
        p.eat(TokenType::Comma);
    }

    p.expect(TokenType::CloseParenthesis);
}
