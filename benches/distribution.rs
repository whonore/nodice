#![allow(clippy::unwrap_used)]

use criterion::{Criterion, criterion_group, criterion_main};
use nodice::{expr::Expr, stats::Stats};

fn parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("distribution");

    let exprs = [
        Expr::scalar(6),
        Expr::die(6),
        Expr::die(6).repeat(2).unwrap(),
        Expr::die(6) + Expr::die(4),
    ];

    for expr in exprs {
        group.bench_with_input(expr.to_string(), &expr, |b, expr| {
            b.iter(|| expr.distribution());
        });
    }

    group.finish();
}

criterion_group!(benches, parse);
criterion_main!(benches);
