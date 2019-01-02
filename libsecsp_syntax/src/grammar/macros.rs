use crate::ast::keywords;
use crate::ast::keywords::Keyword;
use crate::ast::SyntaxKind;
use crate::grammar::block;
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn parse_macro(p: &mut CspParser) {
    assert_eq!(keywords::MACRO, p.current_text());

    p.bump_as(Keyword::MACRO);
    p.expect(TokenType::Name);

    parse_macro_param_list(p);
    block::parse_block(p, true);
}

pub fn parse_macro_param_list(p: &mut CspParser) {
    let m = p.mark();

    p.expect(TokenType::OpenParenthesis);

    if !p.at(TokenType::CloseParenthesis) {
        loop {
            if !parse_macro_param_list_item(p) {
                break;
            }
        }
    }

    p.expect(TokenType::CloseParenthesis);
    m.complete(p, SyntaxKind::MacroParamList);
}

pub fn parse_macro_param_list_item(p: &mut CspParser) -> bool {
    let m = p.mark();

    match Keyword::from_str(p.current_text()) {
        Some(kw) => p.bump_as(kw),
        None if p.at(TokenType::Name) => {
            p.error("expected keyword");
            p.bump();
        }
        None => {
            m.abandon(p);
            return false;
        }
    }

    p.expect(TokenType::Name);
    m.complete(p, SyntaxKind::MacroParamListItem);

    p.eat(TokenType::Comma)
}
