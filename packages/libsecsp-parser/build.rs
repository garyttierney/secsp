extern crate inflector;
#[macro_use]
extern crate quote;
extern crate ron;
extern crate serde;
extern crate syn;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use inflector::cases::screamingsnakecase::to_screaming_snake_case;

use inflector::cases::camelcase::to_camel_case;
use inflector::cases::pascalcase::to_pascal_case;
use ron::de::from_reader;

#[derive(Debug, serde::Deserialize)]
pub struct SyntaxKinds {
    pub keywords: Vec<String>,
    pub terminals: Vec<(String, String)>,
    pub tokens: Vec<String>,
    pub nodes: Vec<String>,
}

struct Keywords {
    literals: Vec<String>,
    syntax_names: Vec<quote::__rt::Ident>,
    kw_names: Vec<quote::__rt::Ident>,
}

struct Terminals {
    literal: Vec<String>,
    syntax_names: Vec<quote::__rt::Ident>,
}

impl SyntaxKinds {
    fn keywords(&self) -> Keywords {
        let literals = self.keywords.clone();

        let syntax_names = self
            .keywords
            .iter()
            .map(|kw| to_ident(to_screaming_snake_case(kw.as_str()), Some("KW")))
            .collect();

        let kw_names = self
            .keywords
            .iter()
            .map(|kw| to_ident(to_pascal_case(kw.as_str()), None))
            .collect();

        Keywords {
            literals,
            syntax_names,
            kw_names,
        }
    }

    fn tokens_and_nodes(&self) -> Vec<quote::__rt::Ident> {
        self.tokens
            .iter()
            .chain(self.nodes.iter())
            .map(|ty| format_ident!("{}", to_screaming_snake_case(&ty)))
            .collect()
    }
}

const BUILD_ERR: &'static str = "Build Script must be run by cargo as part of the build process";
const READ_ERR: &'static str = "Couldn't parse syntax kind definitions";
const IO_ERR: &'static str = "Couldn't open syntax kind definitions";

fn to_ident<S: AsRef<str>>(value: S, prefix: Option<&str>) -> quote::__rt::Ident {
    match prefix {
        Some(p) => format_ident!("{}_{}", p, value.as_ref()),
        None => format_ident!("{}", value.as_ref()),
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect(BUILD_ERR);
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect(BUILD_ERR);
    let syntax_input_path = Path::new(&project_dir).join("src/syntax.ron");

    let syntax: SyntaxKinds = {
        let text = fs::read_to_string(syntax_input_path.clone()).expect(IO_ERR);
        ron::de::from_str(&text).expect(READ_ERR)
    };

    let keywords = syntax.keywords();
    let kw_literals = keywords.literals;
    let kw_syntax_kinds = keywords.syntax_names;
    let kw_kinds = keywords.kw_names;

    let mut kinds = syntax.tokens_and_nodes();
    kinds.extend(kw_syntax_kinds.clone());

    let term_values = syntax.terminals.iter().map(|t| t.0.clone());
    let term_names = syntax.terminals.iter().map(|t| format_ident!("{}", t.1));

    let syntax_output_path = Path::new(&project_dir).join("src/syntax-generated.rs");
    let syntax_output = quote! {
        #[repr(u16)]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum SyntaxKind {
            #(#kinds),*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum KeywordKind {
            #(#kw_kinds),*
        }

        impl Into<SyntaxKind> for KeywordKind {
            fn into(self) -> SyntaxKind {
                match self {
                    #(KeywordKind::#kw_kinds => SyntaxKind::#kw_syntax_kinds),*
                }
            }
        }

        impl AsRef<str> for KeywordKind {
            fn as_ref(&self) -> &str {
                match self {
                    #(KeywordKind::#kw_kinds => #kw_literals),*
                }
            }
        }

        impl ::std::str::FromStr for KeywordKind {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#kw_literals => Ok(KeywordKind::#kw_kinds),)*
                    _ => Err(())
                }
            }
        }

        macro_rules! tok {
            #((#term_values) => {
                $crate::syntax::SyntaxKind::#term_names
            };)*
        }
    };

    fs::write(syntax_output_path, format!("{}", syntax_output.to_string())).expect(IO_ERR);

    println!("cargo:rerun-if-changed={:?}", syntax_input_path);
}
