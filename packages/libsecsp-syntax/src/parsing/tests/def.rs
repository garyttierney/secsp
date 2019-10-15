#[test]
fn parse_macro_def_no_params() {
    super::test_parser(
        r#"
        <marker type="NODE_MACRO_DEF">macro test<marker type="NODE_MACRO_PARAM_LIST">()</marker> {
        }</marker>
    "#,
    )
}

#[test]
fn parse_macro_def() {
    super::test_parser(
        r#"
        <marker type="NODE_MACRO_DEF">macro test<marker type="NODE_MACRO_PARAM_LIST">(
            <marker type="NODE_MACRO_PARAM_LIST_ITEM">type t</marker>
        )</marker> {
        }</marker>
    "#,
    )
}

#[test]
#[ignore]
fn parse_abstract_container_def() {
    super::test_parser(
        r#"
        <marker type="KW_ABSTRACT">abstract</marker> block test {}
    "#,
    );
}

#[test]
fn parse_abstract_container_with_extends_list() {
    super::test_parser(
        r#"
        abstract block test <marker type="NODE_EXTENDS_LIST">extends abc</marker> {}
    "#,
    );
}

#[test]
fn parse_var_def() {
    super::test_parser(
        r#"
        <marker type="NODE_VARIABLE_DEF">type a;</marker>
    "#,
    )
}

#[test]
fn parse_var_with_initializer() {
    super::test_parser(
        r#"
        <marker type="NODE_VARIABLE_DEF">type_attribute a = <marker type="NODE_BINARY_EXPR">a | b</marker>;</marker>
    "#,
    )
}

#[test]
fn parse_class_def() {
    super::test_parser(
        r#"
        <marker type="NODE_CLASS_DEF">class abc { read write }</marker>
        "#,
    );
}

#[test]
fn parse_class_def_with_extends() {
    super::test_parser(r#"
        <marker type="NODE_CLASS_DEF">class abc <marker type="NODE_EXTENDS_LIST">extends base</marker> { read write }</marker>
    "#)
}
