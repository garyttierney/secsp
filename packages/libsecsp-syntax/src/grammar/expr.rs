use std::convert::TryFrom;

use crate::ast::BinaryOperator;
use crate::grammar::atom;
use crate::grammar::error_recovery;
use crate::parser::syntax::NodeKind;
use crate::parser::syntax::TokenKind;
use crate::parser::CompletedMarker;
use crate::parser::CspParser;
use crate::token::Token;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExprRestriction {
    NoContext,
    NoRange,
    None,
}

impl ExprRestriction {
    pub fn allows_context(self) -> bool {
        self != ExprRestriction::NoContext
    }

    pub fn allows_range(self) -> bool {
        self != ExprRestriction::NoRange && self != ExprRestriction::NoContext
    }
}

pub fn expression(p: &mut CspParser, restriction: ExprRestriction) -> bool {
    expression_prec(p, 1, restriction)
}

fn expression_lhs(p: &mut CspParser) -> Option<CompletedMarker<Token>> {
    if atom::is_at_path_start(p, 0) {
        return Some(atom::path_expr(p));
    } else if p.at(TokenKind::String) || p.at(TokenKind::Integer) {
        return Some(atom::literal_expr(p));
    }

    match TokenKind::try_from(p.current()).ok() {
        Some(TokenKind::Exclamation) | Some(TokenKind::Tilde) => {
            let m = p.mark();
            p.bump();
            expression_prec(p, 255, ExprRestriction::None);
            Some(m.complete(p, NodeKind::PrefixExpr))
        }
        Some(TokenKind::OpenParenthesis) => Some(atom::list_or_paren_expr(p)),
        _ => {
            error_recovery::recover_from_expr(p);
            None
        }
    }
}

fn expression_prec(p: &mut CspParser, precedence: u8, restriction: ExprRestriction) -> bool {
    let mut lhs = match expression_lhs(p) {
        Some(lhs) => lhs,
        None => return false,
    };

    match TokenKind::try_from(p.current()).ok() {
        Some(TokenKind::Colon) if restriction.allows_context() => {
            return atom::context_expr(p, lhs);
        }
        Some(TokenKind::DotDot) => {
            return atom::range_expr(p, lhs, NodeKind::CategoryRangeExpr);
        }
        Some(TokenKind::Hyphen) if restriction.allows_range() => {
            return atom::range_expr(p, lhs, NodeKind::LevelRangeExpr);
        }
        _ => {}
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

        expression_prec(p, precedence + 1, ExprRestriction::None);
        lhs = m.complete(p, NodeKind::BinaryExpr);
    }

    true
}
