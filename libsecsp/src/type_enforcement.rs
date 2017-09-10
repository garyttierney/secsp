//! Parser for SELinux type-enforcement statements.

use ast::*;
use expr::*;
use name::*;

named!(pub access_vector<&[u8], AccessVector>,
    alt_complete!(
        ws!(do_parse!(
            security_class: variable >>
            permissions: primary_expr >>

            (AccessVector::ClassAndPermissions(security_class, permissions))
        ))  |
        map!(expr, |v| AccessVector::Permission(v))
    )
);

named!(pub allow_rule<&[u8], Statement>,
    ws!(do_parse!(
        rule_type: type_specifier >>
        source: primary_expr >>
        target: primary_expr >>
        tag!(":") >>
        access_vector: access_vector >>
        tag!(";") >> 

        (Statement::AccessVectorRule {
            rule_type,
            source,
            target,
            access_vector
        })
    ))
);

#[cfg(test)]
mod testing {
    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_allow_rule() {
        let expected = Statement::AccessVectorRule {
            rule_type: AllowRuleType::Allow,
            source: Expr::var("a"),
            target: Expr::var("b"),
            access_vector: AccessVector::Permission(Expr::var("c")),
        };

        let actual = parse::<Statement, _>("allow a b : c;", allow_rule);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_dontaudit_rule_with_permission_sets() {
        let expected = Statement::AccessVectorRule {
            rule_type: AllowRuleType::DontAudit,
            source: Expr::var("a"),
            target: Expr::var("b"),
            access_vector: AccessVector::Permission(Expr::Binary(
                Box::from(Expr::var("permission_set_1")),
                BinaryOp::BitwiseAnd,
                Box::from(Expr::var("permission_set_2")),
            )),
        };

        let actual = parse::<Statement, _>(
            "dontaudit a b : permission_set_1 & permission_set_2;",
            allow_rule,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn parse_allow_rule_with_class_and_permisisons() {
        let expected = Statement::AccessVectorRule {
            rule_type: AllowRuleType::AuditAllow,
            source: Expr::var("a"),
            target: Expr::var("b"),
            access_vector: AccessVector::ClassAndPermissions(
                Expr::var("class"),
                Expr::Binary(
                    Box::from(Expr::var("permission1")),
                    BinaryOp::BitwiseOr,
                    Box::from(Expr::var("permission2")),
                ),
            ),
        };

        let actual = parse::<Statement, _>(
            "auditallow a b : class (permission1 | permission2);",
            allow_rule,
        );
        assert_eq!(expected, actual);
    }
}