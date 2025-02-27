use criterion::{criterion_group, criterion_main, Criterion};
use testing::{hashmap_prealloc, hashmap_simple};

const ITER: usize = 100000;

fn hashmap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMaps");
    group.bench_function("Hashmap Simple", |b| b.iter(|| hashmap_simple(ITER)));
    group.bench_function("HashMap Prealloc", |b| b.iter(|| hashmap_prealloc(ITER)));
    group.finish();
}

criterion_group!(hash, hashmap_bench);
criterion_main!(hash);
