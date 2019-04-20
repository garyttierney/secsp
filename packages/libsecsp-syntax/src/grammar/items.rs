use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block::BlockType;
use crate::grammar::{
    container::parse_container, macros::parse_macro, stmt::statement, var::parse_var,
};
use crate::parser::syntax::KeywordKind;
use crate::parser::syntax::NodeKind;
use crate::parser::syntax::TokenKind;
use crate::parser::CspParser;

pub fn parse_item(p: &mut CspParser) -> bool {
    if !p.at_kw() {
        p.error("expected keyword");
        return false;
    }

    fn do_parse_item(
        p: &mut CspParser,
        ty: BlockType,
        kind: NodeKind,
        parser: fn(&mut CspParser),
    ) -> Option<(BlockType, NodeKind)> {
        parser(p);
        Some((ty, kind))
    }

    let m = p.mark();

    let item = match KeywordKind::from_str(p.current_text()).ok() {
        Some(KeywordKind::ABSTRACT)
        | Some(KeywordKind::BLOCK)
        | Some(KeywordKind::OPTIONAL)
        | Some(KeywordKind::IN) => do_parse_item(
            p,
            BlockType::BlockLike,
            NodeKind::Container,
            parse_container,
        ),
        Some(KeywordKind::MACRO) => {
            do_parse_item(p, BlockType::BlockLike, NodeKind::MacroDef, parse_macro)
        }
        Some(kw) if kw.is_var_type() && atom::is_at_path_start(p, 1) => {
            do_parse_item(p, BlockType::NotBlockLike, NodeKind::Variable, parse_var)
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
