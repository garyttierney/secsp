use std::str::FromStr;

use crate::grammar::block;
use crate::parser::Parser;
use crate::syntax::KeywordKind;
use crate::syntax::NodeKind;
use crate::syntax::TokenKind;

pub(crate) fn parse_macro(p: &mut Parser) {
    // pre-test: parser must be at a "macro" keyword.
    assert!(p.eat_keyword(KeywordKind::Macro));

    p.bump_as(KeywordKind::Macro);
    p.expect(TokenKind::Name);

    parse_macro_param_list(p);
    block::parse_block(p, true);
}

fn parse_macro_param_list(p: &mut Parser) {
    let m = p.mark();

    p.expect(TokenKind::OpenParenthesis);

    if !p.at(TokenKind::CloseParenthesis) {
        loop {
            if !parse_macro_param_list_item(p) {
                break;
            }
        }
    }

    p.expect(TokenKind::CloseParenthesis);
    m.complete(p, NodeKind::MacroParamList);
}

fn parse_macro_param_list_item(p: &mut Parser) -> bool {
    let m = p.mark();

    match KeywordKind::from_str(p.current_text()).ok() {
        Some(kw) => p.bump_as(kw),
        None if p.at(TokenKind::Name) => {
            p.error("expected keyword");
            p.bump();
        }
        None => {
            m.abandon(p);
            return false;
        }
    }

    p.expect(TokenKind::Name);
    m.complete(p, NodeKind::MacroParamListItem);

    p.eat(TokenKind::Comma)
}

#[test]
fn parse_macro_def_no_params() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="macrodef">macro test<marker type="macroparamlist">()</marker> {
        }</marker>
    "#,
    )
}

#[test]
fn parse_macro_def() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="macrodef">macro test<marker type="macroparamlist">(
            <marker type="macroparamlistitem">type t</marker>
        )</marker> {
        }</marker>
    "#,
    )
}
