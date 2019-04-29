#[test]
fn parse_macro_call() {
    super::test_parser(
        r#"
        <marker type="MacroCall">macro_name(a && b, "test", 123);</marker>
    "#,
    );
}

#[test]
fn parse_conditional() {
    super::test_parser(
        r#"
        <marker type="conditionalstmt">if a && b {
        }</marker>
      "#,
    );
}

#[test]
fn parse_conditional_with_else() {
    super::test_parser(
        r#"
        <marker type="conditionalstmt">if a {
        } else {
        }</marker>
        "#,
    )
}

#[test]
fn parse_conditional_with_else_if() {
    super::test_parser(
        r#"
        <marker type="conditionalstmt">if a {
        } else <marker type="conditionalstmt"> if b || c {
        }</marker></marker>
        "#,
    )
}
