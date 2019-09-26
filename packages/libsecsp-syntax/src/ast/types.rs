pub use rowan::WalkEvent;

use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

pub use self::{api::*, def::*, expr::*, stmt::*};

mod api;
mod def;
mod expr;
mod stmt;

#[derive(AstType, Debug, Clone, PartialEq, Eq, Hash)]
#[ast(kind = "NODE_BLOCK")]
pub struct Block(SyntaxNode);

#[derive(AstType, Debug, Clone, PartialEq, Eq, Hash)]
#[ast(kind = "NODE_SOURCE_FILE")]
pub struct SourceFile(SyntaxNode);

impl ItemOwner for SourceFile {}
