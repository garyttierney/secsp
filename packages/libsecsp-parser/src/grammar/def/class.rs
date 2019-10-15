use crate::grammar::atom;
use crate::parser::Parser;
use crate::syntax::SyntaxKind::*;

pub(crate) fn class(p: &mut Parser) {
    let m = p.mark();
    let is_common = p.eat_keyword(kw!["common"]);

    if !p.eat_keyword(kw!["class"]) {
        p.error("expected 'class'");
    }

    p.expect(TOK_NAME);

    if p.at_text(kw!["extends"]) {
        atom::parse_extends_list(p);
    }

    p.expect(tok!["{"]);

    while !p.at(tok!["}"]) && !p.at(TOK_EOF) {
        if !p.eat(TOK_NAME) {
            break;
        }
    }

    p.expect(tok!["}"]);
    m.complete(p, NODE_CLASS_DEF);
}
