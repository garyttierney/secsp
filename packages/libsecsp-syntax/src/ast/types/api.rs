use secsp_parser::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

use crate::ast::types::{Block, Definition};
use crate::ast::{AstChildren, AstNode, Statement};

pub enum BlockItem {
    Definition(Definition),
    Statement(Statement),
}

impl AstNode for BlockItem {
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Definition::cast(syntax.clone())
            .map(BlockItem::Definition)
            .or_else(|| Statement::cast(syntax).map(BlockItem::Statement))
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            BlockItem::Definition(def) => def.syntax(),
            BlockItem::Statement(stmt) => stmt.syntax(),
        }
    }
}

pub trait BlockItemOwner: AstNode {
    fn items(&self) -> AstChildren<BlockItem> {
        self.child::<Block>().children()
    }

    fn items_of<T: AstNode>(&self) -> AstChildren<T> {
        self.child::<Block>().children()
    }
}

pub trait NameOwner: AstNode {
    fn name(&self) -> Option<String> {
        self.syntax()
            .children_with_tokens()
            .find_map(|child| match child {
                SyntaxElement::Token(tok) => {
                    if tok.kind() == SyntaxKind::TOK_NAME {
                        Some(tok.text().to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }
}
#[cfg(test)]
mod tests {
    use crate::ast::{ContainerDef, MacroDef, SourceFile, VariableDef};

    use super::*;

    fn test_name_owner<T: NameOwner>(text: &str, name: &str) {
        let m = parse_item::<T>(text);
        let macro_name = m.name().expect("couldn't find name on node");

        assert_eq!(name, macro_name);
    }

    fn parse_item<T: AstNode>(text: &str) -> T {
        SourceFile::parse(text)
            .tree()
            .items_of::<T>()
            .nth(0)
            .unwrap()
    }

    #[test]
    fn variable_as_name_owner() {
        test_name_owner::<VariableDef>("type t;", "t");
    }

    #[test]
    fn variable_with_initializer_as_name_owner() {
        test_name_owner::<VariableDef>("type_attribute t = v;", "t");
    }

    #[test]
    fn macro_as_name_owner() {
        test_name_owner::<MacroDef>("macro abc() {}", "abc");
    }

    #[test]
    fn block_as_name_owner() {
        test_name_owner::<ContainerDef>("block abc {}", "abc");
    }
}
