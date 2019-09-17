use std::marker::PhantomData;

pub use rowan::WalkEvent;

use secsp_parser::syntax::{SyntaxNode, SyntaxNodeChildren};

pub use self::types::*;

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
