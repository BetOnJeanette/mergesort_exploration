use criterion::{criterion_group, criterion_main, Criterion};
use power_merge::powermerge;
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut vec: Vec<i32> = (0..16).collect();
    let mut rng = rand::rng();
    vec.shuffle(&mut rng);

    c.bench_function("powermerge", |b| b.iter(|| powermerge::power_merge(&mut vec.clone())));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
