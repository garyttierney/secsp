use crate::parser::Parser;
use crate::syntax::NodeKind;
use crate::syntax::SyntaxKindClass;
use crate::syntax::TokenKind;

pub(crate) fn recover_from_item(p: &mut Parser) {
    let mut brace_depth = 0;
    let m = p.mark();

    loop {
        match TokenKind::from_syntax_kind(p.current()) {
            Some(TokenKind::OpenBrace) => {
                p.bump();
                brace_depth += 1;
            }
            Some(TokenKind::CloseBrace) => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    m.complete(p, NodeKind::ParseError);
                    return;
                }

                p.bump();
            }
            Some(TokenKind::Semicolon) => {
                p.bump();
                m.complete(p, NodeKind::ParseError);
                return;
            }
            Some(TokenKind::Eof) => {
                m.complete(p, NodeKind::ParseError);
                return;
            }
            _ => p.bump(),
        }
    }
}

pub(crate) fn recover_from_expr(p: &mut Parser) {
    let m = p.mark();
    p.bump();
    m.complete(p, NodeKind::ParseError);
}
