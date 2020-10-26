use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let keywords = &["budi", "~/proyek", "/home/budi/proyek"];
    let target = include_str!("./assets/rust_compilation_error.txt");
    c.bench_function("redact", |b| {
        b.iter(|| redact::redact(black_box(keywords), black_box(target)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
