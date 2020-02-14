use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn container(p: &mut ItemParser) -> Result<(), ItemParseError> {
    let is_abstract = p.eat_keyword(kw!["abstract"])?;
    let kw = KeywordKind::from_str(p.current_text());

    match kw {
        Ok(kw!["block"]) | Ok(kw!["optional"]) | Ok(kw!["in"]) => {
            let kw = kw.unwrap();
            p.bump_as(kw.into());
        }
        _ => p.error("expected block or optional"),
    };

    p.expect(TOK_NAME)?;

    if p.current_text() == "extends" {
        atom::parse_extends_list(p.inner);
    }

    block::parse_block(p.inner);

    Ok(())
}
