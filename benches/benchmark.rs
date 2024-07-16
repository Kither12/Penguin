use criterion::{black_box, criterion_group, criterion_main, Criterion};
use penguin::run_code;

pub fn parse_expression_bench(c: &mut Criterion) {
    c.bench_function("parse expression", |b| {
        b.iter(|| {
            run_code(black_box(
                "gimme a = 2 + 2;
        2 + 2 * a - 3;
        a = -10;
        2 + 2 * a - 3;",
            ))
        })
    });
}

criterion_group!(benches, parse_expression_bench);
criterion_main!(benches);
