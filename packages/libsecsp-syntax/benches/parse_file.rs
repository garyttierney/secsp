#[macro_use]
extern crate criterion_bencher_compat;
extern crate secsp_syntax;

use criterion_bencher_compat::Bencher;

fn parse_file(bench: &mut Bencher) {
    bench.iter(|| secsp_syntax::SourceFile::parse(include_str!("parse_file_fixture.csp")))
}

benchmark_group!(benches, parse_file);
benchmark_main!(benches);
