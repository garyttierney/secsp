#[test]
fn parse_macro_def_no_params() {
    super::test_parser(
        r#"
        <marker type="macrodef">macro test<marker type="macroparamlist">()</marker> {
        }</marker>
    "#,
    )
}

#[test]
fn parse_macro_def() {
    super::test_parser(
        r#"
        <marker type="macrodef">macro test<marker type="macroparamlist">(
            <marker type="macroparamlistitem">type t</marker>
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
fn parse_var_def() {
    super::test_parser(
        r#"
        <marker type="variabledef">type a;</marker>
    "#,
    )
}

#[test]
fn parse_var_with_initializer() {
    super::test_parser(
        r#"
        <marker type="variabledef">type_attribute a = <marker type="binaryexpr">a | b</marker>;</marker>
    "#,
    )
}
