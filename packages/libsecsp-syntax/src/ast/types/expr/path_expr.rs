use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_PATH_EXPR")]
pub struct PathExpr(pub SyntaxNode);

impl PathExpr {}
