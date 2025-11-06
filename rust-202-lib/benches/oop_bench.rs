use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_202::oop::composition::{Circle, HasArea, Rectangle};
use rust_202::oop::patterns::singleton::Config;

fn bench_circle_area(c: &mut Criterion) {
    c.bench_function("circle_area", |b| {
        let circle = Circle::new(5.0);
        b.iter(|| black_box(circle.area()))
    });
}

fn bench_rectangle_area(c: &mut Criterion) {
    c.bench_function("rectangle_area", |b| {
        let rect = Rectangle::new(10.0, 20.0);
        b.iter(|| black_box(rect.area()))
    });
}

fn bench_singleton_access(c: &mut Criterion) {
    Config::set("test_key", "test_value");

    c.bench_function("singleton_read", |b| {
        b.iter(|| black_box(Config::get("test_key")))
    });
}

criterion_group!(
    benches,
    bench_circle_area,
    bench_rectangle_area,
    bench_singleton_access
);
criterion_main!(benches);
