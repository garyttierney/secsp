use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block;
use crate::parser::Parser;
use crate::syntax::{KeywordKind, SyntaxKind, TokenKind};

pub(crate) fn parse_container(p: &mut Parser) {
    let is_abstract = p.eat_keyword(KeywordKind::Abstract);

    match KeywordKind::from_str(p.current_text()).ok() {
        Some(kw) if kw == KeywordKind::Block => p.bump_as(kw),
        Some(kw) if kw == KeywordKind::Optional || kw == KeywordKind::In => {
            if is_abstract {
                p.error("only blocks can be declared as abstract");
            }

            p.bump_as(kw);
        }
        _ => p.error("expected block or optional"),
    };

    p.expect(TokenKind::Name);

    if p.at_text(KeywordKind::Extends) {
        parse_extends_list(p);
    }

    block::parse_block(p, true);
}

fn parse_extends_list(p: &mut Parser) {
    let m = p.mark();

    assert!(p.eat_keyword(KeywordKind::Extends));
    atom::path_expr(p);

    while p.eat(TokenKind::Comma) {
        atom::path_expr(p);
    }

    m.complete(p, SyntaxKind::NODE_EXTENDS_LIST);
}
