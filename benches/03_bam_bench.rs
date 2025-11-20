use bam as bam_reader;
use criterion::{Criterion, criterion_group, criterion_main};
use noodles_bam::io as noodles;
use rust_htslib::bam::{self as htslib_reader, Read};
use std::fs::File;
use std::hint::black_box;
use test_parser::utils::NCount;

const LR_BAM: &str = "data/";
const SR_BAM: &str = "data/";
const SINGLE_THREADS: u16 = 1;
const MULTI_THREADS: u16 = 4;

fn bench_lr_bam_io(c: &mut Criterion) {
    let path = LR_BAM;
    let mut group = c.benchmark_group("LR_BAM");
    group.sample_size(5);
    group.bench_function("bam", |b| {
        b.iter(|| bam_io(black_box(path), black_box(SINGLE_THREADS)))
    });
    group.bench_function("bam_mult", |b| {
        b.iter(|| bam_io(black_box(path), black_box(MULTI_THREADS)))
    });
    group.bench_function("htslib", |b| b.iter(|| htslib_io(black_box(path))));
    group.bench_function("noodles", |b| b.iter(|| noodles_io(black_box(path))));
    group.finish();
}

fn bam_io(path: &str, thread: u16) -> Result<(), std::io::Error> {
    let reader = bam_reader::BamReader::from_path(path, thread)?;
    let mut nc_count = NCount::new();
    for record in reader {
        let record = record?;
        record.sequence().raw().iter().for_each(|c| nc_count.add(c));
    }
    nc_count.print();
    Ok(())
}

fn htslib_io(path: &str) -> Result<(), std::io::Error> {
    let mut reader = htslib_reader::Reader::from_path(path).unwrap();
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let record = record.unwrap();
        let seq = record.seq();
        for i in 0..seq.len() {
            nc_count.add(&seq.encoded_base(i));
        }
    }
    nc_count.print();
    Ok(())
}

fn noodles_io(path: &str) -> Result<(), std::io::Error> {
    let mut reader = File::open(path).map(noodles::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let record = record?;
        record
            .sequence()
            .as_ref()
            .iter()
            .for_each(|c| nc_count.add(c));
    }
    nc_count.print();
    Ok(())
}

criterion_group!(benches, bench_lr_bam_io);
criterion_main!(benches);
