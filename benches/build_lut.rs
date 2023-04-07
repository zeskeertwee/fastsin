use criterion::{criterion_group, criterion_main, Criterion};
use fastsin::FastSin;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("FastSin::build", |b| b.iter(|| FastSin::build(2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);