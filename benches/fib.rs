use bench_action::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_fib_20(c: &mut Criterion) {
    c.bench_function("BenchFib20", move |b| {
        b.iter(|| {
            let _ = fibonacci(20);
        });
    });
}

criterion_group!(benches, bench_fib_20,);
criterion_main!(benches);
