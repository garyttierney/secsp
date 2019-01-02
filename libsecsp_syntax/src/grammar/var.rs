use crate::ast::keywords::Keyword;
use crate::parser::CspParser;
use crate::token::TokenType;
use crate::grammar::expr::expression;

pub fn parse_var(p: &mut CspParser) {
    let kw = Keyword::from_str(p.current_text()).expect("should be at var type keyword");
    assert!(kw.is_var_type());

    p.bump_as(kw);
    p.expect(TokenType::Name);

    if p.eat(TokenType::Equals) {
        expression(p);
    }
}
