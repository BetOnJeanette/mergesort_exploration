use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use power_merge::dividing_algorithms::{top_down, bottom_up};
use power_merge::merging_algorithms::quicksort_like::quicksort_like_merge;
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut group = c.benchmark_group("Merge opts");
    for i in [16, 32, 64, 128, 256, 512, 1024, 2048].iter() {
        let mut vec: Vec<i32> = (0..*i).collect();
        vec.shuffle(&mut rng);
        group.bench_with_input(BenchmarkId::new("Bottom Up Quicksort-like", i), &vec.clone(), |b, v| b.iter_batched(|| {
                let mut out = v.clone();
                out.shuffle(&mut rng);
                return out;
            }, |mut v| bottom_up::merge_sort(black_box(&mut v), quicksort_like_merge),
            criterion::BatchSize::SmallInput));

}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
