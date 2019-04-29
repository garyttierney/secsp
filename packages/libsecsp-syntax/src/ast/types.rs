use secsp_parser::syntax::NodeKind;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[repr(transparent)]
pub struct SourceFile(rowan::SyntaxNode);
