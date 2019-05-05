use rowan::TransparentNewType;

use secsp_parser::syntax::NodeKind;
use secsp_parser::syntax::SyntaxKindClass;
use secsp_syntax_derive::AstType;

use crate::ast::types::{ItemOwner, NameOwner};
use crate::ast::{AstNode, Expr};

#[repr(transparent)]
#[derive(AstType)]
pub struct ContainerDef(rowan::SyntaxNode);

impl ContainerDef {}
impl NameOwner for ContainerDef {}
impl ItemOwner for ContainerDef {}

#[repr(transparent)]
#[derive(AstType)]
pub struct MacroDef(rowan::SyntaxNode);

impl NameOwner for MacroDef {}
impl ItemOwner for MacroDef {}

#[repr(transparent)]
#[derive(AstType)]
pub struct VariableDef(rowan::SyntaxNode);

impl NameOwner for VariableDef {}
impl VariableDef {
    pub fn initializer(&self) -> Option<&Expr> {
        self.children::<Expr>().next()
    }
}
#[repr(transparent)]
#[derive(AstType)]
#[kind(ContainerDef, MacroDef, VariableDef)]
pub struct Definition(rowan::SyntaxNode);

pub enum DefinitionKind<'a> {
    ContainerDef(&'a ContainerDef),
    MacroDef(&'a MacroDef),
    VariableDef(&'a VariableDef),
}

impl Definition {
    pub fn kind(&self) -> DefinitionKind {
        let kind = self.syntax().kind();
        let repr = self.syntax().into_repr();

        match NodeKind::from_kind(kind).unwrap() {
            NodeKind::ContainerDef => DefinitionKind::ContainerDef(ContainerDef::from_repr(repr)),
            NodeKind::MacroDef => DefinitionKind::MacroDef(MacroDef::from_repr(repr)),
            NodeKind::VariableDef => DefinitionKind::VariableDef(VariableDef::from_repr(repr)),
            _ => unimplemented!(),
        }
    }
}
