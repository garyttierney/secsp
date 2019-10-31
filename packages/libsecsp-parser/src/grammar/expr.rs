use crate::grammar::atom;
use crate::grammar::error_recovery;
use crate::parser::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::syntax::SyntaxKind::*;

pub enum BinaryOperator {
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

impl BinaryOperator {
    pub fn binding_power(&self) -> u8 {
        use self::BinaryOperator::*;

        match self {
            LogicalOr => 1,
            LogicalAnd => 2,
            BitwiseOr => 3,
            BitwiseXor => 4,
            BitwiseAnd => 5,
        }
    }

    pub fn from(tok: SyntaxKind) -> Option<Self> {
        use self::BinaryOperator::*;

        let op = match tok {
            tok!["^"] => BitwiseXor,
            tok!["|"] => BitwiseOr,
            tok!["&"] => BitwiseAnd,
            tok!["&&"] => LogicalAnd,
            tok!["||"] => LogicalOr,
            _ => return None,
        };

        Some(op)
    }
}

bitflags! {
    pub(crate) struct ExprParseRestriction: u32 {
        const NO_CONTEXT = 0b0000_0001;
        const NO_ATTR_EXPR = 0b0000_0100;
        const NO_LITERAL = 0b0000_1000;
        const NO_LEVEL_RANGE = 0b0001_0000;
        const NO_CATEGORY_RANGE = 0b0010_0000;

        const NO_SECURITY_LITERALS = Self::NO_CONTEXT.bits | Self::NO_LEVEL_RANGE.bits | Self::NO_CATEGORY_RANGE.bits;
        const RANGE_OR_NAME_ONLY = Self::NO_CONTEXT.bits | Self::NO_ATTR_EXPR.bits | Self::NO_LITERAL.bits;
        const NAMES_ONLY = Self::NO_CONTEXT.bits | Self::NO_ATTR_EXPR.bits | Self::NO_LITERAL.bits;
        const LITERAL_ONLY = Self::NO_SECURITY_LITERALS.bits | Self::NO_ATTR_EXPR.bits;
    }
}

pub(crate) fn expression(p: &mut Parser, restriction: ExprParseRestriction) -> bool {
    expression_prec(p, 1, restriction)
}

pub(crate) fn try_expression(
    p: &mut Parser,
    restriction: ExprParseRestriction,
    msg: &'static str,
) -> bool {
    if !expression(p, restriction) {
        p.error(msg);
        false
    } else {
        true
    }
}

fn expression_lhs(p: &mut Parser, r: ExprParseRestriction) -> Option<CompletedMarker> {
    match p.current() {
        tok!["!"] | tok!["~"] if !r.contains(ExprParseRestriction::NO_ATTR_EXPR) => {
            Some(atom::prefix_expr(p))
        }
        tok!["-"] => Some(atom::prefix_expr(p)),
        tok!["("] => Some(atom::list_or_paren_expr(p)),
        tok!["{"] => Some(atom::set_expr(p)),
        TOK_STRING | TOK_INTEGER if !r.contains(ExprParseRestriction::NO_LITERAL) => {
            Some(atom::literal_expr(p))
        }
        _ => {
            if atom::is_at_path_start(p, 0) {
                let lhs = atom::path_expr(p);

                Some(expression_postfix(p, lhs, r))
            } else {
                error_recovery::recover_from_expr(p);
                None
            }
        }
    }
}

fn expression_postfix(
    p: &mut Parser,
    lhs: CompletedMarker,
    restriction: ExprParseRestriction,
) -> CompletedMarker {
    match p.current() {
        tok![":"] if !restriction.contains(ExprParseRestriction::NO_CONTEXT) => {
            atom::context_expr(p, lhs)
        }
        tok![".."] if !restriction.contains(ExprParseRestriction::NO_CATEGORY_RANGE) => {
            atom::range_expr(p, lhs, SyntaxKind::NODE_CATEGORY_RANGE_EXPR)
        }
        tok!["-"] if !restriction.contains(ExprParseRestriction::NO_LEVEL_RANGE) => {
            atom::range_expr(p, lhs, NODE_LEVEL_RANGE_EXPR)
        }
        _ => lhs,
    }
}

pub(crate) fn expression_prec(
    p: &mut Parser,
    precedence: u8,
    restriction: ExprParseRestriction,
) -> bool {
    let mut lhs = match expression_lhs(p, restriction) {
        Some(lhs) => lhs,
        None => return false,
    };

    if restriction.contains(ExprParseRestriction::NO_ATTR_EXPR) {
        return true;
    }

    loop {
        let current_op_prec = BinaryOperator::from(p.current())
            .map(|p| p.binding_power())
            .unwrap_or(0);

        if current_op_prec < precedence {
            break;
        }

        let m = lhs.precede(p);
        p.bump();

        expression_prec(p, precedence + 1, restriction);
        lhs = m.complete(p, NODE_BINARY_EXPR);
    }

    true
}
