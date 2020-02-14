use crate::grammar::atom;
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::SyntaxKind::*;

pub(crate) fn class(p: &mut ItemParser) -> Result<(), ItemParseError> {
    let is_common = p.eat_keyword(kw!["common"])?;

    if !p.eat_keyword(kw!["class"])? && !p.eat_keyword(kw!["class_map"])? {
        p.error("expected 'class'");
    }

    p.expect(TOK_NAME)?;

    if p.at_keyword(kw!["extends"]) {
        atom::parse_extends_list(p.inner);
    }

    p.expect(tok!["{"])?;

    while !p.at(tok!["}"]) && !p.at(TOK_EOF) {
        if !p.eat(TOK_NAME)? {
            break;
        }
    }

    p.expect(tok!["}"])?;

    Ok(())
}
