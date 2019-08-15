#[macro_use]
extern crate criterion;

#[path = "../src/aoc/y2015/task_4.rs"]
mod task_4;

use criterion::Criterion;


fn md5_benchmark(c: &mut Criterion) {
    c.bench_function("md5 10", |b| b.iter(|| task_4::calculate("ckczppom".to_string(), 1, num_cpus::get())));
}

criterion_group!(benches, md5_benchmark);
criterion_main!(benches);