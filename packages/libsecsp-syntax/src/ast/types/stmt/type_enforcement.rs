use crate::ast::AstNode;

use secsp_parser::syntax::{SyntaxKind, SyntaxNode};
use secsp_syntax_derive::AstType;

#[derive(AstType)]
#[ast(kind = "NODE_TE_RULE")]
pub struct TeRule(SyntaxNode);

#[derive(Debug, PartialEq, Eq)]
pub enum TeRuleKind {
    Allow,
    AuditAllow,
    DontAudit,
    NeverAllow,
}

impl TeRule {
    fn rule_kind(&self) -> TeRuleKind {
        self.syntax()
            .children_with_tokens()
            .find_map(|child| {
                if let Some(tok) = child.into_token() {
                    let rule_kind = match tok.kind() {
                        SyntaxKind::KW_ALLOW => TeRuleKind::Allow,
                        SyntaxKind::KW_AUDIT_ALLOW => TeRuleKind::AuditAllow,
                        SyntaxKind::KW_DONT_AUDIT => TeRuleKind::DontAudit,
                        SyntaxKind::KW_NEVER_ALLOW => TeRuleKind::NeverAllow,
                        _ => return None,
                    };

                    Some(rule_kind)
                } else {
                    None
                }
            })
            .expect("TeRule nodes must have a TeRuleType token")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::testing::parse_and_find;

    fn test_rule_kind(kind: TeRuleKind, code: &str) {
        let rule: TeRule = parse_and_find(code);

        assert_eq!(kind, rule.rule_kind());
    }

    #[test]
    fn test_allow() {
        test_rule_kind(TeRuleKind::Allow, "allow src dest : perms;");
    }

    #[test]
    fn test_dont_audit() {
        test_rule_kind(TeRuleKind::DontAudit, "dont_audit src dest : perms;");
    }

    #[test]
    fn test_audit_allow() {
        test_rule_kind(TeRuleKind::AuditAllow, "audit_allow src dest : perms;");
    }

    #[test]
    fn test_never_allow() {
        test_rule_kind(TeRuleKind::NeverAllow, "never_allow src dest : perms;");
    }
}
