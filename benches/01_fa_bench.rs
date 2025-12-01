use test_bioinformatics_io::fasta_parser::{bio_parse, noodles_parse};
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

const SAMPLE_SIZE: usize = 10;
const MULTILINE_PATH: &str = "test_multiline.fasta";
const LR_PATH: &str = "data/GM24385_1_subset.fasta";
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fasta.gz";
const SR_PATH: &str = "data/D1_S1_L001_R1_001_subset.fasta";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fasta.gz";

fn bench_test_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("test");
    group.sample_size(SAMPLE_SIZE);

    // group.bench_function("bio multiline", |b| b.iter(|| bio_parse(black_box(MULTILINE_PATH))));
    group.bench_function("noodle multiline", |b| b.iter(|| noodles_parse(black_box(MULTILINE_PATH))));
    group.finish();
}

fn bench_lrfa_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("LRFA parser");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(LR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(LR_PATH))));
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(LR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(LR_PATH_GZ)))
    });
    group.finish();
}

fn bench_srfa_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("SRFA parser");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(SR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(SR_PATH))));
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(SR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(SR_PATH_GZ)))
    });
    group.finish();
}

criterion_group!(benches, bench_test_parser);
criterion_main!(benches);
