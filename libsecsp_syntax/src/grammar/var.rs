use crate::ast::keywords::Keyword;
use crate::grammar::expr::expression;
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn parse_var(p: &mut CspParser) {
    let kw = Keyword::from_str(p.current_text()).expect("should be at var type keyword");
    assert!(kw.is_var_type());

    p.bump_as(kw);
    p.expect(TokenType::Name);

    if p.eat(TokenType::Equals) {
        expression(p);
    }
}

#[test]
fn parse_var_decl() {
    crate::grammar::test::test_parser(r#"
        <marker type="variable">type a;</marker>
    "#)
}

#[test]
fn parse_var_with_initializer() {
    crate::grammar::test::test_parser(r#"
        <marker type="variable">type_attribute a = a | b;</marker>
    "#)
}
