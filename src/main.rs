#[macro_use]
extern crate clap;
extern crate secsp;

use secsp::ParseResult;

use std::io::Read;
use std::io::Write;
use std::fs::File;


mod compiler;
mod decompiler;

fn compile(input: &mut Box<Read>, output: &mut Box<Write>, print_ast: bool) {
    match secsp::parse(input) {
        ParseResult::Ok(ref statements) => {
            if print_ast {
                write!(output, "{:#?}", statements);
            } else {
                compiler::emit(output, statements);
            }
        }
        ParseResult::Err(e) => panic!("{:?}", e),
        ParseResult::Incomplete(n) => panic!("{:?}", n),
    }
}

fn decompile(input: &mut Box<Read>, output: &mut Box<Write>, print_ast: bool) {}

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
        Some(filename) => {
            Box::new(File::open(filename).unwrap_or_else(|e| {
                panic!("Unable to open file \"{}\": {}", filename, e)
            }))
        }
        None => Box::new(std::io::stdin()),
    };

    let mut output: Box<Write> = Box::new(std::io::stdout());
    let print_ast = opts.is_present("PRINT_AST");

    if opts.is_present("DECOMPILE") {
        decompile(&mut input, &mut output, print_ast);
    } else {
        compile(&mut input, &mut output, print_ast);
    }
}
