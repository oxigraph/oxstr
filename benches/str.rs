use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
use oxstr::OxStr;
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

criterion_group!(
    oxstr,
    bench_oxstr_new_owned,
    bench_oxstr_owned_to_owned,
    bench_oxstr_borrowed_to_owned,
    bench_oxstr_owned_clone,
);

criterion_main!(oxstr);
