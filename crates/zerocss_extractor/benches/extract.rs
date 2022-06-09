use zerocss_extractor::extract;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn extract_benchmark(c: &mut Criterion) {
    c.bench_function("extract 40 chars", |b| {
        b.iter(|| extract(black_box("<div class=\"foo-3 bar-3 test/ing\"></div>")))
    });
}

criterion_group!(benches, extract_benchmark);
criterion_main!(benches);
