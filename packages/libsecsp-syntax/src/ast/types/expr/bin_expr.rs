use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_BINARY_EXPR")]
pub struct BinaryExpr(pub SyntaxNode);

impl BinaryExpr {}
