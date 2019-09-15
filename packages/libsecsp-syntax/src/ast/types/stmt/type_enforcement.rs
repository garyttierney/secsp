use secsp_parser::syntax::{NodeKind, SyntaxNode};
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[repr(transparent)]
pub struct TeRule(SyntaxNode);

pub enum TeRuleKind {
    Allow,
    AuditAllow,
    DontAudit,
    NeverAllow,
}

impl TeRule {}
