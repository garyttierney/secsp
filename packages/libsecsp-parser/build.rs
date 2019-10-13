extern crate inflector;
#[macro_use]
extern crate quote;
extern crate ron;
extern crate serde;
extern crate syn;

use std::path::Path;
use std::{env, fs};

use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;

#[derive(Debug, serde::Deserialize)]
struct SyntaxKinds {
    keywords: Vec<String>,
    terminals: Vec<(String, String)>,
    tokens: Vec<String>,
    nodes: Vec<String>,
}

struct Keywords {
    kw_literals: Vec<String>,
    kw_syntax_kinds: Vec<quote::__rt::Ident>,
    kw_kinds: Vec<quote::__rt::Ident>,
}

struct Terminals {
    terminal_literals: Vec<String>,
    terminal_kinds: Vec<quote::__rt::Ident>,
}

impl SyntaxKinds {
    fn keywords(&self) -> Keywords {
        let literals = self.keywords.clone();

        let syntax_names = self
            .keywords
            .iter()
            .map(|kw| to_ident(to_screaming_snake_case(kw.as_str()), Some("KW")))
            .collect();

        let kw_kinds = self
            .keywords
            .iter()
            .map(|kw| to_ident(to_pascal_case(kw.as_str()), None))
            .collect();

        Keywords {
            kw_literals: literals,
            kw_syntax_kinds: syntax_names,
            kw_kinds,
        }
    }

    fn terminals(&self) -> Terminals {
        let terminal_literals = self.terminals.iter().map(|t| t.0.clone()).collect();
        let terminal_kinds = self
            .terminals
            .iter()
            .map(|t| format_ident!("{}", t.1))
            .collect();

        Terminals {
            terminal_literals,
            terminal_kinds,
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
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect(BUILD_ERR);
    let syntax_input_path = Path::new(&project_dir).join("src/syntax.ron");

    let syntax: SyntaxKinds = {
        let text = fs::read_to_string(syntax_input_path.clone()).expect(IO_ERR);
        ron::de::from_str(&text).expect(READ_ERR)
    };

    let Keywords {
        kw_literals,
        kw_syntax_kinds,
        kw_kinds,
    } = syntax.keywords();

    let Terminals {
        terminal_literals,
        terminal_kinds,
    } = syntax.terminals();

    let mut kinds = syntax.tokens_and_nodes();
    kinds.extend(kw_syntax_kinds.clone());

    let syntax_output_path = Path::new(&project_dir).join("src/syntax-generated.rs");
    let rules_output_path = Path::new(&project_dir).join("src/rules-generated.rs");
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
    };

    let rules_output = quote! {
        #[macro_export]
        macro_rules! tok {
            #((#terminal_literals) => {
                $crate::syntax::SyntaxKind::#terminal_kinds
            };)*
        }

        #[macro_export]
        macro_rules! kw {
            #((#kw_literals) => {
                $crate::syntax::KeywordKind::#kw_kinds
            };)*
        }
    };

    fs::write(syntax_output_path, format!("{}", syntax_output.to_string())).expect(IO_ERR);
    fs::write(rules_output_path, format!("{}", rules_output.to_string())).expect(IO_ERR);

    println!("cargo:rerun-if-changed={:?}", syntax_input_path);
}
