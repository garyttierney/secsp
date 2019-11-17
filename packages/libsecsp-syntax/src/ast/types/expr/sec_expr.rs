use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_LEVEL_EXPR")]
pub struct LevelExpr(pub SyntaxNode);

impl LevelExpr {}

#[derive(AstType)]
#[ast(kind = "NODE_LEVEL_RANGE_EXPR")]
pub struct LevelRangeExpr(pub SyntaxNode);

impl LevelRangeExpr {}

#[derive(AstType)]
#[ast(kind = "NODE_CATEGORY_RANGE_EXPR")]
pub struct CategoryRangeExpr(pub SyntaxNode);

impl CategoryRangeExpr {}

#[derive(AstType)]
#[ast(kind = "NODE_CONTEXT_EXPR")]
pub struct ContextExpr(pub SyntaxNode);

impl LevelExpr {}
