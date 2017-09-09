#[macro_use]
extern crate clap;
extern crate secsp;

use std::io::Read;
use std::io::Write;
use std::fs::File;

mod compiler;
mod decompiler;

fn compile(input: &mut Box<Read>, output: &mut Box<Write>) {
    match secsp::parse(input) {
        secsp::ParseResult::Ok(statements) => {}
        secsp::ParseResult::Err(e) => panic!("{:?}", e),
        _ => {}
    };
}


fn main() {
    let opts = clap_app!(cspc =>
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

    match secsp::parse(&mut input) {
        secsp::ParseResult::Ok(statements) => {
            if opts.is_present("PRINT_AST") {
                print!("{:#?}", statements);
            } else {
                let stdout = std::io::stdout();
                let mut output_handle = stdout.lock();

                for statement in &statements {
                    compiler::show_statement(&mut output_handle, statement);
                }
            }
        }
        secsp::ParseResult::Err(err) => println!("{}", err.info),
        _ => {}
    }
}
