use std::marker::PhantomData;

use itertools::Itertools;
pub use rowan::WalkEvent;
use text_unit::TextUnit;

use secsp_parser::syntax::{SyntaxNode, SyntaxNodeChildren};

pub use self::types::*;

#[cfg(test)]
mod testing;

mod types;
pub mod visitor;

pub trait AstNode {
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

    fn child<C: AstNode>(&self) -> C {
        self.children().next().unwrap()
    }

    fn children<C: AstNode>(&self) -> AstChildren<C> {
        AstChildren::new(self.syntax().clone())
    }
}

#[derive(Debug)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.by_ref().find_map(N::cast)
    }
}

pub fn descendants(tree: &SyntaxNode) -> impl Iterator<Item = SyntaxNode> {
    tree.preorder().filter_map(|event| match event {
        WalkEvent::Enter(node) => Some(node),
        WalkEvent::Leave(_) => None,
    })
}

pub fn ancestors_at_offset(
    node: &SyntaxNode,
    offset: TextUnit,
) -> impl Iterator<Item = SyntaxNode> {
    node.token_at_offset(offset)
        .map(|token| token.parent().ancestors())
        .kmerge_by(|node1, node2| node1.text_range().len() < node2.text_range().len())
}

pub fn find_node_at_offset<N: AstNode>(syntax: &SyntaxNode, offset: TextUnit) -> Option<N> {
    ancestors_at_offset(syntax, offset).find_map(N::cast)
}
