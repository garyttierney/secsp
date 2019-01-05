use crate::ast::AstChildren;
use crate::ast::AstNode;
use crate::ast::SyntaxKind;
use crate::token::TokenType;

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
        use self::TokenType::*;

        let op = match kind {
            SyntaxKind::Token(Caret) => BitwiseXor,
            SyntaxKind::Token(Pipe) => BitwiseOr,
            SyntaxKind::Token(Ampersand) => BitwiseAnd,
            SyntaxKind::Token(DoubleAmpersand) => LogicalAnd,
            SyntaxKind::Token(DoublePipe) => LogicalOr,
            _ => return None,
        };

        Some(op)
    }
}

ast_type!(pub struct SourceFileNode => SourceFile: BlockOwner);
ast_type!(pub struct BlockNode => Block:);

ast_type!(pub struct ContainerNode => Container: BlockOwner);

ast_enum!(
    pub enum BlockItem {
        Container,
        Variable,
    }
);

ast_type!(pub struct VariableNode => Variable: {});

ast_enum!(
    pub enum Stmt {
        Variable,
    }
);

ast_type!(pub struct PathExprNode => PathExpr:);
ast_type!(pub struct PrefixExprNode => PrefixExpr:);

ast_enum!(
    pub enum Expr {
        PathExpr,
        PrefixExpr,
    }
);

pub trait BlockOwner<'a>: AstNode<'a> {
    fn items(self) -> AstChildren<'a, BlockItem<'a>> {
        self.child::<Block>().children()
    }
}
