use secsp_parser::syntax::{NodeKind, SyntaxNode};
use secsp_syntax_derive::AstType;

pub use self::{api::*, def::*, expr::*, stmt::*};
pub use rowan::WalkEvent;

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
