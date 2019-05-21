use criterion::{black_box, criterion_group, criterion_main, Criterion};
use interner::{impl_arena_id, InternerBuilder, RawId};

const KB: usize = 2048;

#[derive(Debug, Hash, Eq, Clone, Copy, PartialEq)]
struct TestId(RawId);
impl_arena_id!(TestId);

fn into_id(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "into_id",
        |b, &size| {
            let size = size as u32;
            let interner = InternerBuilder::new()
                .add_all((0..size).map(|i| (TestId(RawId(i)), RawId(i))))
                .build();
            b.iter(|| black_box(interner.to_id(&RawId(1))))
        },
        vec![KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB],
    );
}

fn from_id(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "from_id",
        |b, &size| {
            let size = size as u32;
            let interner = InternerBuilder::new()
                .add_all((0..size).map(|i| (TestId(RawId(i)), RawId(i))))
                .build();

            b.iter(|| black_box(interner.fetch(TestId(RawId(1)))))
        },
        vec![KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB],
    );
}

criterion_group!(benches, into_id, from_id);
criterion_main!(benches);
