use crate::ast::SyntaxKind;
use crate::ast::BinaryOperator;
use crate::grammar::atom;
use crate::grammar::error_recovery;
use crate::parser::CompletedMarker;
use crate::parser::CspParser;
use crate::token::Token;
use crate::token::TokenType;

pub fn expression(p: &mut CspParser) {
    expression_prec(p, 1);
}

fn expression_lhs(p: &mut CspParser) -> Option<CompletedMarker<SyntaxKind, Token>> {
    if atom::is_at_path_start(p, 0) {
        return Some(atom::path_expr(p));
    } else if p.at(TokenType::String) || p.at(TokenType::Integer) {
        return Some(atom::literal_expr(p));
    }

    match p.current() {
        SyntaxKind::Token(TokenType::Exclamation) | SyntaxKind::Token(TokenType::Tilde) => {
            let m = p.mark();
            p.bump();
            expression_prec(p, 255);
            Some(m.complete(p, SyntaxKind::PrefixExpr))
        }
        SyntaxKind::Token(TokenType::OpenParenthesis) => Some(atom::list_or_paren_expr(p)),
        _ => {
            error_recovery::recover_from_expr(p);
            None
        }
    }
}

fn expression_prec(p: &mut CspParser, precedence: u8) {
    let mut lhs = match expression_lhs(p) {
        Some(lhs) => lhs,
        None => return,
    };

    loop {
        let current_op_prec = BinaryOperator::from(p.current())
            .map(|p| p.precedence())
            .unwrap_or(0);

        if current_op_prec < precedence {
            break;
        }

        let m = lhs.precede(p);
        p.bump();

        expression_prec(p, precedence + 1);
        lhs = m.complete(p, SyntaxKind::BinaryExpr);
    }
}
