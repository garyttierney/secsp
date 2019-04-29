#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::{Ident, TokenStream};

use quote::quote;
use syn::parse_quote::parse;
use syn::Meta;
use syn::{parse_macro_input, DeriveInput, Path};

//
//#[proc_macro_derive(AstEnum, attributes(AstKinds))]
//pub fn ast_enum(input: TokenStream) -> TokenStream {}

#[proc_macro_derive(AstType, attributes(kind))]
pub fn ast_type(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let kind = input
        .attrs
        .into_iter()
        .filter_map(|attr| attr.interpret_meta())
        .find(|attr| attr.name() == "kind")
        .map(|meta| match meta {
            Meta::Word(v) => v,
            Meta::NameValue(v) => v.ident,
            _ => panic!("No name value on {:#?}", meta),
        })
        .unwrap_or_else(|| name.clone());

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

                match NodeKind::from_syntax_kind(node.kind())? {
                    NodeKind::#kind => Some(#name::from_repr(node.into_repr())),
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
