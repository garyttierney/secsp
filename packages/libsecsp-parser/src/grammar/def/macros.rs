use std::str::FromStr;

use crate::grammar::block;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn macro_(p: &mut ItemParser) -> Result<(), ItemParseError> {
    // pre-test: parser must be at a "macro" keyword.
    assert!(p.eat_keyword(kw!["macro"])?);
    p.expect(TOK_NAME)?;

    parse_macro_param_list(p);
    block::parse_block(p.inner);

    Ok(())
}

fn parse_macro_param_list(p: &mut ItemParser) -> Result<(), ItemParseError> {
    p.expect(tok!["("])?;

    while !p.at(tok![")"]) && !p.at(TOK_EOF) {
        parse_macro_param_list_item(p);
        if !p.eat(tok![","])? {
            break;
        }
    }

    p.expect(tok![")"])?;

    Ok(())
}

fn parse_macro_param_list_item(p: &mut ItemParser) -> Result<(), ItemParseError> {
    let m = p.mark();

    match KeywordKind::from_str(p.current_text()) {
        Ok(kw) => p.bump_as(kw.into())?,
        Err(_) if p.at(TOK_NAME) => {
            p.bump()?;
        }
        _ => {
            m.abandon(p.inner);
            return Ok(());
        }
    }

    p.expect(TOK_NAME)?;
    m.complete(p.inner, NODE_MACRO_PARAM_LIST_ITEM);

    Ok(())
}
