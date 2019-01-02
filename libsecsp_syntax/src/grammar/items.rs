use crate::ast::keywords::Keyword;
use crate::ast::SyntaxKind;
use crate::grammar::atom;
use crate::grammar::block::BlockType;
use crate::grammar::{
    container::parse_container, macros::parse_macro, stmt::parse_stmt, var::parse_var,
};
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn parse_item(p: &mut CspParser) -> bool {
    if !p.at(TokenType::Name) {
        p.error("expected keyword");
        return false;
    }

    fn do_parse_item(
        p: &mut CspParser,
        ty: BlockType,
        kind: SyntaxKind,
        parser: fn(&mut CspParser),
    ) -> Option<(BlockType, SyntaxKind)> {
        parser(p);
        Some((ty, kind))
    }

    let m = p.mark();

    let item = match Keyword::from_str(p.current_text()) {
        Some(Keyword::ABSTRACT)
        | Some(Keyword::BLOCK)
        | Some(Keyword::OPTIONAL)
        | Some(Keyword::IN) => do_parse_item(
            p,
            BlockType::BlockLike,
            SyntaxKind::Container,
            parse_container,
        ),
        Some(Keyword::MACRO) => {
            do_parse_item(p, BlockType::BlockLike, SyntaxKind::MacroDef, parse_macro)
        }
        Some(kw) if kw.is_var_type() && atom::is_at_path_start(p, 1) => {
            do_parse_item(p, BlockType::NotBlockLike, SyntaxKind::Variable, parse_var)
        }
        _ => {
            m.abandon(p);
            return parse_stmt(p);
        }
    };

    match item {
        Some((ty, kind)) => {
            if ty == BlockType::NotBlockLike {
                p.expect(TokenType::Semicolon);
            } else {
                p.eat(TokenType::Semicolon);
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
