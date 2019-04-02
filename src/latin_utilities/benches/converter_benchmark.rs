use criterion::{black_box, criterion_group, criterion_main, Criterion};
use latin_utilities::StandardLatinConverter;
use std::iter;

static KB: usize = 1024;

fn null_conversion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "no conversions",
        |b, &size| {
            let s: String = iter::repeat("e").take(size).collect();
            let conver = StandardLatinConverter::default();
            b.iter(|| black_box(conver.convert(&s)))
        },
        vec![KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB],
    );

    c.bench_function("empty", |b| {
        let s = "";
        let conver = StandardLatinConverter::default();
        b.iter(|| black_box(conver.convert(s)))
    });
}

fn conversion_benchmarks(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "uv-replacement",
        |b, &size| {
            let s: String = iter::repeat("v").take(size).collect();
            let conver = StandardLatinConverter::default();
            b.iter(|| black_box(conver.convert(&s)))
        },
        vec![KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB],
    );
}

criterion_group!(benches, null_conversion_benchmark, conversion_benchmarks);
criterion_main!(benches);
