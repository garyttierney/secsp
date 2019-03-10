use std::marker::PhantomData;

use crate::token::TokenType;

#[macro_use]
pub mod types;
pub mod keywords;

mod error;
mod kinds;
pub mod visitor;

// Re-export AST types under the crate::ast root namespace.
pub use self::error::SyntaxError;
pub use self::kinds::*;
pub use self::types::{
    GreenNode, GreenNodeBuilder, SyntaxNode, SyntaxNodeChildren, SyntaxNodeRef, TreeArc,
};
use crate::ast::types::WalkEvent;
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SyntaxKind {
    /// Syntax-tree leaf for an individual token and it any optional trivia (e.g. doc comments, whitespace).
    Token(TokenType),

    /// Syntax-tree leaf for an individual token remapped to a keyword type.
    Keyword(keywords::Keyword),

    /// Syntax-tree marker for a parser error within a syntax tree.
    ParseError,

    /// Syntax-tree marker for the a list of statements within `{ ... }`.
    Block,

    /// Syntax-tree marker for a named container.
    Container,

    /// Syntax-tree marker for a list of parent-containers in a container declaration.
    ExtendsList,

    /// Syntax-tree marker for a macro definition and its body.
    MacroDef,

    /// Syntax-tree marker for a macro call statement.
    MacroCall,

    /// Syntax-tree marker for the argument list of a macro call.
    MacroArgumentList,

    /// Syntax-tree marker for individual arguments within an argument list.
    MacroArgumentListItem,

    /// Syntax-tree marker for the parameter list of within the parenthesis of a macro definition.
    MacroParamList,

    /// Syntax-tree marker for an individual item in a macro definition's parameter list.
    MacroParamListItem,

    /// Syntax-tree marker for a variable declaration.
    Variable,

    // region SyntaxKind::Expr(...)
    BinaryExpr,

    LiteralExpr,

    /// Syntax-tree marker for a sub-list expression that takes a subset of children from a named list.
    ListExpr,

    /// Syntax-tree marker for a reference expression that points to a path.
    PathExpr,

    ParenExpr,

    /// Syntax-tree marker for a unary expression with a token preceding another expression.
    PrefixExpr,

    // endregion
    // region SyntaxKind::Stmt(...)
    /// Syntax-tree marker for a conditional (if, else-if, else) statement.
    ConditionalStmt,

    // endregion
    /// Syntax-tree marker for the top level node in a files AST.
    SourceFile,
}

pub trait AstNode:
    rowan::TransparentNewType<Repr = SyntaxNode> + ToOwned<Owned = TreeArc<Self>>
{
    fn cast(syntax: &SyntaxNode) -> Option<&Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

    fn child<C: AstNode>(&self) -> &C {
        self.children().next().unwrap()
    }

    fn children<C: AstNode>(&self) -> AstChildren<C> {
        AstChildren::new(self.syntax())
    }
}

#[derive(Debug)]
pub struct AstChildren<'a, N> {
    inner: SyntaxNodeChildren<'a>,
    ph: PhantomData<N>,
}

impl<'a, N> AstChildren<'a, N> {
    fn new(parent: &'a SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<'a, N: AstNode + 'a> Iterator for AstChildren<'a, N> {
    type Item = &'a N;
    fn next(&mut self) -> Option<&'a N> {
        loop {
            if let Some(n) = N::cast(self.inner.next()?) {
                return Some(n);
            }
        }
    }
}

pub fn descendants(tree: SyntaxNodeRef) -> impl Iterator<Item = SyntaxNodeRef> {
    tree.preorder().filter_map(|event| match event {
        WalkEvent::Enter(node) => Some(node),
        WalkEvent::Leave(_) => None,
    })
}

pub fn leaf_text(tree: SyntaxNodeRef) -> Option<&SmolStr> {
    tree.leaf_text()
}
