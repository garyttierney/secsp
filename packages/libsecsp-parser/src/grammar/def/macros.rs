use std::str::FromStr;

use crate::grammar::block;
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;
use crate::syntax::{KeywordKind, TokenKind};

pub(crate) fn macro_(p: &mut Parser) {
    let m = p.mark();
    // pre-test: parser must be at a "macro" keyword.
    assert!(p.eat_keyword(kw![macro]));
    p.expect(TOK_NAME);

    parse_macro_param_list(p);
    block::parse_block(p);

    m.complete(p, NODE_MACRO_DEF);
}

fn parse_macro_param_list(p: &mut Parser) {
    let m = p.mark();

    p.expect(tok!['(']);

    if !p.at(tok![')']) {
        loop {
            if !parse_macro_param_list_item(p) {
                break;
            }
        }
    }

    p.expect(tok![')']);
    m.complete(p, NODE_MACRO_PARAM_LIST);
}

fn parse_macro_param_list_item(p: &mut Parser) -> bool {
    let m = p.mark();

    match KeywordKind::from_str(p.current_text()) {
        Ok(kw) => p.bump_as(kw),
        Err(_) if p.at(TokenKind::Name) => {
            p.error("expected keyword");
            p.bump();
        }
        _ => {
            m.abandon(p);
            return false;
        }
    }

    p.expect(TokenKind::Name);
    m.complete(p, NODE_MACRO_PARAM_LIST_ITEM);

    p.eat(tok![,])
}
