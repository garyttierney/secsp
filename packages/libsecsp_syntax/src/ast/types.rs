pub use rowan::WalkEvent;

use crate::ast::{SyntaxError, SyntaxKind};

pub type GreenNode = ::rowan::GreenNode<Types>;
pub type GreenNodeBuilder = ::rowan::GreenNodeBuilder<Types>;
pub type SyntaxNode = ::rowan::SyntaxNode<Types>;
pub type SyntaxNodeRef<'a> = &'a SyntaxNode;
pub type SyntaxNodeChildren<'a> = ::rowan::SyntaxNodeChildren<'a, Types>;
pub type TreeArc<T> = ::rowan::TreeArc<Types, T>;

#[derive(Debug, Clone, Copy)]
pub enum Types {}

impl rowan::Types for Types {
    type Kind = SyntaxKind;
    type RootData = Vec<SyntaxError>;
}

macro_rules! ast_enum {
    ($meta_item:tt $vis:vis enum $name:ident => $tykind:ident { $($kind:ident $(,)*)* }) => {

        #[derive(Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name { syntax: crate::ast::types::SyntaxNode }

        unsafe impl ::rowan::TransparentNewType for $name {
            type Repr = crate::ast::SyntaxNode;
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $tykind<'a> {
            $($kind(&'a $kind),)*
        }

        impl $name {
            pub fn kind(&self) -> $tykind {
                match self.syntax.kind() {
                    $(crate::ast::SyntaxKind::$kind => $tykind::$kind($kind::cast(&self.syntax).unwrap()),)*
                    _ => unreachable!()
                }
            }
        }

        impl crate::ast::AstNode for $name {
            fn cast(syntax: &crate::ast::SyntaxNode) -> Option<&Self> {
                use ::rowan::TransparentNewType;

                match syntax.kind() {
                    $(| crate::ast::SyntaxKind::$kind) * => Some($name::from_repr(syntax.into_repr())),
                    _ => None
                }
            }

            fn syntax(&self) -> &crate::ast::SyntaxNode {
                 &self.syntax
            }
        }

        impl ToOwned for $name {
            type Owned = crate::ast::TreeArc<$name>;
            fn to_owned(&self) -> crate::ast::TreeArc<$name> { crate::ast::TreeArc::cast(self.syntax.to_owned()) }
        }

        $(
            impl<'a> From<&'a $kind> for &'a $name {
                fn from(n: &'a $kind) -> &'a $name {
                    $name::cast(&n.syntax).unwrap()
                }
            }
        )*
    }
}

macro_rules! ast_type {
    ($vis:vis struct $name:ident: $($trait:ident $(,)*)  *$( { $($block:tt)* })*) => {
        ast_type!($vis struct $name => $name: $($trait,)* { $($($block)*)* });
    };
    ($vis:vis struct $name:ident => $kind:ident: $($trait:ident $(,)*)   *$( { $($block:tt)* })*) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name { syntax: crate::ast::types::SyntaxNode }

        unsafe impl ::rowan::TransparentNewType for $name {
            type Repr = crate::ast::SyntaxNode;
        }

        impl crate::ast::AstNode for $name {
            fn cast(node: &crate::ast::types::SyntaxNode) -> Option<&Self> {
                use ::rowan::TransparentNewType;

                if node.kind() == crate::ast::SyntaxKind::$kind {
                    Some(Self::from_repr(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &crate::ast::SyntaxNode {
                 &self.syntax
            }
        }

        impl ToOwned for $name {
            type Owned = crate::ast::TreeArc<$name>;
            fn to_owned(&self) -> crate::ast::TreeArc<$name> { crate::ast::TreeArc::cast(self.syntax.to_owned()) }
        }

        $(impl $trait for $name {})*
        $(impl $name { $($block)* })*
    };
}
