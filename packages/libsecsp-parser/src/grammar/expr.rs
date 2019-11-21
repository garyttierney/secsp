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
    pub(crate) struct ExprContext: u32 {
        const NO_CONTEXT = 1;
        const NO_ATTR_EXPR = 1 << 1;
        const NO_LITERAL = 1 << 2;
        const NO_LEVEL_RANGE = 1 << 3;
        const NO_CATEGORY_RANGE = 1 << 4;
        const NO_NAMED_SET  = 1 << 5;
        const NO_INT_RANGE = 1 << 6;
        const NO_SET = 1 << 7;
        const NO_IDENTIFIER = 1 << 8;
        const NO_SET_ELEMENT = 1 << 9;

        const NONE = 0b1111_1111_1111_1111;

        const SET_ELEMENT = Self::NONE.bits ^ Self::NO_SET_ELEMENT.bits;
        const CATEGORY_RANGE = Self::NONE.bits ^ Self::NO_CATEGORY_RANGE.bits;
        const IDENTIFIER = Self::NONE.bits ^ Self::NO_IDENTIFIER.bits;
        const LEVEL_RANGE = Self::NONE.bits ^ Self::NO_LEVEL_RANGE.bits;
        const NAMED_SET = Self::NONE.bits^ Self::NO_NAMED_SET.bits;

        const CONTEXT = Self::NONE.bits ^ Self::NO_CONTEXT.bits;
        const INT_RANGE = Self::NONE.bits ^ (Self::NO_INT_RANGE.bits | Self::NO_LITERAL.bits);
        const LITERAL = Self::NONE.bits ^ Self::NO_LITERAL.bits;

        const NO_SECURITY_LITERALS = Self::NO_CONTEXT.bits | Self::NO_LEVEL_RANGE.bits | Self::NO_CATEGORY_RANGE.bits;
        const RANGE_OR_NAME_ONLY = Self::NO_CONTEXT.bits | Self::NO_ATTR_EXPR.bits | Self::NO_LITERAL.bits;
        const NAMES_ONLY = Self::NO_CONTEXT.bits | Self::NO_ATTR_EXPR.bits | Self::NO_LITERAL.bits | Self::NO_NAMED_SET.bits;
        const LITERAL_ONLY = Self::NO_SECURITY_LITERALS.bits | Self::NO_ATTR_EXPR.bits;
        const BIN_EXPR = Self::NO_SECURITY_LITERALS.bits | Self::NO_LITERAL.bits | Self::NO_NAMED_SET.bits;
    }
}

// TODO: remove error recovery from expression parsing code, delegate up to item parsers
//       convert ExprContext blacklist to whitelist
//       class permission expression should never refer to classpermissionset when using nested set expressions
//       when a class permission expression refers to a named classpermissionset, it can only be the identifier
//       when a class permission expression refers to an anonymous classpermissionset, it can use bitwise operators on permissions, but never named sets
//            e.g. file { read } & ~write;

pub(crate) fn expression(p: &mut Parser, restriction: ExprContext) -> bool {
    expression_prec(p, 1, restriction)
}

pub(crate) fn try_expression(p: &mut Parser, restriction: ExprContext, msg: &'static str) -> bool {
    if !expression(p, restriction) {
        p.error(msg);
        false
    } else {
        true
    }
}

fn expression_lhs(p: &mut Parser, r: ExprContext) -> Option<CompletedMarker> {
    match p.current() {
        tok!["!"] | tok!["~"] if !r.contains(ExprContext::NO_ATTR_EXPR) => {
            Some(atom::prefix_expr(p))
        }
        tok!["-"] => Some(atom::prefix_expr(p)),
        tok!["("] => Some(atom::paren_expr(p)),
        tok!["{"] => Some(atom::set_expr(p, None)),
        TOK_STRING if !r.contains(ExprContext::NO_LITERAL) => Some(atom::literal_expr(p)),
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

fn expression_postfix(
    p: &mut Parser,
    lhs: CompletedMarker,
    restriction: ExprContext,
) -> CompletedMarker {
    match p.current() {
        tok![":"]
            if !restriction.contains(ExprContext::NO_CONTEXT)
                || !restriction.contains(ExprContext::NO_LEVEL_RANGE) =>
        {
            atom::context_expr(p, lhs)
        }
        tok![".."] if !restriction.contains(ExprContext::NO_CATEGORY_RANGE) => {
            atom::range_expr(p, lhs, SyntaxKind::NODE_CATEGORY_RANGE_EXPR)
        }
        tok!["-"]
            if !restriction.contains(ExprContext::NO_LEVEL_RANGE)
                || !restriction.contains(ExprContext::NO_INT_RANGE) =>
        {
            let kind = if lhs.kind() == SyntaxKind::NODE_LITERAL_EXPR {
                SyntaxKind::NODE_INT_RANGE_EXPR
            } else {
                SyntaxKind::NODE_LEVEL_RANGE_EXPR
            };

            atom::range_expr(p, lhs, kind)
        }
        tok!["{"] if !restriction.contains(ExprContext::NO_NAMED_SET) => {
            atom::set_expr(p, Some(lhs))
        }
        _ => lhs,
    }
}

pub(crate) fn expression_prec(p: &mut Parser, precedence: u8, restriction: ExprContext) -> bool {
    let mut lhs = match expression_lhs(p, restriction) {
        Some(lhs) => lhs,
        None => return false,
    };

    if restriction.contains(ExprContext::NO_ATTR_EXPR) {
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
