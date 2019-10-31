#[test]
fn parse_macro_call() {
    super::test_parser(
        r#"
        <marker type="NODE_MACRO_CALL">macro_name(a && b, "test", 123);</marker>
    "#,
    );
}

#[test]
fn parse_conditional() {
    super::test_parser(
        r#"
        <marker type="NODE_CONDITIONAL_STMT">if a && b {
        }</marker>
      "#,
    );
}

#[test]
fn parse_conditional_with_else() {
    super::test_parser(
        r#"
        <marker type="NODE_CONDITIONAL_STMT">if a {
        } else {
        }</marker>
        "#,
    )
}

#[test]
fn parse_conditional_with_else_if() {
    super::test_parser(
        r#"
        <marker type="NODE_CONDITIONAL_STMT">if a {
        } else <marker type="NODE_CONDITIONAL_STMT"> if b || c {
        }</marker></marker>
        "#,
    )
}

#[test]
fn parse_allow_rule() {
    super::test_parser(
        r#"
         <marker type="NODE_TE_RULE">allow a b : file { read write };</marker>
        "#,
    )
}

#[test]
fn parse_type_transition() {
    super::test_parser(
        r#"
        <marker type="NODE_TE_TRANSITION">type_transition a b : file c "file_name";</marker>
        "#,
    )
}
