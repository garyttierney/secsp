use crate::ast::SyntaxKind;
use crate::grammar::error_recovery;
use crate::grammar::items;
use crate::parser::CspParser;
use crate::token::TokenType;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    BlockLike,
    NotBlockLike,
}

pub fn parse_block(p: &mut CspParser, include_braces: bool) {
    let m = p.mark();

    if include_braces && !p.eat(TokenType::OpenBrace) {
        p.error("expected open brace");
        m.abandon(p);
        return;
    }

    while !p.at(TokenType::Eof) {
        match p.current() {
            SyntaxKind::Token(TokenType::Semicolon) => p.bump(),
            SyntaxKind::Token(TokenType::CloseBrace) if include_braces => {
                break;
            }
            _ => {
                if !items::parse_item(p) {
                    error_recovery::recover_from_item(p);

                    if p.eat(TokenType::CloseBrace) {
                        break;
                    }
                }
            }
        }
    }

    if include_braces && !p.eat(TokenType::CloseBrace) {
        p.error("expected closing brace");
    }

    m.complete(p, SyntaxKind::Block);
}
