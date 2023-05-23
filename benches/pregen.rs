use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tejuino::pregen::*;

pub fn benchmarks(c: &mut Criterion) {
    c.bench_function("pregen", |b| b.iter(|| {
        let p = Pregen::init();
        black_box(p)
    }));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
