use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use test_bioinformatics_io::fasta_parser::{
    bio_parse, noodles_parse, seq_io_parallel_parse, seq_io_parse,
};
use test_bioinformatics_io::fastq_parser::{fxread_parse, kseq_parse, needletail_parse};

const SAMPLE_SIZE: usize = 30;
const LR_PATH: &str = "data/GM24385_1_subset.fasta";
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fasta.gz";
const SR_PATH: &str = "data/D1_S1_L001_R1_001_subset.fasta";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fasta.gz";

fn bench_lrfa_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("LrFa");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(LR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(LR_PATH))));
    group.bench_function("seq_io", |b| b.iter(|| seq_io_parse(black_box(LR_PATH))));
    group.bench_function("fxread", |b| b.iter(|| fxread_parse(black_box(LR_PATH))));
    group.bench_function("needletail", |b| {
        b.iter(|| needletail_parse(black_box(LR_PATH)))
    });
    group.bench_function("kseq", |b| b.iter(|| kseq_parse(black_box(LR_PATH))));
    group.finish();
}

fn bench_lrfa_gz_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("LrFaGz");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(LR_PATH_GZ))));
    group.bench_function("noodles", |b| {
        b.iter(|| noodles_parse(black_box(LR_PATH_GZ)))
    });
    group.bench_function("seq_io", |b| b.iter(|| seq_io_parse(black_box(LR_PATH_GZ))));
    group.bench_function("fxread", |b| b.iter(|| fxread_parse(black_box(LR_PATH_GZ))));
    group.bench_function("needletail", |b| {
        b.iter(|| needletail_parse(black_box(LR_PATH_GZ)))
    });
    group.bench_function("kseq", |b| b.iter(|| kseq_parse(black_box(LR_PATH_GZ))));
    group.finish();
}

fn bench_srfa_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("SrFa");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(SR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(SR_PATH))));
    group.bench_function("seq_io", |b| b.iter(|| seq_io_parse(black_box(SR_PATH))));
    group.bench_function("fxread", |b| b.iter(|| fxread_parse(black_box(SR_PATH))));
    group.bench_function("needletail", |b| {
        b.iter(|| needletail_parse(black_box(SR_PATH)))
    });
    group.bench_function("kseq", |b| b.iter(|| kseq_parse(black_box(SR_PATH))));
    group.finish();
}

fn bench_srfa_gz_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("SrFaGz");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(SR_PATH_GZ))));
    group.bench_function("noodles", |b| {
        b.iter(|| noodles_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("seq_io", |b| b.iter(|| seq_io_parse(black_box(SR_PATH_GZ))));
    group.bench_function("fxread", |b| b.iter(|| fxread_parse(black_box(SR_PATH_GZ))));
    group.bench_function("needletail", |b| {
        b.iter(|| needletail_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("kseq", |b| b.iter(|| kseq_parse(black_box(SR_PATH_GZ))));
    group.finish();
}

fn bench_gz_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("LrFaGz");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("lr_seq_io", |b| {
        b.iter(|| seq_io_parse(black_box(LR_PATH_GZ)))
    });
    group.bench_function("lr_seq_io_t2", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(LR_PATH_GZ), black_box(2)))
    });
    group.bench_function("lr_seq_io_t4", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(LR_PATH_GZ), black_box(4)))
    });
    group.bench_function("lr_seq_io_t6", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(LR_PATH_GZ), black_box(6)))
    });
    group.bench_function("sr_seq_io", |b| {
        b.iter(|| seq_io_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("sr_seq_io_t2", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("sr_seq_io_t4", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(SR_PATH_GZ), black_box(4)))
    });
    group.bench_function("sr_seq_io_t6", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(SR_PATH_GZ), black_box(6)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_lrfa_parser,
    bench_lrfa_gz_parser,
    bench_srfa_parser,
    bench_srfa_gz_parser,
);
criterion_main!(benches);
