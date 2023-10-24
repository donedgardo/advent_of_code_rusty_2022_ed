use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_rusty_2022_ed::{find_answer_1, parse_filesystem};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("src/bin/07/input.txt").unwrap();
    c.bench_function("parse_filesystem", |b| {
        b.iter(|| {
            let cli = parse_filesystem(black_box(input.as_str())).unwrap();
            cli.dir_size("/");
        });

    });
    c.bench_function("find_answer_1", |b| {
        b.iter(|| {
            find_answer_1(black_box(input.as_str()));
        });
    });

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
