use rowan::TransparentNewType;

use secsp_parser::syntax::NodeKind;
use secsp_parser::syntax::SyntaxKindClass;
use secsp_syntax_derive::AstType;

use crate::ast::types::{ItemOwner, NameOwner};
use crate::ast::AstNode;

#[repr(transparent)]
#[derive(AstType)]
pub struct Container(rowan::SyntaxNode);

impl Container {}
impl NameOwner for Container {}
impl ItemOwner for Container {}

#[repr(transparent)]
#[derive(AstType)]
#[kind(MacroDef)]
pub struct MacroDecl(rowan::SyntaxNode);

impl NameOwner for MacroDecl {}
impl ItemOwner for MacroDecl {}

#[repr(transparent)]
#[derive(AstType)]
#[kind(Variable)]
pub struct Variable(rowan::SyntaxNode);

impl NameOwner for Variable {}

#[repr(transparent)]
#[derive(AstType)]
#[kind(Container, MacroDef, Variable)]
pub struct Item(rowan::SyntaxNode);

pub enum ItemKind<'a> {
    Container(&'a Container),
    Macro(&'a MacroDecl),
    Variable(&'a Variable),
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        let kind = self.syntax().kind();
        let repr = self.syntax().into_repr();

        match NodeKind::from_syntax_kind(kind).unwrap() {
            NodeKind::Container => ItemKind::Container(Container::from_repr(repr)),
            NodeKind::MacroDef => ItemKind::Macro(MacroDecl::from_repr(repr)),
            NodeKind::Variable => ItemKind::Variable(Variable::from_repr(repr)),
            _ => unimplemented!(),
        }
    }
}
