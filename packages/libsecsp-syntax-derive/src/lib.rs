#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Ident, Path};
use syn::{Meta, NestedMeta};

//
//#[proc_macro_derive(AstEnum, attributes(AstKinds))]
//pub fn ast_enum(input: TokenStream) -> TokenStream {}

#[proc_macro_derive(AstType, attributes(kind))]
pub fn ast_type(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let kinds = input
        .attrs
        .into_iter()
        .filter_map(|attr| attr.interpret_meta())
        .find(|attr| attr.name() == "kind")
        .map(|meta| match meta {
            Meta::List(list) => list
                .nested
                .into_iter()
                .map(|n| match n {
                    NestedMeta::Meta(meta) => match meta {
                        Meta::Word(id) => id,
                        _ => panic!("Expected an identifier"),
                    },
                    _ => panic!("Expected meta info"),
                })
                .collect(),
            _ => panic!("Expected a list of node kinds"),
        })
        .unwrap_or_else(|| vec![name.clone()]);

    let kinds_expr: Vec<proc_macro2::TokenStream> = kinds
        .iter()
        .map(|kind| {
            quote! { NodeKind::#kind => Some(#name::from_repr(node.into_repr())), }
        })
        .collect();

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
         unsafe impl rowan::TransparentNewType for #name {
            type Repr = rowan::SyntaxNode;
         }

        impl ToOwned for #name {
            type Owned = rowan::TreeArc<#name>;
            fn to_owned(&self) -> rowan::TreeArc<#name> { rowan::TreeArc::cast(self.0.to_owned()) }
        }

         impl crate::ast::AstNode for #name {
            fn cast(node: &rowan::SyntaxNode) -> Option<&Self> {
                use rowan::TransparentNewType;
                use secsp_parser::syntax::SyntaxKindClass;

                match NodeKind::from_kind(node.kind())? {
                    #(#kinds_expr)*
                    _ => None
                }
            }

            fn syntax(&self) -> &rowan::SyntaxNode {
                &self.0
            }
         }
    };

    TokenStream::from(expanded)
}
