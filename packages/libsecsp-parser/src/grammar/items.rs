use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block::BlockType;
use crate::grammar::{
    container::parse_container,
    macros::parse_macro,
    stmt::{kw_statement, statement},
    var::parse_var,
};
use crate::parser::Parser;
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind;
use crate::syntax::TokenKind;

pub(crate) fn parse_item(p: &mut Parser) -> bool {
    fn at_kw(p: &Parser) -> bool {
        p.at(TokenKind::Name) || p.at(TokenKind::IfKw) || p.at(TokenKind::ElseKw)
    }

    if !at_kw(p) {
        p.error("expected keyword");
        return false;
    }

    fn do_parse_item(
        p: &mut Parser,
        ty: BlockType,
        kind: SyntaxKind,
        parser: fn(&mut Parser),
    ) -> Option<(BlockType, SyntaxKind)> {
        parser(p);
        Some((ty, kind))
    }

    let m = p.mark();

    let item = match KeywordKind::from_str(p.current_text()) {
        Ok(KeywordKind::Abstract)
        | Ok(KeywordKind::Block)
        | Ok(KeywordKind::Optional)
        | Ok(KeywordKind::In) => do_parse_item(
            p,
            BlockType::BlockLike,
            SyntaxKind::NODE_CONTAINER_DEF,
            parse_container,
        ),
        Ok(KeywordKind::Macro) => do_parse_item(
            p,
            BlockType::BlockLike,
            SyntaxKind::NODE_MACRO_DEF,
            parse_macro,
        ),
        Ok(kw) if kw.is_var_type() && atom::is_at_path_start(p, 1) => do_parse_item(
            p,
            BlockType::NotBlockLike,
            SyntaxKind::NODE_VARIABLE_DEF,
            parse_var,
        ),
        Ok(kw) => {
            m.abandon(p);
            return kw_statement(p, kw);
        }
        _ => {
            m.abandon(p);
            return statement(p);
        }
    };

    match item {
        Some((ty, kind)) => {
            if ty == BlockType::NotBlockLike {
                p.expect(TokenKind::Semicolon);
            } else {
                p.eat(TokenKind::Semicolon);
            }

            m.complete(p, kind);
            true
        }
        None => {
            m.abandon(p);
            false
        }
    }
}
