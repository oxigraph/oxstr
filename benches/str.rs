use codspeed_criterion_compat::{BatchSize, Criterion, criterion_group, criterion_main};
use oxstr::OxStr;
use std::hint::black_box;

fn bench_oxstr_new_owned(c: &mut Criterion) {
    c.bench_function("OxStr.new_owned", |b| {
        b.iter(|| {
            black_box(OxStr::new_owned(black_box(
                "I am a quite enough long string, isn't it?",
            )))
        })
    });
}

fn bench_oxstr_owned_to_owned(c: &mut Criterion) {
    c.bench_function("owned OxStr.to_owned", |b| {
        let value = black_box(OxStr::new_owned(
            "I am a quite enough long string, isn't it?",
        ));
        b.iter(|| black_box(black_box(&value).to_owned()))
    });
}

fn bench_oxstr_borrowed_to_owned(c: &mut Criterion) {
    c.bench_function("borrowed OxStr.to_owned", |b| {
        let value = black_box(OxStr::new("I am a quite enough long string, isn't it?"));
        b.iter(|| black_box(black_box(&value).to_owned()))
    });
}

fn bench_oxstr_owned_clone(c: &mut Criterion) {
    c.bench_function("owned OxStr.clone", |b| {
        let value = black_box(OxStr::new_owned(
            "I am a quite enough long string, isn't it?",
        ));
        b.iter(|| black_box(black_box(&value).clone()))
    });
}

fn bench_oxstr_from_string(c: &mut Criterion) {
    c.bench_function("OxStr.from String", |b| {
        b.iter_batched(
            || black_box("I am a quite enough long string, isn't it?".to_string()),
            |str| black_box(OxStr::from(black_box(str))),
            BatchSize::SmallInput,
        )
    });
}

fn bench_oxstr_borrowed_to_string(c: &mut Criterion) {
    c.bench_function("borrowed OxStr.to String", |b| {
        b.iter_batched(
            || black_box(OxStr::new("I am a quite enough long string, isn't it?")),
            |str| black_box(String::from(black_box(str))),
            BatchSize::SmallInput,
        )
    });
}

fn bench_oxstr_owned_to_string(c: &mut Criterion) {
    c.bench_function("owned OxStr.to String", |b| {
        b.iter_batched(
            || {
                black_box(OxStr::new_owned(
                    "I am a quite enough long string, isn't it?",
                ))
            },
            |str| black_box(String::from(black_box(str))),
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
    bench_oxstr_from_string,
    bench_oxstr_borrowed_to_string,
    bench_oxstr_owned_to_string
);

criterion_main!(oxstr);
