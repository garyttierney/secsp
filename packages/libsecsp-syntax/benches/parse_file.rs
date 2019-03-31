#[macro_use]
extern crate criterion;
extern crate secsp_syntax;

use criterion::Criterion;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_file", |b| {
        b.iter(|| secsp_syntax::parser::parse_file(include_str!("parse_file_fixture.csp")))
    });
}
