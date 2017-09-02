extern crate secsp;

mod compiler;

fn main() {
    let stdin = std::io::stdin();
    let mut input_handle = stdin.lock();
    let parse_result = secsp::parse(&mut input_handle);

    let stdout = std::io::stdout();
    let mut output_handle = stdout.lock();

    for statement in &parse_result.statements {
        compiler::show_statement(&mut output_handle, statement);
    }
}