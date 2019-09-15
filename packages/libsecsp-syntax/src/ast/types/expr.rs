use secsp_parser::syntax::{NodeKind, SyntaxNode};
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[kind(
    BinaryExpr,
    CategoryRangeExpr,
    LevelExpr,
    LevelRangeExpr,
    ContextExpr,
    LiteralExpr,
    ListExpr,
    PathExpr,
    ParenExpr,
    PrefixExpr
)]
#[repr(transparent)]
pub struct Expr(SyntaxNode);
