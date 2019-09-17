use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstType;

use crate::ast::types::{ItemOwner, NameOwner};
use crate::ast::{AstNode, Expr};

#[repr(transparent)]
#[derive(AstType)]
pub struct ContainerDef(SyntaxNode);

impl ContainerDef {}
impl NameOwner for ContainerDef {}
impl ItemOwner for ContainerDef {}

#[repr(transparent)]
#[derive(AstType)]
pub struct MacroDef(SyntaxNode);

impl NameOwner for MacroDef {}
impl ItemOwner for MacroDef {}

#[repr(transparent)]
#[derive(AstType)]
pub struct VariableDef(SyntaxNode);

impl NameOwner for VariableDef {}
impl VariableDef {
    pub fn initializer(&self) -> Option<Expr> {
        self.children::<Expr>().next()
    }
}
#[repr(transparent)]
#[derive(AstType)]
#[kind(ContainerDef, MacroDef, VariableDef)]
pub struct Definition(SyntaxNode);

pub enum DefinitionKind<'a> {
    ContainerDef(&'a ContainerDef),
    MacroDef(&'a MacroDef),
    VariableDef(&'a VariableDef),
}
