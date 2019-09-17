pub use rowan::WalkEvent;

use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

pub use self::{api::*, def::*, expr::*, stmt::*};

mod api;
mod def;
mod expr;
mod stmt;

#[repr(transparent)]
#[derive(AstType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block(SyntaxNode);

#[repr(transparent)]
#[derive(AstType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile(SyntaxNode);

impl ItemOwner for SourceFile {}
