#![recursion_limit = "128"]
extern crate darling;
extern crate proc_macro;

use proc_macro::TokenStream;

use darling::{ast, FromDeriveInput, FromMeta, FromVariant};
use syn::{parse_macro_input, DeriveInput};

use quote::quote;
use quote::ToTokens;

#[derive(Debug, FromVariant)]
#[darling(from_ident, attributes(ast))]
#[allow(dead_code)]
struct AstEnumVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<syn::Type>,
    kind: Option<String>,
}

impl From<syn::Ident> for AstEnumVariant {
    fn from(ident: syn::Ident) -> Self {
        AstEnumVariant {
            ident,
            kind: Default::default(),
            fields: darling::ast::Style::Unit.into(),
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct AstEnum {
    ident: syn::Ident,
    data: ast::Data<AstEnumVariant, ()>,
}

impl ToTokens for AstEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let AstEnum {
            ref ident,
            ref data,
            ..
        } = *self;

        let variants = data.as_ref().take_enum().expect("Should never be struct");

        let mut kind_matcher = proc_macro2::TokenStream::new();
        let mut variant_matcher = proc_macro2::TokenStream::new();

        let field_list: Vec<_> = variants
            .into_iter()
            .map(|variant| {
                let ast_struct_name = match &variant.fields.fields[0] {
                    syn::Type::Path(typ) => quote!(#typ),
                    _ => unimplemented!(),
                };

                let ast_kind_name = variant
                    .kind
                    .as_ref()
                    .map(|v| syn::Ident::from_string(v).expect("Invalid identifier"))
                    .expect("No kind attribute found");

                let enum_kind_name = variant.ident.clone();

                (enum_kind_name, ast_struct_name, ast_kind_name)
            })
            .collect();

        for (enum_name, ast_name, kind_name) in &field_list {
            variant_matcher.extend(quote! {
                #ident::#enum_name(e) => e.syntax(),
            });

            kind_matcher.extend(quote! {
                secsp_parser::syntax::SyntaxKind::#kind_name => Some(#ident::#enum_name(#ast_name(syntax))),
            });

            tokens.extend(quote! {
                impl From<#ast_name> for #ident {
                    fn from(node: #ast_name) -> #ident {
                        #ident::#enum_name(node)
                    }
                }
            });
        }

        tokens.extend(quote! {
            impl crate::ast::AstNode for #ident {
                fn syntax(&self) -> &secsp_parser::syntax::SyntaxNode {
                    match self {
                        #variant_matcher
                    }
                }

                fn cast(syntax: secsp_parser::syntax::SyntaxNode) -> Option<Self> {
                    match syntax.kind() {
                        #kind_matcher
                        _ => None
                    }
                }
            }
        });
    }
}

#[proc_macro_derive(AstEnum, attributes(ast))]
pub fn derive_ast_enum(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let ast_enum = AstEnum::from_derive_input(&input).expect("Unable to parse AstEnum type");

    TokenStream::from(quote!(#ast_enum))
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_any), attributes(ast))]
struct AstType {
    ident: syn::Ident,
    kind: Option<String>,
}

impl ToTokens for AstType {
    fn to_tokens(&self, output: &mut proc_macro2::TokenStream) {
        let AstType {
            ref ident,
            ref kind,
        } = *self;

        let kind_ident = kind
            .as_ref()
            .map(|v| syn::Ident::from_string(v).expect("Invalid identifier"))
            .expect("No kind attribute found");

        output.extend(quote! {
            impl crate::ast::AstNode for #ident {
                fn syntax(&self) -> &SyntaxNode {
                    &self.0
                }

                fn cast(syntax: secsp_parser::syntax::SyntaxNode) -> Option<Self> {
                    match syntax.kind() {
                        secsp_parser::syntax::SyntaxKind::#kind_ident => Some(#ident(syntax)),
                        _ => None
                    }
                }
            }
        });
    }
}

#[proc_macro_derive(AstType, attributes(ast))]
pub fn derive_ast_type(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let ast_type = AstType::from_derive_input(&input).expect("Unable to parse AstType type");

    TokenStream::from(quote!(#ast_type))
}
