#[test]
fn parse_global_path() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_PATH_EXPR">.global.item</marker>);
    "#,
    )
}

#[test]
fn parse_nested_path() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_PATH_EXPR">nested1.nested2.nested3</marker>);
    "#,
    )
}

#[test]
fn parse_paren_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_PAREN_EXPR">(a && b)</marker>);
    "#,
    )
}

#[test]
fn parse_context_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_CONTEXT_EXPR">user:role:type</marker>);
    "#,
    )
}

#[test]
fn parse_mls_context_expr() {
    super::test_parser(
        r#"
        callstub(
            <marker type="NODE_CONTEXT_EXPR">user:role:type:<marker type="NODE_LEVEL_RANGE_EXPR">s1-s2</marker></marker>
        );
    "#,
    )
}

#[test]
fn parse_mls_mcs_context_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_CONTEXT_EXPR">user:role:type:s1:c2..c5-s10:c1</marker>);
    "#,
    )
}

#[test]
fn parse_level_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_LEVEL_EXPR">sensitivity:category</marker>);
    "#,
    )
}

#[test]
fn parse_level_range_expr() {
    super::test_parser(
        r#"
            callstub(<marker type="NODE_LEVEL_RANGE_EXPR">sensitivity:category-sensitivity2</marker>);
        "#,
    )
}

#[test]
fn parse_set_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_SET_EXPR">{ id1 id2 }</marker>);
    "#,
    )
}

#[test]
fn parse_negated_set_expr() {
    super::test_parser(r#"
        callstub(<marker type="NODE_PREFIX_EXPR">~<marker type="NODE_SET_EXPR">{ id1 id2 }</marker></marker>);
    "#)
}

#[test]
fn parse_bin_set_expr() {
    super::test_parser(
        r#"
        callstub(<marker type="NODE_BINARY_EXPR"><marker type="NODE_SET_EXPR">{ id1 }</marker> | abc</marker>);
    "#,
    )
}
