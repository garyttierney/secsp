use std::str::FromStr;

use crate::grammar::expr::{expression, ExprContext};
use crate::grammar::items::{ItemParseError, ItemParser};
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind::*;

pub(crate) fn variable(p: &mut ItemParser) -> Result<(), ItemParseError> {
    let kw = KeywordKind::from_str(p.current_text()).expect("should be at var type keyword");
    assert!(kw.is_var_type());
    p.bump_as(kw.into())?;
    p.expect(TOK_NAME)?;

    if p.eat(tok!["="])? && !expression(p.inner, ExprContext::all()) {
        p.error_check()?;
    }

    p.expect(tok![";"])?;

    Ok(())
}
