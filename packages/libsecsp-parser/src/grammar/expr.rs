use rowan::SyntaxKind;

use crate::grammar::atom;
use crate::grammar::error_recovery;
use crate::parser::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::NodeKind;
use crate::syntax::SyntaxKindClass;
use crate::syntax::TokenKind;

pub enum BinaryOperator {
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        use self::BinaryOperator::*;

        match self {
            LogicalOr => 1,
            LogicalAnd => 2,
            BitwiseOr => 3,
            BitwiseXor => 4,
            BitwiseAnd => 5,
        }
    }

    pub fn from(tok: TokenKind) -> Option<Self> {
        use self::BinaryOperator::*;
        use self::TokenKind::*;

        let op = match tok {
            Caret => BitwiseXor,
            Pipe => BitwiseOr,
            Ampersand => BitwiseAnd,
            DoubleAmpersand => LogicalAnd,
            DoublePipe => LogicalOr,
            _ => return None,
        };

        Some(op)
    }
}

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

pub(crate) fn expression(p: &mut Parser, restriction: ExprRestriction) -> bool {
    expression_prec(p, 1, restriction)
}

fn expression_lhs(p: &mut Parser) -> Option<CompletedMarker> {
    if atom::is_at_path_start(p, 0) {
        return Some(atom::path_expr(p));
    } else if p.at(TokenKind::String) || p.at(TokenKind::Integer) {
        return Some(atom::literal_expr(p));
    }

    match p.current() {
        TokenKind::Exclamation | TokenKind::Tilde => {
            let m = p.mark();
            p.bump();
            expression_prec(p, 255, ExprRestriction::None);
            Some(m.complete(p, NodeKind::PrefixExpr))
        }
        TokenKind::OpenParenthesis => Some(atom::list_or_paren_expr(p)),
        _ => {
            error_recovery::recover_from_expr(p);
            None
        }
    }
}

fn expression_prec(p: &mut Parser, precedence: u8, restriction: ExprRestriction) -> bool {
    let mut lhs = match expression_lhs(p) {
        Some(lhs) => lhs,
        None => return false,
    };

    match p.current() {
        TokenKind::Colon if restriction.allows_context() => {
            return atom::context_expr(p, lhs);
        }
        TokenKind::DotDot => {
            return atom::range_expr(p, lhs, NodeKind::CategoryRangeExpr);
        }
        TokenKind::Hyphen if restriction.allows_range() => {
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
