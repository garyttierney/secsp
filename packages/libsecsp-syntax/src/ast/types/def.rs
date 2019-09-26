use secsp_parser::syntax::SyntaxNode;
use secsp_syntax_derive::AstEnum;
use secsp_syntax_derive::AstType;

use crate::ast::types::{ItemOwner, NameOwner};

#[repr(transparent)]
#[derive(AstType)]
#[ast(kind = "NODE_CONTAINER_DEF")]
pub struct ContainerDef(SyntaxNode);

impl ContainerDef {}
impl NameOwner for ContainerDef {}
impl ItemOwner for ContainerDef {}

#[repr(transparent)]
#[derive(AstType)]
#[ast(kind = "NODE_MACRO_DEF")]
pub struct MacroDef(SyntaxNode);

impl NameOwner for MacroDef {}
impl ItemOwner for MacroDef {}

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
