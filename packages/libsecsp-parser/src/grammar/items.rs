use std::str::FromStr;

use crate::grammar::def;
use crate::grammar::{atom, stmt};
use crate::parser::marker::Marker;
use crate::parser::Parser;
use crate::syntax::KeywordKind;
use crate::syntax::SyntaxKind;
use crate::syntax::SyntaxKind::*;

pub(crate) enum ItemParseError {
    AtNewKeyword,
    AtDelimiter,
}

pub(crate) struct ItemParser<'p, 't> {
    pub(crate) inner: &'p mut Parser<'t>,
    seen_error: bool,
}

impl<'p, 't> ItemParser<'p, 't> {
    pub fn new(inner: &'p mut Parser<'t>) -> Self {
        Self {
            inner,
            seen_error: false,
        }
    }

    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    pub fn at_keyword<K>(&mut self, kw: K) -> bool
    where
        K: AsRef<str>,
    {
        self.inner.current_text() == kw.as_ref()
    }

    pub fn bump(&mut self) -> Result<(), ItemParseError> {
        self.check()?;
        self.inner.bump();

        Ok(())
    }

    pub fn bump_as(&mut self, kind: SyntaxKind) -> Result<(), ItemParseError> {
        self.check()?;
        self.inner.bump_as(kind);

        Ok(())
    }

    pub fn current(&self) -> SyntaxKind {
        self.inner.current()
    }

    pub fn current_text(&self) -> &str {
        self.inner.current_text()
    }

    pub fn eat(&mut self, tok: SyntaxKind) -> Result<bool, ItemParseError> {
        self.check()?;

        Ok(self.inner.eat(tok))
    }

    pub fn eat_keyword<K>(&mut self, kw: K) -> Result<bool, ItemParseError>
    where
        K: AsRef<str> + Into<SyntaxKind>,
    {
        self.check()?;

        Ok(self.inner.eat_keyword(kw))
    }

    pub fn error(&mut self, msg: &str) {
        self.inner.error(msg);
    }

    pub fn expect(&mut self, tok: SyntaxKind) -> Result<(), ItemParseError> {
        self.check()?;

        if !self.inner.expect(tok) {
            self.seen_error = true;
        }

        Ok(())
    }

    pub fn mark(&mut self) -> Marker {
        self.inner.mark()
    }

    fn check(&mut self) -> Result<(), ItemParseError> {
        if !self.seen_error {
            return Ok(());
        }

        match self.inner.current() {
            tok![";"] | tok!["{"] | tok!["}"] => Err(ItemParseError::AtDelimiter),
            tok => {
                let kw = KeywordKind::from_str(self.inner.current_text());
                if kw.is_err() {
                    Ok(())
                } else {
                    Err(ItemParseError::AtDelimiter)
                }
            }
            _ => Ok(()),
        }
    }

    pub fn error_check(&mut self) -> Result<(), ItemParseError> {
        self.seen_error = true;
        self.check()
    }

    pub fn try_parse_with<ParseFn>(&mut self, kind: SyntaxKind, marker: Marker, parser: ParseFn)
    where
        ParseFn: FnOnce(&mut Self) -> Result<(), ItemParseError>,
    {
        let result = parser(self);

        if let Err(ItemParseError::AtDelimiter) = result {
            self.inner.bump();
        }

        marker.complete(self.inner, kind);
    }

    pub fn try_parse<ParseFn>(&mut self, kind: SyntaxKind, parser: ParseFn)
    where
        ParseFn: FnOnce(&mut Self) -> Result<(), ItemParseError>,
    {
        let marker = self.inner.mark();
        self.try_parse_with(kind, marker, parser)
    }
}

pub(crate) fn parse_item(p: &mut Parser) {
    fn at_kw(p: &Parser) -> bool {
        atom::is_at_path_start(p, 0) || p.at(tok!["if"]) || p.at(tok!["else"])
    }

    if !at_kw(p) {
        p.bump();
        p.error("expected keyword");
        return;
    }

    let kw = KeywordKind::from_str(p.current_text());
    let mut item_parser = ItemParser::new(p);

    match kw {
        Ok(kw) => {
            match kw {
                // test concrete_block
                // block abc {
                // }

                // test abstract_block {
                // abstract block abc extends dfg {
                // }
                kw!["abstract"] | kw!["block"] | kw!["optional"] | kw!["in"] => {
                    item_parser.try_parse(NODE_CONTAINER_DEF, def::container)
                }

                // test te_rule
                // allow src target : expr;

                // test te_rule_inline_classpermission
                // allow src target : file (read);
                kw!["allow"] | kw!["audit_allow"] | kw!["never_allow"] | kw!["dont_audit"] => {
                    item_parser.try_parse(NODE_TE_RULE, |p| stmt::te_rule(p, kw));
                }

                kw!["constrain"] | kw!["mlsconstrain"] | kw!["mlsvalidatetrans"] => {
                    item_parser.try_parse(NODE_CONSTRAIN, |p| stmt::constrain(p, kw))
                }

                kw!["class_permission_set"]
                | kw!["type_attribute_set"]
                | kw!["role_attribute_set"]
                | kw!["user_attribute_set"] => {
                    item_parser.try_parse(NODE_ATTRIBUTE_SET, |p| stmt::attribute_set(p, kw))
                }

                kw!["role_transition"] => {
                    item_parser.try_parse(NODE_ROLE_TRANSITION, stmt::role_transition)
                }

                kw!["range_transition"] => {
                    item_parser.try_parse(NODE_RANGE_TRANSITION, stmt::range_transition)
                }

                kw!["type_transition"] | kw!["type_member"] | kw!["type_change"] => {
                    item_parser.try_parse(NODE_TE_TRANSITION, |p| stmt::type_transition(p, kw));
                }

                kw!["netifcon"] => item_parser.try_parse(NODE_NETIF_CONTEXT, stmt::netifcon),
                kw!["portcon"] => item_parser.try_parse(NODE_PORT_CONTEXT, stmt::portcon),
                kw!["filecon"] => item_parser.try_parse(NODE_FILE_CONTEXT, stmt::filecon),

                kw!["role_transition"] => {
                    item_parser.try_parse(NODE_ROLE_TRANSITION, stmt::role_transition)
                }

                kw!["range_transition"] => {
                    item_parser.try_parse(NODE_RANGE_TRANSITION, stmt::range_transition)
                }

                // test class_def
                // class file { read write }

                // test common_class_def
                // common class filecommon { read write }

                // test class_extends_def
                // class file extends filecommon {}
                kw!["class"] | kw!["common"] | kw!["class_map"] => {
                    item_parser.try_parse(NODE_CLASS_DEF, def::class);
                }

                kw!["class_mapping"] => {
                    item_parser.try_parse(NODE_CLASS_MAPPING, def::class_mapping)
                }

                // test if_stmt
                // if expr {
                // }

                // test if_else_stmt
                // if expr {
                // } else {
                // }
                kw!["if"] => item_parser.try_parse(NODE_CONDITIONAL_STMT, stmt::conditional),

                // test macro_def
                // macro my_macro() {
                // }

                // test macro_def_with_params
                // macro my_macro(type t) {
                // }
                kw!["macro"] => item_parser.try_parse(NODE_MACRO_DEF, def::macro_),

                // test var_def
                // type t;

                // test var_def_with_initializer
                // type_attribute t = a & b;
                kw if kw.is_var_type() && atom::is_at_path_start(item_parser.inner, 1) => {
                    item_parser.try_parse(NODE_VARIABLE_DEF, def::variable)
                }
                _kw => unimplemented!("Parsing {:?} is unimplemented", kw),
            }
        }
        Err(_) => {
            let ident = atom::path_expr(item_parser.inner);

            // We didn't find a keyword statement, so determine if the current
            // token stream represents a statement that begins with an identifier.
            // e.g.,
            // `macro_call(a);`
            // `my_ident |= val;`
            match item_parser.inner.current() {
                tok!["("] => {
                    let preceding = ident.precede(item_parser.inner);
                    item_parser.try_parse_with(NODE_MACRO_CALL, preceding, stmt::macro_call)
                }
                _ => {
                    item_parser.inner.bump();
                }
            };
        }
    };
}
