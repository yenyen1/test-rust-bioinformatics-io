use test_parser::utils::{NCount, open_bufreader};

use std::hint::black_box;

use bio::io::fasta as bio_fa;
use criterion::{Criterion, criterion_group, criterion_main};
use noodles_fasta::io as noodle_fa;

const SAMPLE_SIZE: usize = 1;
const LR_PATH: &str = "data/GM24385_1_subset.fasta";
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fasta.gz";
const SR_PATH: &str = "data/D1_S1_L001_R1_001_subset.fasta";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fasta.gz";

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

fn noodles_parse(path: &str) -> Result<(), std::io::Error> {
    let mut reader = open_bufreader(path).map(noodle_fa::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record?;
        let seq = result.sequence();
        seq.as_ref().iter().for_each(|e| nc_count.add(e, 1));
    }
    // nc_count.print();
    Ok(())
}

fn bio_parse(path: &str) -> Result<(), std::io::Error> {
    let reader = open_bufreader(path).map(bio_fa::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let result = record?;
        let seq = result.seq();
        seq.iter().for_each(|e| nc_count.add(e, 1));
    }
    // nc_count.print();
    Ok(())
}

fn fastq_parse(path: &str) -> Result<(), std::io::Error> {
    Ok(())
}
criterion_group!(benches, bench_lrfa_parser, bench_srfa_parser);
criterion_main!(benches);
