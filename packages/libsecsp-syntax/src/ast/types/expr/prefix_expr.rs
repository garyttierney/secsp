use crate::ast::{AstNode, Expr};

use secsp_parser::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_PREFIX_EXPR")]
pub struct PrefixExpr(pub SyntaxNode);

pub enum PrefixOp {
    Not,
}

impl PrefixExpr {
    pub fn op_kind(&self) -> Option<PrefixOp> {
        match self.op_token()?.kind() {
            SyntaxKind::TOK_EXCLAMATION | SyntaxKind::TOK_TILDE | SyntaxKind::TOK_HYPHEN => {
                Some(PrefixOp::Not)
            }
            _ => None,
        }
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.syntax().first_child_or_token()?.into_token()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.children().nth(0)
    }
}
