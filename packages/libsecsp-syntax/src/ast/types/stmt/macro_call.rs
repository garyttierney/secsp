use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

use crate::ast::AstNode;
use crate::ast::Expr;

#[derive(AstType)]
#[ast(kind = "NODE_MACRO_CALL")]
pub struct MacroCall(pub SyntaxNode);

impl MacroCall {
    pub fn name(&self) -> Option<Expr> {
        self.children().nth(0)
    }

    pub fn arguments(&self) -> impl Iterator<Item = Expr> {
        self.children().skip(1)
    }
}
