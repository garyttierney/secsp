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

    use super::*;
    use crate::ast::{Container, MacroDecl, SourceFile};

    fn parse_item<T: AstNode>(text: &str) -> TreeArc<T> {
        SourceFile::parse(text)
            .items_of::<T>()
            .nth(0)
            .unwrap()
            .to_owned()
    }

    #[test]
    fn macro_as_name_owner() {
        let m = parse_item::<MacroDecl>("macro abc() {}");
        let macro_name = m.name_text().expect("couldn't find name on macro");

        assert_eq!("abc", macro_name);
    }

    #[test]
    fn block_as_name_owner() {
        let container = parse_item::<Container>("block abc {}");
        let container_name = container
            .name_text()
            .expect("couldn't find name on container");

        assert_eq!("abc", container_name);
    }
}
