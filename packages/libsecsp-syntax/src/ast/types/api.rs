use std::fmt::Write;

use rowan::SmolStr;
use rowan::SyntaxElement;

use secsp_parser::syntax::{NodeKind, SyntaxKindClass, TokenKind};

use crate::ast::types::{Block, Item};
use crate::ast::{AstChildren, AstNode};

pub trait ItemOwner: AstNode {
    fn items(&self) -> AstChildren<Item> {
        self.child::<Block>().children()
    }

    fn items_of<T: AstNode>(&self) -> AstChildren<T> {
        self.child::<Block>().children()
    }
}

pub trait NameOwner: AstNode {
    fn name(&self) -> Option<&SmolStr> {
        self.syntax()
            .children_with_tokens()
            .find_map(|child| match child {
                SyntaxElement::Token(tok) if child.kind() == TokenKind::Name.into_syntax_kind() => {
                    Some(tok.text())
                }
                _ => None,
            })
    }

    fn name_text(&self) -> Option<&str> {
        self.name().map(|st| st.as_str())
    }
}

#[cfg(test)]
mod tests {
    use rowan::TreeArc;

    use crate::ast::{Container, MacroDecl, SourceFile, Variable};

    use super::*;

    fn parse_item<T: AstNode>(text: &str) -> TreeArc<T> {
        SourceFile::parse(text)
            .items_of::<T>()
            .nth(0)
            .unwrap()
            .to_owned()
    }

    fn test_name_owner<T: NameOwner>(text: &str, name: &str) {
        let m = parse_item::<T>(text);
        let macro_name = m.name_text().expect("couldn't find name on node");

        assert_eq!(name, macro_name);
    }

    #[test]
    fn variable_as_name_owner() {
        test_name_owner::<Variable>("type t;", "t");
    }

    #[test]
    fn macro_as_name_owner() {
        test_name_owner::<MacroDecl>("macro abc() {}", "abc");
    }

    #[test]
    fn block_as_name_owner() {
        test_name_owner::<Container>("block abc {}", "abc");
    }
}
