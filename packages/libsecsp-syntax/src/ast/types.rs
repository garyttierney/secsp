use secsp_parser::syntax::NodeKind;
use secsp_syntax_derive::AstType;

pub use self::{api::*, decl::*, expr::*, stmt::*};

mod api;
mod decl;
mod expr;
mod stmt;

#[derive(AstType, Debug)]
#[repr(transparent)]
pub struct Block(rowan::SyntaxNode);

#[derive(AstType, Debug)]
#[repr(transparent)]
pub struct SourceFile(rowan::SyntaxNode);

impl ItemOwner for SourceFile {}
