use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2023::{day_01, day_02, day_03};

pub fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(disable)]
    c.bench_function("day 1 part 1", |b| {
        //b.iter(|| part1(black_box(include_str!("../src/day_01/input.txt"))))
        b.iter(|| day_01::part1(black_box(include_str!("../src/day_01/input.txt"))))
    });

    #[cfg(disable)]
    c.bench_function("day 3 part 1", |b| {
        //b.iter(|| part1(black_box(include_str!("../src/day_01/input.txt"))))
        b.iter(|| day_03::part1_clean(black_box(include_str!("../src/day_03/input.txt"))))
    });

    c.bench_function("day 3 part 2", |b| {
        //b.iter(|| part1(black_box(include_str!("../src/day_01/input.txt"))))
        b.iter(|| day_03::part2_clean(black_box(include_str!("../src/day_03/input.txt"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
