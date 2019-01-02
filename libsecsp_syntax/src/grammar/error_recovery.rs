use crate::ast::SyntaxKind;
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn recover_from_item(p: &mut CspParser) {
    let mut brace_depth = 0;
    let m = p.mark();

    loop {
        match p.current() {
            SyntaxKind::Token(TokenType::OpenBrace) => {
                p.bump();
                brace_depth += 1;
            }
            SyntaxKind::Token(TokenType::CloseBrace) => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    m.complete(p, SyntaxKind::ParseError);
                    return;
                }

                p.bump();
            }
            SyntaxKind::Token(TokenType::Semicolon) => {
                p.bump();
                m.complete(p, SyntaxKind::ParseError);
                return;
            }
            SyntaxKind::Token(TokenType::Eof) => {
                m.complete(p, SyntaxKind::ParseError);
                return;
            }
            _ => p.bump(),
        }
    }
}

pub fn recover_from_expr(p: &mut CspParser) {
    let m = p.mark();
    p.bump();
    m.complete(p, SyntaxKind::ParseError);
}
