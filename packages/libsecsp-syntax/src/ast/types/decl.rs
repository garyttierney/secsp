use secsp_parser::syntax::NodeKind;
use secsp_syntax_derive::AstType;

#[repr(transparent)]
#[derive(AstType)]
pub struct Container(rowan::SyntaxNode);

impl Container {}

#[repr(transparent)]
#[derive(AstType)]
#[kind(MacroDef)]
pub struct MacroDecl(rowan::SyntaxNode);

#[repr(transparent)]
#[derive(AstType)]
#[kind(Container, Variable)]
pub struct BlockItem(rowan::SyntaxNode);
