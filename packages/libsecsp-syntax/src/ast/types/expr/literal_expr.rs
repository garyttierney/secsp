use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_LITERAL_EXPR")]
pub struct LiteralExpr(pub SyntaxNode);

impl LiteralExpr {}
