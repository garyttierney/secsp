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
