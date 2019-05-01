use crate::grammar::atom;
use crate::grammar::block::parse_block;
use crate::grammar::block::BlockType;
use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::Parser;
use crate::syntax::NodeKind;
use crate::syntax::SyntaxKindClass;
use crate::syntax::TokenKind;

pub(crate) fn statement(p: &mut Parser) -> bool {
    if p.at(TokenKind::IfKw) {
        conditional(p);
        return true;
    } else if !atom::is_at_path_start(p, 0) {
        p.error("expected identifier");
        return false;
    }

    let m = atom::path_expr(p).precede(p);

    let (block_type, kind) = match p.current() {
        TokenKind::OpenParenthesis => {
            macro_call(p);
            (BlockType::NotBlockLike, NodeKind::MacroCall)
        }
        TokenKind::IfKw => {
            conditional(p);
            m.abandon(p);
            return true;
        }
        _ => {
            m.complete(p, NodeKind::ParseError);
            return false;
        }
    };

    if block_type == BlockType::NotBlockLike {
        p.expect(TokenKind::Semicolon);
    } else {
        p.eat(TokenKind::Semicolon);
    }

    m.complete(p, kind);
    true
}

fn conditional(p: &mut Parser) {
    let m = p.mark();
    assert!(p.eat(TokenKind::IfKw));

    expression(p, ExprRestriction::NoContext);
    parse_block(p, true);

    if p.eat(TokenKind::ElseKw) {
        if p.at(TokenKind::IfKw) {
            conditional(p);
        } else {
            parse_block(p, true);
        }
    }

    m.complete(p, NodeKind::ConditionalStmt);
}

fn macro_call(p: &mut Parser) {
    assert!(p.eat(TokenKind::OpenParenthesis));

    while !p.at(TokenKind::CloseParenthesis) {
        if !expression(p, ExprRestriction::None) {
            break;
        }

        p.eat(TokenKind::Comma);
    }

    p.expect(TokenKind::CloseParenthesis);
}
