use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block;
use crate::parser::Parser;
use crate::syntax::{KeywordKind, NodeKind, TokenKind};

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

    m.complete(p, NodeKind::ExtendsList);
}

#[test]
#[ignore]
fn parse_abstract_container() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="KeywordKind(ABSTRACT)">abstract</marker> block test {}
    "#,
    );
}

#[test]
fn parse_abstract_container_with_extends_list() {
    crate::grammar::test::test_parser(
        r#"
        abstract block test <marker type="ExtendsList">extends abc</marker> {}
    "#,
    );
}
