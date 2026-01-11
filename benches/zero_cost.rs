use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use no_drop::wrap::{NoDrop, NoDropNoOpEmpty};

const SMALL_VALUE: u32 = 42;
const LARGE_VALUE: [u64; 9] = [0; 9];

/// Benchmark wrap/unwrap for small types (u32)
fn small_wrap_unwrap(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_wrap_unwrap");

    group.bench_function("baseline", |b| {
        b.iter(|| {
            let value = black_box(SMALL_VALUE);
            black_box(value == SMALL_VALUE)
        })
    });

    group.bench_function("release", |b| {
        b.iter(|| {
            let value = NoDropNoOpEmpty::wrap(black_box(SMALL_VALUE));
            black_box(value.unwrap() == SMALL_VALUE)
        })
    });

    group.bench_function("no_op", |b| {
        b.iter(|| {
            let value = NoDrop::wrap(black_box(SMALL_VALUE));
            black_box(value.unwrap() == SMALL_VALUE)
        })
    });

    group.finish();
}

/// Benchmark wrap/unwrap for large types (72 bytes)
fn large_wrap_unwrap(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_wrap_unwrap");

    group.bench_function("baseline", |b| {
        b.iter(|| {
            let value = black_box(LARGE_VALUE);
            black_box(value == LARGE_VALUE)
        })
    });

    group.bench_function("release", |b| {
        b.iter(|| {
            let value = NoDropNoOpEmpty::wrap(black_box(LARGE_VALUE));
            black_box(value.unwrap() == LARGE_VALUE)
        })
    });

    group.bench_function("debug", |b| {
        b.iter(|| {
            let value = NoDrop::wrap(black_box(LARGE_VALUE));
            black_box(value.unwrap() == LARGE_VALUE)
        })
    });

    group.finish();
}

criterion_group!(benches, small_wrap_unwrap, large_wrap_unwrap);
criterion_main!(benches);
