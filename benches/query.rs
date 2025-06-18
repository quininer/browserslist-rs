use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_query(c: &mut Criterion) {
    let opts = browserslist::Opts::default();

    c.bench_function("simple query", |b| {
        let query = ["> 0.5%", "last 2 versions", "Firefox ESR", "not dead"];

        b.iter(|| {
            let result = browserslist::resolve(black_box(query.iter()), black_box(&opts)).unwrap();
            black_box(result);
        });
    });
}

criterion_group!(query, bench_query);
criterion_main!(query);
