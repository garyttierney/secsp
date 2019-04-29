use secsp_parser::syntax::NodeKind;
use secsp_syntax_derive::AstType;

use crate::ast::{AstChildren, AstNode};

pub use self::{decl::*, expr::*, stmt::*};

mod decl;
mod expr;
mod stmt;

#[derive(AstType, Debug)]
#[repr(transparent)]
pub struct SourceFile(rowan::SyntaxNode);

impl BlockOwner for SourceFile {}

pub trait BlockOwner: AstNode {
    fn items(&self) -> AstChildren<BlockItem> {
        self.child::<BlockItem>().children()
    }
}
