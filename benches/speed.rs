use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastsin::FastSin;

pub fn criterion_benchmark(c: &mut Criterion) {
    let fs = FastSin::build(2);

    c.bench_function("FastSin::sin", |b| b.iter(|| fs.sin(black_box(25.831))));
    c.bench_function("FastSin::cos", |b| b.iter(|| fs.cos(black_box(25.831))));

    c.bench_function("f64::sin", |b| b.iter(|| f64::sin(black_box(25.831))));
    c.bench_function("f64::cos", |b| b.iter(|| f64::cos(black_box(25.831))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);