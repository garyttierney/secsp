#[macro_use]
use std::marker::PhantomData;

pub use rowan::{TreeRoot, WalkEvent};

use crate::ast::{SyntaxError, SyntaxKind};
use crate::token::TokenType;
use smol_str::SmolStr;

pub type OwnedRoot = ::rowan::OwnedRoot<Types>;
pub type RefRoot<'a> = ::rowan::RefRoot<'a, Types>;
pub type GreenNode = ::rowan::GreenNode<Types>;
pub type GreenNodeBuilder = ::rowan::GreenNodeBuilder<Types>;
pub type SyntaxNode<R: TreeRoot<Types> = OwnedRoot> = ::rowan::SyntaxNode<Types, R>;
pub type SyntaxNodeRef<'a> = SyntaxNode<RefRoot<'a>>;
pub type SyntaxNodeChildren<'a> = ::rowan::SyntaxNodeChildren<Types, RefRoot<'a>>;

#[derive(Debug, Clone, Copy)]
pub enum Types {}

impl rowan::Types for Types {
    type Kind = SyntaxKind;
    type RootData = Vec<SyntaxError>;
}

macro_rules! ast_enum {
    ($meta_item:tt $vis:vis enum $name:ident { $($kind:ident $(,)*)* }) => {
        #[derive(Debug, Copy, Clone)]
        pub enum $name<'a> {
            $($kind($kind<'a>),)*
        }

        impl<'a> $name<'a> {
            pub fn new(kind: crate::ast::SyntaxKind, syntax: crate::ast::SyntaxNodeRef<'a>) -> Self {
                match syntax.kind() {
                    $(crate::ast::SyntaxKind::$kind => $name::$kind($kind::new(crate::ast::SyntaxKind::$kind, syntax)),)*
                    _ => unreachable!()
                }
            }
        }

        impl<'a> crate::ast::AstNode<'a> for $name<'a> {
            fn cast(syntax: crate::ast::SyntaxNodeRef<'a>) -> Option<Self> {
                match syntax.kind() {
                    $(crate::ast::SyntaxKind::$kind => $kind::cast(syntax).map(|i| $name::$kind(i)),)*
                    _ => None
                }
            }

            fn syntax(self) -> crate::ast::SyntaxNodeRef<'a> {
                match self {
                    $($name::$kind(inner) => inner.syntax,)*
                }
            }
        }
    }
}

macro_rules! ast_type {
    ($vis:vis struct $name:ident => $kind:ident: $($trait:ident $(,)*)   *$( { $($block:tt)* })*) => {
        #[derive(Debug, Copy, Clone,)]
        pub struct $name<R: rowan::TreeRoot<crate::ast::types::Types> = crate::ast::types::OwnedRoot> {
            pub (crate) syntax: crate::ast::types::SyntaxNode<R>
        }

        pub type $kind<'a> = $name<crate::ast::types::RefRoot<'a>>;

        impl<R: ::rowan::TreeRoot<crate::ast::types::Types>> $name<R> {
            pub fn borrowed(&self) -> $kind {
                $name { syntax: self.syntax.borrowed() }
            }

            pub fn owned(&self) -> $name {
                $name { syntax: self.syntax.owned() }
            }
        }

        impl<R1: ::rowan::TreeRoot<crate::ast::types::Types>, R2: ::rowan::TreeRoot<crate::ast::types::Types>> PartialEq<$name<R1>>
            for $name<R2>
        {
            fn eq(&self, other: &$name<R1>) -> bool {
                self.syntax == other.syntax
            }
        }
        impl<R: ::rowan::TreeRoot<crate::ast::types::Types>> Eq for $name<R> {}
        impl<R: ::rowan::TreeRoot<crate::ast::types::Types>> ::std::hash::Hash for $name<R> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.syntax.hash(state)
            }
        }

        impl<'a> crate::ast::AstNode<'a> for $kind<'a> {
            fn cast(syntax: crate::ast::types::SyntaxNodeRef<'a>) -> Option<Self> {
                match syntax.kind() {
                    crate::ast::SyntaxKind::$kind => Some($kind { syntax }),
                    _ => None,
                }
            }
            fn syntax(self) -> crate::ast::types::SyntaxNodeRef<'a> {
                self.syntax
            }
        }

        $(impl<'a> $trait<'a> for $kind<'a> {})*
        $(impl<'a> $kind<'a> { $($block)* })*

        impl<'a> $kind<'a> {
            pub fn new(_: crate::ast::SyntaxKind, syntax: crate::ast::SyntaxNodeRef<'a>) -> Self {
                Self { syntax }
            }
        }
    }
}
