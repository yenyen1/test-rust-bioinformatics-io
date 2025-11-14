use test_parser::utils::{NCount, open_bufreader};

use std::hint::black_box;
use std::time::Duration;

use bio::io::fastq as bio_fq;
use criterion::{Criterion, criterion_group, criterion_main};
use noodles_fastq::io as noodle_fq;

const LR_PATH: &str = "data/long_1000.fq";
const LR_PATH_GZ: &str = "data/long_1000.fq.gz";
const SR_PATH: &str = "data/SR_10000.fq";
const SR_PATH_GZ: &str = "data/SR_10000.fq.gz";

fn bench_lrfq_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("LRFQ parser");
    // group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(LR_PATH))));
    group.bench_function("noodles", |b| {
        b.iter(|| noodles_parse(black_box(LR_PATH)))
    });
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(LR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(LR_PATH_GZ)))
    });
    group.finish();
}

fn bench_srfq_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("SRFQ parser");
    // group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    group.bench_function("bio", |b| b.iter(|| bio_parse(black_box(SR_PATH))));
    group.bench_function("noodles", |b| {
        b.iter(|| noodles_parse(black_box(SR_PATH)))
    });
    group.bench_function("bio_gz", |b| b.iter(|| bio_parse(black_box(SR_PATH_GZ))));
    group.bench_function("noodles_gz", |b| {
        b.iter(|| noodles_parse(black_box(SR_PATH_GZ)))
    });
    group.finish();
}

fn bio_parse(path: &str) -> Result<(), std::io::Error> {
    let reader = open_bufreader(path).map(bio_fq::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record.unwrap();
        result.seq().iter().for_each(|c| nc_count.add(c));
    }
    // nc_count.print();
    Ok(())
}

fn noodles_parse(path: &str) -> Result<(), std::io::Error> {
    let mut reader = open_bufreader(path).map(noodle_fq::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let result = record?;
        result.sequence().iter().for_each(|c| nc_count.add(c));
    }
    // nc_count.print();
    Ok(())
}

criterion_group!(benches, bench_lrfq_parser, bench_srfq_parser);
criterion_main!(benches);
