#[macro_use]
extern crate clap;
extern crate secsp_syntax;

use std::io::Read;
use std::io::Write;
use std::fs::File;

mod compiler;
mod decompiler;

fn decompile<I: Read, O: Write>(input: &mut I, output: &mut O, print_ast: bool) {}

fn main() {
    let opts = clap_app!(cspc =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Compiles C-style policy to CIL")
        (@arg DECOMPILE: -d --decompile "Decompile CIL sources into equivalent CSP")
        (@arg PRINT_AST: -s --show_ast "Print the parsed AST to stdout")
        (@arg INPUT: -f --file +takes_value "Sets the input file to use")
    ).get_matches();

    let mut input: Box<Read> = match opts.value_of("INPUT") {
        Some(filename) => Box::new(
            File::open(filename)
                .unwrap_or_else(|e| panic!("Unable to open file \"{}\": {}", filename, e)),
        ),
        None => Box::new(std::io::stdin()),
    };

    let mut output: Box<Write> = Box::new(std::io::stdout());
    let print_ast = opts.is_present("PRINT_AST");
}
