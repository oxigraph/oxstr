use codspeed_criterion_compat::{BatchSize, Criterion, criterion_group, criterion_main};
use oxstr::{OxStr, OxStrBuilder};
use std::hint::black_box;

fn bench_oxstr_new_owned(c: &mut Criterion) {
    c.bench_function("OxStr.new_owned", |b| {
        b.iter(|| {
            OxStr::new_owned(black_box("I am a quite enough long string, isn't it?"));
        })
    });
}

fn bench_oxstr_owned_to_owned(c: &mut Criterion) {
    c.bench_function("owned OxStr.to_owned", |b| {
        let value = black_box(OxStr::new_owned(
            "I am a quite enough long string, isn't it?",
        ));
        b.iter(|| {
            black_box(&value).to_owned();
        })
    });
}

fn bench_oxstr_borrowed_to_owned(c: &mut Criterion) {
    c.bench_function("borrowed OxStr.to_owned", |b| {
        let value = black_box(OxStr::new("I am a quite enough long string, isn't it?"));
        b.iter(|| {
            black_box(&value).to_owned();
        })
    });
}

fn bench_oxstr_owned_clone(c: &mut Criterion) {
    c.bench_function("owned OxStr.clone", |b| {
        let value = black_box(OxStr::new_owned(
            "I am a quite enough long string, isn't it?",
        ));
        b.iter(|| {
            let _ = black_box(&value).clone();
        })
    });
}

fn bench_oxstr_builder_push_str_without_capacity(c: &mut Criterion) {
    c.bench_function("OxStrBuilder.push_str without capacity", |b| {
        b.iter(|| {
            let mut builder = OxStrBuilder::new();
            for part in black_box([
                "https://",
                "example.com",
                "/users/",
                "alice",
                "/repositories/",
                "oxstr",
                "?page=",
                "42",
            ]) {
                builder.push_str(part);
            }
            black_box(OxStr::from(builder));
        })
    });
}

fn bench_oxstr_builder_push_str_with_capacity(c: &mut Criterion) {
    c.bench_function("OxStrBuilder.push_str without capacity", |b| {
        b.iter(|| {
            let mut builder = OxStrBuilder::with_capacity(58);
            for part in black_box([
                "https://",
                "example.com",
                "/users/",
                "alice",
                "/repositories/",
                "oxstr",
                "?page=",
                "42",
            ]) {
                builder.push_str(part);
            }
            black_box(OxStr::from(builder));
        })
    });
}

fn bench_oxstr_builder_push_chars(c: &mut Criterion) {
    c.bench_function("OxStrBuilder.push_chars", |b| {
        b.iter(|| {
            let mut builder = OxStrBuilder::new();
            for ch in black_box("OxStrBuilder says hello! é水🦀").chars() {
                builder.push(ch);
            }
            black_box(OxStr::from(builder));
        })
    });
}

fn bench_oxstr_builder_into_oxstr_exact_capacity(c: &mut Criterion) {
    c.bench_function("OxStrBuilder.into_OxStr with exact capacity", |b| {
        b.iter_batched(
            || {
                let mut builder = OxStrBuilder::with_capacity(black_box(58));
                builder.push_str(black_box(
                    "https://example.com/users/alice/repositories/oxstr?page=42",
                ));
                builder
            },
            |builder| black_box(OxStr::from(builder)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_oxstr_builder_into_oxstr_spear_capacity(c: &mut Criterion) {
    c.bench_function("OxStrBuilder.into_OxStr with spare_capacity", |b| {
        b.iter_batched(
            || {
                let mut builder = OxStrBuilder::with_capacity(black_box(60));
                builder.push_str(black_box(
                    "https://example.com/users/alice/repositories/oxstr?page=42",
                ));
                builder
            },
            |builder| OxStr::from(black_box(builder)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    oxstr,
    bench_oxstr_new_owned,
    bench_oxstr_owned_to_owned,
    bench_oxstr_borrowed_to_owned,
    bench_oxstr_owned_clone,
);

criterion_group!(
    oxst_builder,
    bench_oxstr_builder_push_str_without_capacity,
    bench_oxstr_builder_push_str_with_capacity,
    bench_oxstr_builder_push_chars,
    bench_oxstr_builder_into_oxstr_exact_capacity,
    bench_oxstr_builder_into_oxstr_spear_capacity
);

criterion_main!(oxstr, oxst_builder);
