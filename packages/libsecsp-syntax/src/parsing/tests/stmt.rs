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

#[test]
fn parse_constrain() {
    super::test_parser(
        r#"
        <marker type="NODE_CONSTRAIN">constrain file { read write } (u1 || u2);</marker>
    "#,
    )
}

#[test]
fn parse_portcon() {
    super::test_parser(
        r#"
        <marker type="NODE_PORTCON">portcon tcp 5050 my_ctx;</marker>
    "#,
    )
}

#[test]
fn parse_portcon_range() {
    super::test_parser(r#"
        <marker type="NODE_PORTCON">portcon tcp <marker type="NODE_INT_RANGE_EXPR">6667-6669</marker> my_ctx;</marker>
    "#)
}

#[test]
fn parse_portcon_inline_ctx() {
    super::test_parser(r#"
        <marker type="NODE_PORTCON">portcon tcp 5050 <marker type="NODE_CONTEXT_EXPR">my_user:my_role:my_type</marker>;</marker>
    "#)
}
