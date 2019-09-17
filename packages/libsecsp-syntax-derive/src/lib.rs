#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;

use inflector::Inflector;
use syn::{parse_macro_input, DeriveInput, Path};
use syn::{Meta, NestedMeta};

use quote::quote;

//
//#[proc_macro_derive(AstEnum, attributes(AstKinds))]
//pub fn ast_enum(input: TokenStream) -> TokenStream {}

#[proc_macro_derive(AstType, attributes(kind))]
pub fn ast_type(input: TokenStream) -> TokenStream {
    use inflector::Inflector;

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
            let kind_text = format!("NODE_{}", kind.to_string().to_screaming_snake_case());
            let kind_ident = proc_macro2::Ident::new(&kind_text, kind.span());

            quote! { secsp_parser::syntax::SyntaxKind::#kind_ident => Some(#name(node)), }
        })
        .collect();

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
         impl crate::ast::AstNode for #name {
            fn cast(node: secsp_parser::syntax::SyntaxNode) -> Option<Self> {
                match node.kind() {
                    #(#kinds_expr)*
                    _ => None
                }
            }

            fn syntax(&self) -> &secsp_parser::syntax::SyntaxNode {
                &self.0
            }
         }
    };

    TokenStream::from(expanded)
}
