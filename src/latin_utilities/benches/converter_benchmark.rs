use criterion::{black_box, criterion_group, criterion_main, Criterion};
use latin_utilities::StandardLatinConverter;

fn null_conversion_benchmark(c: &mut Criterion) {
    c.bench_function("null", |b| {
        let s = "dura lex sed lex";
        let conver = StandardLatinConverter::default();
        b.iter(|| black_box(conver.convert(s)))
    });

    c.bench_function("empty", |b| {
        let s = "";
        let conver = StandardLatinConverter::default();
        b.iter(|| black_box(conver.convert(s)))
    });
}

fn conversion_benchmarks(c: &mut Criterion) {
    c.bench_function("uv-replacement", |b| {
        let s = "ivljvs caesar";
        let c = StandardLatinConverter::default();
        b.iter(|| black_box(c.convert(s)))
    });
}

criterion_group!(benches, null_conversion_benchmark, conversion_benchmarks);
criterion_main!(benches);
