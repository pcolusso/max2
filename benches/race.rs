use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::Rng;
use max2::{sort_max2, fold_max2};

fn bench_max2(c: &mut Criterion) {
    let mut group = c.benchmark_group("max2");
    let mut rng = rand::thread_rng();

    let samples: Vec<Vec<u64>> = vec![
        (0..10).map(|_| rng.gen_range(0, 10)).collect(),
        (0..100).map(|_| rng.gen_range(0, 100)).collect(),
        (0..1000).map(|_| rng.gen_range(0, 1000)).collect(),
        (0..10000).map(|_| rng.gen_range(0, 10000)).collect(),
    ];

    for sample in samples {
        group.bench_with_input(BenchmarkId::new("max2-sort", sample.len()), &sample, |b, sample| {
            b.iter(|| sort_max2(sample));
        });
        group.bench_with_input(BenchmarkId::new("max2-fold", sample.len()), &sample, |b, sample| {
            b.iter(|| fold_max2(sample) );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_max2);
criterion_main!(benches);