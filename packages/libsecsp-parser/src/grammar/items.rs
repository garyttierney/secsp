use std::str::FromStr;

use crate::grammar::def;
use crate::grammar::{atom, stmt};
use crate::parser::Parser;
use crate::syntax::KeywordKind;

pub(crate) fn parse_item(p: &mut Parser) {
    fn at_kw(p: &Parser) -> bool {
        atom::is_at_path_start(p, 0) || p.at(tok!["if"]) || p.at(tok!["else"])
    }

    if !at_kw(p) {
        p.bump();
        p.error("expected keyword");
        return;
    }

    match KeywordKind::from_str(p.current_text()) {
        Ok(kw) => {
            match kw {
                // test concrete_block
                // block abc {
                // }

                // test abstract_block {
                // abstract block abc extends dfg {
                // }
                kw!["abstract"] | kw!["block"] | kw!["optional"] | kw!["in"] => def::container(p),

                // test te_rule
                // allow src target : expr;

                // test te_rule_inline_classpermission
                // allow src target : file (read);
                kw!["allow"] | kw!["audit_allow"] | kw!["never_allow"] | kw!["dont_audit"] => {
                    stmt::te_rule(p, kw)
                }

                kw!["type_transition"] | kw!["type_member"] | kw!["type_change"] => {
                    stmt::te_transition(p, kw)
                }

                // test class_def
                // class file { read write }

                // test common_class_def
                // common class filecommon { read write }

                // test class_extends_def
                // class file extends filecommon {}
                kw!["class"] | kw!["common"] | kw!["class_map"] => {
                    def::class(p);
                }

                // test if_stmt
                // if expr {
                // }

                // test if_else_stmt
                // if expr {
                // } else {
                // }
                kw!["if"] => stmt::conditional(p),

                // test macro_def
                // macro my_macro() {
                // }

                // test macro_def_with_params
                // macro my_macro(type t) {
                // }
                kw!["macro"] => def::macro_(p),

                // test var_def
                // type t;

                // test var_def_with_initializer
                // type_attribute t = a & b;
                kw if kw.is_var_type() && atom::is_at_path_start(p, 1) => def::variable(p),
                _kw => unimplemented!("Parsing {:?} is unimplemented", kw),
            }
        }
        Err(_) => {
            let ident = atom::path_expr(p);

            // We didn't find a keyword statement, so determine if the current
            // token stream represents a statement that begins with an identifier.
            // e.g.,
            // `macro_call(a);`
            // `my_ident |= val;`
            match p.current() {
                tok!["("] => stmt::macro_call(p, ident),
                _ => {
                    p.bump();
                }
            };
        }
    };
}
