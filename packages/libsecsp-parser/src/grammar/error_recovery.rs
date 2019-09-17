use crate::parser::Parser;
use crate::syntax::SyntaxKind;

pub(crate) fn recover_from_item(p: &mut Parser) {
    let mut brace_depth = 0;
    let m = p.mark();

    loop {
        match p.current() {
            SyntaxKind::TOK_OPEN_BRACE => {
                p.bump();
                brace_depth += 1;
            }
            SyntaxKind::TOK_CLOSE_BRACE => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    m.complete(p, SyntaxKind::NODE_PARSE_ERROR);
                    return;
                }

                p.bump();
            }
            SyntaxKind::TOK_SEMICOLON => {
                p.bump();
                m.complete(p, SyntaxKind::NODE_PARSE_ERROR);
                return;
            }
            SyntaxKind::TOK_EOF => {
                m.complete(p, SyntaxKind::NODE_PARSE_ERROR);
                return;
            }
            _ => p.bump(),
        }
    }
}

pub(crate) fn recover_from_expr(p: &mut Parser) {
    let m = p.mark();
    p.bump();
    m.complete(p, SyntaxKind::NODE_PARSE_ERROR);
}
