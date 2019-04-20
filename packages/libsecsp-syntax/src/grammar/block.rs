use std::convert::TryFrom;

use crate::grammar::error_recovery;
use crate::grammar::items;
use crate::parser::syntax::NodeKind;
use crate::parser::syntax::TokenKind;
use crate::parser::CspParser;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    BlockLike,
    NotBlockLike,
}

pub fn parse_block(p: &mut CspParser, include_braces: bool) {
    let m = p.mark();

    if include_braces && !p.eat(TokenKind::OpenBrace) {
        p.error("expected open brace");
        m.abandon(p);
        error_recovery::recover_from_item(p);
        return;
    }

    while !p.at(TokenKind::Eof) {
        match TokenKind::try_from(p.current()).ok() {
            Some(TokenKind::Semicolon) => p.bump(),
            Some(TokenKind::CloseBrace) if include_braces => {
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
