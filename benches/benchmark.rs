use chess_lang::parser::{ast::ASTNode, parser::parse_ast};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn parse_expression_bench(c: &mut Criterion) {
    c.bench_function("parse expression", |b| {
        b.iter(|| parse_ast(black_box("2 + (2 + 2) * 4;")))
    });
}
pub fn parse__evaluate_expression_bench(c: &mut Criterion) {
    // c.bench_function("evaluate expression", |b| {
    //     b.iter(|| {
    //         let ast = parse_ast("2 + (2 + 2) * 4;").unwrap();
    //         if let ASTNode::Expr(expr) = ast.as_ref() {
    //             expr.evaluation();
    //         }
    //     })
    // });
}

criterion_group!(
    benches,
    parse_expression_bench,
    parse__evaluate_expression_bench
);
criterion_main!(benches);
