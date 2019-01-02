use crate::ast::keywords;
use crate::ast::keywords::Keyword;
use crate::ast::SyntaxKind;
use crate::grammar::atom;
use crate::grammar::block;
use crate::parser::CspParser;
use crate::token::TokenType;

pub fn parse_container(p: &mut CspParser) {
    let is_abstract = if p.at_text(keywords::ABSTRACT) {
        p.bump_as(Keyword::ABSTRACT);
        true
    } else {
        false
    };

    match Keyword::from_str(p.current_text()) {
        Some(kw) if kw == Keyword::BLOCK => p.bump_as(kw),
        Some(kw) if kw == Keyword::OPTIONAL || kw == Keyword::IN => {
            if is_abstract {
                p.error("only blocks can be declared as abstract");
            }

            p.bump_as(kw);
        }
        _ => p.error("expected block or optional"),
    };

    p.expect(TokenType::Name);

    if p.at_text(keywords::EXTENDS) {
        parse_extends_list(p);
    }

    block::parse_block(p, true);
}

pub fn parse_extends_list(p: &mut CspParser) {
    assert_eq!(keywords::EXTENDS, p.current_text());

    let m = p.mark();

    p.bump_as(Keyword::EXTENDS);
    atom::path_expr(p);

    while p.eat(TokenType::Comma) {
        atom::path_expr(p);
    }

    m.complete(p, SyntaxKind::ExtendsList);
}

#[test]
fn parse_abstract_container() {
    crate::grammar::test::test_parser(
        r#"
        <marker type="Keyword(ABSTRACT)">abstract</marker> block test {}
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
