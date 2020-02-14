use crate::grammar::atom;
use crate::grammar::atom::is_at_path_start;
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
    pub(crate) struct ExprContext: u32 {
        const CONTEXT = 1;
        const ATTR_EXPR = 1 << 1;
        const LITERAL = 1 << 2;
        const LEVEL_RANGE = 1 << 3;
        const CATEGORY_RANGE = 1 << 4;
        const NAMED_SET  = 1 << 5;
        const INT_RANGE = 1 << 6;
        const SET = 1 << 7;
        const IDENTIFIER = 1 << 8;
        const SET_ELEMENT = 1 << 9;
        const LEVEL = 1 << 10;

        const NAMES_ONLY = Self::SET.bits | Self::IDENTIFIER.bits;
        const LITERAL_ONLY = Self::LITERAL.bits;
        const BIN_EXPR = Self::ATTR_EXPR.bits;

        const HYPHEN_RANGE_R = Self::LEVEL_RANGE.bits | Self::INT_RANGE.bits;
        const DOT_RANGE_R = Self::CATEGORY_RANGE.bits;
        const SECURITY_CONTEXT_R = Self::CONTEXT.bits | Self::LEVEL.bits  | Self::LEVEL_RANGE.bits;
    }
}

pub(crate) fn expression(p: &mut Parser, restriction: ExprContext) -> bool {
    expression_prec(p, 1, restriction).is_some()
}

fn expression_lhs(p: &mut Parser, r: ExprContext) -> Option<CompletedMarker> {
    match p.current() {
        tok!["!"] | tok!["~"] if r.intersects(ExprContext::ATTR_EXPR) => Some(atom::prefix_expr(p)),
        tok!["-"] => Some(atom::prefix_expr(p)),
        tok!["("] => Some(atom::paren_expr(p)),
        tok!["{"] => Some(atom::set_expr(p, None)),
        TOK_STRING if r.intersects(ExprContext::LITERAL) => Some(atom::literal_expr(p)),
        tok => {
            let lhs = if atom::is_at_path_start(p, 0) {
                atom::path_expr(p)
            } else if tok == TOK_INTEGER {
                atom::literal_expr(p)
            } else {
                error_recovery::recover_from_expr(p);
                return None;
            };

            Some(expression_postfix(p, lhs, r))
        }
    }
}

fn expression_postfix(p: &mut Parser, lhs: CompletedMarker, r: ExprContext) -> CompletedMarker {
    match p.current() {
        tok![":"] if r.intersects(ExprContext::SECURITY_CONTEXT_R) => {
            atom::security_context_expr(p, lhs)
        }
        tok![".."] if r.intersects(ExprContext::DOT_RANGE_R) && lhs.kind() == NODE_PATH_EXPR => {
            atom::range_expr(p, lhs, SyntaxKind::NODE_CATEGORY_RANGE_EXPR)
        }
        tok!["-"]
            if r.intersects(ExprContext::HYPHEN_RANGE_R) && lhs.kind() == NODE_LITERAL_EXPR =>
        {
            atom::range_expr(p, lhs, SyntaxKind::NODE_INT_RANGE_EXPR)
        }
        tok!["-"] if r.intersects(ExprContext::HYPHEN_RANGE_R) => {
            atom::range_expr(p, lhs, SyntaxKind::NODE_LEVEL_RANGE_EXPR)
        }
        tok => {
            let at_set_item = tok == tok!["~"] || tok == tok!["{"] || is_at_path_start(p, 0);

            if r.intersects(ExprContext::NAMED_SET) && at_set_item {
                let outer = lhs.precede(p);
                expression(p, ExprContext::SET_ELEMENT);
                outer.complete(p, NODE_NAMED_SET_EXPR)
            } else {
                lhs
            }
        }
    }
}

pub(crate) fn expression_prec(
    p: &mut Parser,
    precedence: u8,
    restriction: ExprContext,
) -> Option<CompletedMarker> {
    let mut lhs = match expression_lhs(p, restriction) {
        Some(lhs) => lhs,
        None => return None,
    };

    if !restriction.intersects(ExprContext::ATTR_EXPR) {
        return Some(lhs);
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

    Some(lhs)
}
