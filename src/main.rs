extern crate secsp;

mod compiler;

fn main() {
    let stdin = std::io::stdin();
    let mut input_handle = stdin.lock();

    match secsp::parse(&mut input_handle) {
        secsp::ParseResult::Ok(statements) => {
            let stdout = std::io::stdout();
            let mut output_handle = stdout.lock();

            for statement in &statements {
                compiler::show_statement(&mut output_handle, statement);
            }
        }
        secsp::ParseResult::Err(err) => panic!(err.info),
        _ => {}
    }
}