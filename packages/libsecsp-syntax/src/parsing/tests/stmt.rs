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

#[test]
#[ignore]
fn parse_abstract_container() {
    super::test_parser(
        r#"
        <marker type="KeywordKind(ABSTRACT)">abstract</marker> block test {}
    "#,
    );
}

#[test]
fn parse_abstract_container_with_extends_list() {
    super::test_parser(
        r#"
        abstract block test <marker type="ExtendsList">extends abc</marker> {}
    "#,
    );
}

#[test]
fn parse_var_decl() {
    super::test_parser(
        r#"
        <marker type="variable">type a;</marker>
    "#,
    )
}

#[test]
fn parse_var_with_initializer() {
    super::test_parser(
        r#"
        <marker type="variable">type_attribute a = <marker type="binaryexpr">a | b</marker>;</marker>
    "#,
    )
}
