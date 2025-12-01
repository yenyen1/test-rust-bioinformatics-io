use test_bioinformatics_io::fastq_parser::{bio_parse, bio_test_file_parse, fastq_parallel_parse, fastq_parse, noodles_parse};

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SAMPLE_SIZE: usize = 10;
const LR_PATH: &str = "data/GM24385_1_subset.fastq";
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fastq.gz";
const SR_PATH: &str = "data/D1_S1_L001_R1_001_subset.fastq";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fastq.gz";

fn bench_lrfq_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("LRFQ parser");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(LR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(LR_PATH))));
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(LR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(LR_PATH_GZ)))
    });
    // fastq_parse was failed to read LR data since read is too long


    group.finish();
}

fn bench_srfq_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("SRFQ parser");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(SR_PATH))));
    group.bench_function("noodles", |b| b.iter(|| noodles_parse(black_box(SR_PATH))));
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(SR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("fastq", |b| b.iter(|| fastq_parse(black_box(SR_PATH))));
    group.bench_function("fastq_gz", |b| {
        b.iter(|| fastq_parse(black_box(SR_PATH_GZ)))
    });
    group.finish();
}

/// To check automatical compression detection function
/// fastq::parse_path is more stable than and better performance than detect file type with file path (utils::open_bufreader).
fn bench_auto_reader(c: &mut Criterion) {
    let mut group = c.benchmark_group("auto reader");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("bio", |b| b.iter(|| bio_test_file_parse(black_box(SR_PATH))));
    group.bench_function("bio_gz", |b| b.iter(|| bio_test_file_parse(black_box(SR_PATH_GZ))));

    group.finish();
}

/// The performance improved by adding threads for Uncompressed FASTQ.
/// However, for compressed FASTQ, the performance of multi-threads was worser than single threads.
fn bench_srfq_fastq_nthread(c: &mut Criterion) {
    let mut group = c.benchmark_group("SRFQ parser");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("fastq", |b| b.iter(|| fastq_parse(black_box(SR_PATH))));
    group.bench_function("fastq_gz", |b| {
        b.iter(|| fastq_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("fastq t=2", |b| {
        b.iter(|| fastq_parallel_parse(black_box(SR_PATH), black_box(2)))
    });
    group.bench_function("fastq_gz t=2", |b| {
        b.iter(|| fastq_parallel_parse(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("fastq t=4", |b| {
        b.iter(|| fastq_parallel_parse(black_box(SR_PATH), black_box(4)))
    });
    group.bench_function("fastq_gz t=4", |b| {
        b.iter(|| fastq_parallel_parse(black_box(SR_PATH_GZ), black_box(4)))
    });
    group.finish();
}

criterion_group!(benches, bench_auto_reader);
criterion_main!(benches);
