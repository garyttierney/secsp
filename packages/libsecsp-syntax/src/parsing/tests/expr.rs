#[test]
fn parse_global_path() {
    super::test_parser(
        r#"
        callstub(<marker type="pathexpr">.global.item</marker>);
    "#,
    )
}

#[test]
fn parse_nested_path() {
    super::test_parser(
        r#"
        callstub(<marker type="pathexpr">nested1.nested2.nested3</marker>);
    "#,
    )
}

#[test]
fn parse_list_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="listexpr">(item1, item2, item3)</marker>);
    "#,
    )
}

#[test]
fn parse_paren_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="parenexpr">(a && b)</marker>);
    "#,
    )
}

#[test]
fn parse_context_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type</marker>);
    "#,
    )
}

#[test]
fn parse_mls_context_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type:<marker type="levelrangeexpr">s1-s2</marker></marker>);
    "#,
    )
}

#[test]
fn parse_mls_mcs_context_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="contextexpr">user:role:type:s1:c2..c5-s10:c1</marker>);
    "#,
    )
}

#[test]
fn parse_level_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="levelexpr">sensitivity:category</marker>);
    "#,
    )
}

#[test]
fn parse_level_range_expr() {
    super::test_parser(
        r#"
            callstub(<marker type="levelrangeexpr">sensitivity:category-sensitivity2</marker>);
        "#,
    )
}
