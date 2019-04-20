use std::convert::TryFrom;

use rowan::SyntaxKind;

use crate::ast::AstChildren;
use crate::ast::AstNode;
use crate::parser::syntax::TokenKind;

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

    pub fn from(kind: SyntaxKind) -> Option<Self> {
        use self::BinaryOperator::*;
        use self::TokenKind::*;

        let tok = TokenKind::try_from(kind).ok()?;
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

ast_type!(pub struct SourceFile: BlockOwner {});
ast_type!(pub struct Block: BlockOwner {});

ast_type!(pub struct Container: BlockOwner {});

ast_enum!(
    pub enum BlockItem => BlockItemKind {
        Container,
        Variable,
    }
);

ast_type!(pub struct Variable: {});

ast_enum!(
    pub enum Stmt => StmtKind {
        Variable,
    }
);

ast_type!(pub struct PathExpr:);
ast_type!(pub struct PrefixExpr => PrefixExpr:);

ast_enum!(
    pub enum Expr => ExprKind {
        PathExpr,
        PrefixExpr,
    }
);

pub trait BlockOwner: AstNode {
    fn items(&self) -> AstChildren<BlockItem> {
        self.child::<Block>().children()
    }
}
