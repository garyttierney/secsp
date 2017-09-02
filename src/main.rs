extern crate secsp;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let parse_result = secsp::parse(&mut handle);

    for statement in &parse_result.statements {
        println!("{:#?}", statement);
    }
}