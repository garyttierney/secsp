use crate::grammar::error_recovery;
use crate::grammar::items;
use crate::parser::Parser;
use crate::syntax::NodeKind;
use crate::syntax::SyntaxKindClass;
use crate::syntax::TokenKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    BlockLike,
    NotBlockLike,
}

pub(crate) fn parse_block(p: &mut Parser, include_braces: bool) {
    let m = p.mark();

    if include_braces && !p.eat(TokenKind::OpenBrace) {
        p.error("expected open brace");
        m.abandon(p);
        error_recovery::recover_from_item(p);
        return;
    }

    while !p.at(TokenKind::Eof) {
        match p.current() {
            TokenKind::Semicolon => p.bump(),
            TokenKind::CloseBrace if include_braces => {
                break;
            }
            _ => {
                if !items::parse_item(p) {
                    error_recovery::recover_from_item(p);

                    if p.eat(TokenKind::CloseBrace) {
                        break;
                    }
                }
            }
        }
    }

    if include_braces && !p.eat(TokenKind::CloseBrace) {
        p.error("expected closing brace");
    }

    m.complete(p, NodeKind::Block);
}
