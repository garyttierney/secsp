use secsp_parser::syntax::{KeywordKind, SyntaxNode};
use secsp_syntax_derive::AstEnum;
use secsp_syntax_derive::AstType;

use crate::ast::types::{BlockItemOwner, NameOwner};
use crate::ast::{AstChildren, AstNode};
use std::convert::TryFrom;

#[repr(transparent)]
#[derive(AstType)]
#[ast(kind = "NODE_CONTAINER_DEF")]
pub struct ContainerDef(SyntaxNode);

impl ContainerDef {
    pub fn is_abstract(&self) -> bool {
        false
    }
}
impl NameOwner for ContainerDef {}
impl BlockItemOwner for ContainerDef {}

#[derive(AstType)]
#[ast(kind = "NODE_MACRO_PARAM_LIST_ITEM")]
pub struct MacroParam(SyntaxNode);

impl MacroParam {
    pub fn ty(&self) -> Option<KeywordKind> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|child| KeywordKind::try_from(child.kind()).ok())
            .nth(0)
    }
}
impl NameOwner for MacroParam {}

#[derive(AstType)]
#[ast(kind = "NODE_MACRO_DEF")]
pub struct MacroDef(SyntaxNode);

impl MacroDef {
    pub fn params(&self) -> AstChildren<MacroParam> {
        self.children()
    }
}

impl NameOwner for MacroDef {}
impl BlockItemOwner for MacroDef {}

#[repr(transparent)]
#[derive(AstType)]
#[ast(kind = "NODE_VARIABLE_DEF")]
pub struct VariableDef(SyntaxNode);

impl NameOwner for VariableDef {}
//impl VariableDef {
//    pub fn initializer(&self) -> Option<Expr> {
//        self.children::<Expr>().next()
//    }
//}

#[derive(AstEnum)]
pub enum Definition {
    #[ast(kind = "NODE_CONTAINER_DEF")]
    Container(ContainerDef),

    #[ast(kind = "NODE_MACRO_DEF")]
    Macro(MacroDef),

    #[ast(kind = "NODE_VARIABLE_DEF")]
    Variable(VariableDef),
}
