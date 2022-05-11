use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::Rng;
use max2::{sort_max2, fold_max2, loop_max2, transform_max2, par_max2};

fn bench_floats(c: &mut Criterion) {
    let mut group = c.benchmark_group("floats");
    let mut rng = rand::thread_rng();

    let samples: Vec<Vec<f64>> = vec![
        (0..10).map(|_| rng.gen_range(0.0, 10.0)).collect(),
        (0..100).map(|_| rng.gen_range(0.0, 100.0)).collect(),
        (0..1000).map(|_| rng.gen_range(0.0, 1000.0)).collect(),
        (0..10000).map(|_| rng.gen_range(0.0, 10000.0)).collect(),
    ];

    for sample in samples {
        group.bench_with_input(BenchmarkId::new("max2-sort", sample.len()), &sample, |b, sample| {
            b.iter(|| sort_max2(sample));
        });
        group.bench_with_input(BenchmarkId::new("max2-fold", sample.len()), &sample, |b, sample| {
            b.iter(|| fold_max2(sample) );
        });
        group.bench_with_input(BenchmarkId::new("loop-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| loop_max2(sample) );
        });
        group.bench_with_input(BenchmarkId::new("xform-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| transform_max2(sample) );
        });
        group.bench_with_input(BenchmarkId::new("par-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| par_max2(sample) );
        });
    }

    group.finish();
}

fn bench_ints(c: &mut Criterion) {
    let mut group = c.benchmark_group("ints");
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
        group.bench_with_input(BenchmarkId::new("loop-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| loop_max2(sample) );
        });
        group.bench_with_input(BenchmarkId::new("xform-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| transform_max2(sample) );
        });
        group.bench_with_input(BenchmarkId::new("par-max2", sample.len()), &sample, |b, sample| {
            b.iter(|| par_max2(sample) );
        });
    }

    group.finish();
}

fn bench_huge(c: &mut Criterion) {
    let mut group = c.benchmark_group("yuge");
    let mut rng = rand::thread_rng();

    let sample: Vec<f32> = (0..10_000_000u64).map(|_| rng.gen_range(f32::MIN, f32::MAX)).collect();

    group.bench_with_input(BenchmarkId::new("xform-max2", sample.len()), &sample, |b, sample| {
        b.iter(|| transform_max2(sample) );
    });
    group.bench_with_input(BenchmarkId::new("par-max2", sample.len()), &sample, |b, sample| {
        b.iter(|| par_max2(sample) );
    });
}

criterion_group!(benches, bench_ints, bench_floats, bench_huge);
criterion_main!(benches);