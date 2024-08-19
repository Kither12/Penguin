use criterion::{black_box, criterion_group, criterion_main, Criterion};
use penguin::run_code;
use std::fs;

pub fn bench_1(c: &mut Criterion) {
    let code: String = fs::read_to_string("examples/prime.pn").unwrap();
    c.bench_function("bench_1", |b| b.iter(|| run_code(black_box(&code))));
}
pub fn bench_2(c: &mut Criterion) {
    let code: String = fs::read_to_string("examples/sum.pn").unwrap();
    c.bench_function("bench_2", |b| b.iter(|| run_code(black_box(&code))));
}

criterion_group!(benches, bench_1, bench_2);
criterion_main!(benches);
