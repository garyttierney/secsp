use crate::parser::syntax::NodeKind;
use crate::parser::syntax::SyntaxKindClass;
use crate::parser::syntax::TokenKind;
use crate::parser::CspParser;

pub fn recover_from_item(p: &mut CspParser) {
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

pub fn recover_from_expr(p: &mut CspParser) {
    let m = p.mark();
    p.bump();
    m.complete(p, NodeKind::ParseError);
}
