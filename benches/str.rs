use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
use oxstr::OxStr;
use std::hint::black_box;

fn bench_oxstr_new_owned(c: &mut Criterion) {
    c.bench_function("OxStr::new_owned", |b| {
        b.iter(|| {
            OxStr::new_owned(black_box("I am a quite enough long string, isn't it?"));
        })
    });
}

fn bench_oxstr_borrowed_to_owned(c: &mut Criterion) {
    c.bench_function("borrowed OxStr::to_owned", |b| {
        let value = OxStr::new("I am a quite enough long string, isn't it?");
        b.iter(|| {
            black_box(&value).to_owned();
        })
    });
}

fn bench_oxstr_owned_clone(c: &mut Criterion) {
    c.bench_function("owned OxStr::clone", |b| {
        let value = OxStr::new_owned("I am a quite enough long string, isn't it?");
        b.iter(|| {
            let _ = black_box(&value).clone();
        })
    });
}

fn bench_oxstr_owned_as_str(c: &mut Criterion) {
    c.bench_function("owned OxStr::a_str", |b| {
        let value = OxStr::new_owned("I am a quite enough long string, isn't it?");
        b.iter(|| {
            black_box(&value).as_str();
        })
    });
}

fn bench_oxstr_borrowed_as_str(c: &mut Criterion) {
    c.bench_function("borrowed OxStr::a_str", |b| {
        let value = OxStr::new("I am a quite enough long string, isn't it?");
        b.iter(|| {
            black_box(&value).as_str();
        })
    });
}

criterion_group!(
    oxstr,
    bench_oxstr_new_owned,
    bench_oxstr_borrowed_to_owned,
    bench_oxstr_owned_clone,
    bench_oxstr_owned_as_str,
    bench_oxstr_borrowed_as_str
);

criterion_main!(oxstr);
