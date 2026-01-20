use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_rational::Ratio;
use rat_trig_rs::geometry::*;
use rat_trig_rs::trigonom::*;

fn bench_quadrance(c: &mut Criterion) {
    c.bench_function("quadrance_i64", |b| {
        b.iter(|| quadrance(black_box((1, 2)), black_box((4, 6))))
    });

    c.bench_function("quadrance_f64", |b| {
        b.iter(|| quadrance(black_box((1.0, 2.0)), black_box((4.0, 6.0))))
    });

    c.bench_function("quadrance_rational", |b| {
        let p1 = (Ratio::new(1, 2), Ratio::new(3, 4));
        let p2 = (Ratio::new(5, 6), Ratio::new(7, 8));
        b.iter(|| quadrance(black_box(p1), black_box(p2)))
    });
}

fn bench_spread(c: &mut Criterion) {
    c.bench_function("spread_i64", |b| {
        b.iter(|| spread(black_box((1, 2)), black_box((3, 4))))
    });

    c.bench_function("spread_f64", |b| {
        b.iter(|| spread(black_box((1.0, 2.0)), black_box((3.0, 4.0))))
    });

    c.bench_function("spread_rational", |b| {
        let v1 = (Ratio::new(1, 2), Ratio::new(3, 4));
        let v2 = (Ratio::new(5, 6), Ratio::new(7, 8));
        b.iter(|| spread(black_box(v1), black_box(v2)))
    });
}

fn bench_cross(c: &mut Criterion) {
    c.bench_function("cross_i64", |b| {
        b.iter(|| cross(black_box((1, 2)), black_box((3, 4))))
    });

    c.bench_function("cross_f64", |b| {
        b.iter(|| cross(black_box((1.0, 2.0)), black_box((3.0, 4.0))))
    });
}

fn bench_archimedes(c: &mut Criterion) {
    c.bench_function("archimedes_i64", |b| {
        b.iter(|| archimedes(black_box(&1), black_box(&2), black_box(&3)))
    });

    c.bench_function("archimedes_f64", |b| {
        b.iter(|| archimedes(black_box(&1.0), black_box(&2.0), black_box(&3.0)))
    });

    c.bench_function("archimedes_rational", |b| {
        let q1 = Ratio::new(1, 2);
        let q2 = Ratio::new(3, 4);
        let q3 = Ratio::new(5, 6);
        b.iter(|| archimedes(black_box(&q1), black_box(&q2), black_box(&q3)))
    });
}

fn bench_3d_operations(c: &mut Criterion) {
    c.bench_function("quadrance3d_i64", |b| {
        b.iter(|| quadrance3d(black_box((1, 2, 3)), black_box((4, 5, 6))))
    });

    c.bench_function("cross3d_i64", |b| {
        b.iter(|| cross3d(black_box((1, 2, 3)), black_box((4, 5, 6))))
    });

    c.bench_function("spread3d_i64", |b| {
        b.iter(|| spread3d(black_box((1, 2, 3)), black_box((4, 5, 6))))
    });
}

fn bench_triangle_operations(c: &mut Criterion) {
    c.bench_function("triangle_quadrances", |b| {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        b.iter(|| triangle.quadrances())
    });

    c.bench_function("triangle_area", |b| {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        b.iter(|| triangle.area())
    });

    c.bench_function("triangle_twist", |b| {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        b.iter(|| triangle.twist())
    });
}

fn bench_advanced_functions(c: &mut Criterion) {
    c.bench_function("twist", |b| {
        b.iter(|| twist(black_box((0, 0)), black_box((1, 0)), black_box((0, 1))))
    });

    c.bench_function("turn", |b| {
        b.iter(|| {
            turn(
                black_box((0.0, 0.0)),
                black_box((1.0, 0.0)),
                black_box((1.0, 1.0)),
            )
        })
    });

    c.bench_function("dilatation", |b| {
        b.iter(|| dilatation(black_box((1, 0)), black_box((2, 0))))
    });

    c.bench_function("cosine_law", |b| {
        b.iter(|| cosine_law(black_box(2.0), black_box(1.0), black_box(1.0)))
    });
}

criterion_group!(
    benches,
    bench_quadrance,
    bench_spread,
    bench_cross,
    bench_archimedes,
    bench_3d_operations,
    bench_triangle_operations,
    bench_advanced_functions
);
criterion_main!(benches);
