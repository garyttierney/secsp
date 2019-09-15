use crate::grammar::atom;
use crate::grammar::block::parse_block;
use crate::grammar::block::BlockType;
use crate::grammar::expr::{expression, ExprRestriction};
use crate::parser::Parser;
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind;
use crate::syntax::TokenKind;

mod type_enforcement;

pub(crate) fn statement(p: &mut Parser) -> bool {
    // A statement may only start with a name token  or fully qualified path (or special-cased non-contextual if keyword).
    if p.at(TokenKind::IfKw) {
        conditional(p);
        return true;
    } else if !atom::is_at_path_start(p, 0) {
        p.error("expected identifier");
        return false;
    }

    // Create a syntax marker beginning before the identifier of this potential statement.
    let m = atom::path_expr(p).precede(p);

    // We didn't find a keyword statement, so determine if the current
    // token stream represents a statement that begins with an identifier.
    // e.g.,
    // `macro_call(a);`
    // `my_ident |= val;`
    let (block_type, kind) = match p.current() {
        SyntaxKind::TOK_OPEN_PARENTHESIS => {
            macro_call(p);
            (BlockType::NotBlockLike, SyntaxKind::NODE_MACRO_CALL)
        }
        _ => {
            m.complete(p, SyntaxKind::NODE_PARSE_ERROR);
            return false;
        }
    };

    finish_stmt(p, block_type);
    m.complete(p, kind);
    true
}

pub(crate) fn kw_statement(p: &mut Parser, kind: KeywordKind) -> bool {
    use self::KeywordKind::*;

    match &kind {
        AuditAllow | DontAudit | NeverAllow | Allow => type_enforcement::te_rule(p, kind),
        _ => unimplemented!(),
    }
}

fn finish_stmt(p: &mut Parser, block_type: BlockType) {
    if block_type == BlockType::NotBlockLike {
        p.expect(TokenKind::Semicolon);
    } else {
        p.eat(TokenKind::Semicolon);
    }
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

    m.complete(p, SyntaxKind::NODE_CONDITIONAL_STMT);
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
