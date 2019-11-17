use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstEnum;
use secsp_syntax_derive::AstType;
pub use {
    self::bin_expr::*, self::literal_expr::*, self::path_expr::*, self::perm_expr::*,
    self::prefix_expr::*, self::sec_expr::*, self::set_expr::*,
};

mod bin_expr;
mod literal_expr;
mod path_expr;
mod perm_expr;
mod prefix_expr;
mod sec_expr;
mod set_expr;

#[derive(AstType)]
#[ast(kind = "NODE_PAREN_EXPR")]
pub struct ParenExpr(pub SyntaxNode);

#[derive(AstEnum)]
pub enum Expr {
    #[ast(kind = "NODE_BINARY_EXPR")]
    Binary(BinaryExpr),

    #[ast(kind = "NODE_CATEGORY_RANGE_EXPR")]
    CategoryRange(CategoryRangeExpr),

    #[ast(kind = "NODE_LEVEL_EXPR")]
    Level(LevelExpr),

    #[ast(kind = "NODE_LEVEL_RANGE_EXPR")]
    LevelRange(LevelRangeExpr),

    #[ast(kind = "NODE_CONTEXT_EXPR")]
    Context(ContextExpr),

    #[ast(kind = "NODE_LITERAL_EXPR")]
    Literal(LiteralExpr),

    #[ast(kind = "NODE_PATH_EXPR")]
    Path(PathExpr),

    #[ast(kind = "NODE_PAREN_EXPR")]
    Paren(ParenExpr),

    #[ast(kind = "NODE_PREFIX_EXPR")]
    Prefix(PrefixExpr),
}
