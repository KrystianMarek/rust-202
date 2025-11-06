use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_202::functional::closures::{apply_twice, make_adder};
use rust_202::functional::iterators::{fibonacci_sequence, IteratorExamples};

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_100", |b| {
        b.iter(|| {
            let _: Vec<u64> = fibonacci_sequence().take(100).collect();
        })
    });
}

fn bench_iterator_chain(c: &mut Criterion) {
    let data = (0..1000).collect::<Vec<i32>>();

    c.bench_function("map_filter_chain", |b| {
        b.iter(|| {
            let _result = IteratorExamples::map_filter_example(black_box(data.clone()));
        })
    });
}

fn bench_fold(c: &mut Criterion) {
    let data = (0..1000).collect::<Vec<i32>>();

    c.bench_function("fold_sum", |b| {
        b.iter(|| {
            let _result = IteratorExamples::fold_example(black_box(data.clone()));
        })
    });
}

fn bench_closure_apply(c: &mut Criterion) {
    c.bench_function("apply_twice", |b| {
        b.iter(|| {
            let result = apply_twice(black_box(100), |x| x * 2);
            black_box(result)
        })
    });
}

fn bench_closure_factory(c: &mut Criterion) {
    c.bench_function("make_adder", |b| {
        b.iter(|| {
            let adder = make_adder(black_box(5));
            let result = adder(black_box(10));
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    bench_fibonacci,
    bench_iterator_chain,
    bench_fold,
    bench_closure_apply,
    bench_closure_factory
);
criterion_main!(benches);
