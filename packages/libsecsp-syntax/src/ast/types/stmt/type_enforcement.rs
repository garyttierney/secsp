use secsp_parser::syntax::NodeKind;
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[repr(transparent)]
pub struct TeRule(rowan::SyntaxNode);

pub enum TeRuleKind {
    Allow,
    AuditAllow,
    DontAudit,
    NeverAllow,
}

impl TeRule {
}
