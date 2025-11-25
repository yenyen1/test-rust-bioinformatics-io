use crate::utils::{NCount, merge_nc_count, open_bufreader};

use bio::io::fastq as bio_fq;
use fastq::{self, Record};
use noodles_fastq::io as noodle_fq;

pub fn bio_parse(path: &str) -> Result<NCount, std::io::Error> {
    let reader = open_bufreader(path).map(bio_fq::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record.unwrap();
        result.seq().iter().for_each(|c| nc_count.add(c, 1));
    }
    // nc_count.print();
    Ok(nc_count)
}

pub fn noodles_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut reader = open_bufreader(path).map(noodle_fq::Reader::new)?;
    let mut nc_count = NCount::new();
    reader.records().for_each(|record| {
        let result = record.unwrap();
        result.sequence().iter().for_each(|c| nc_count.add(c, 1));
    });
    // nc_count.print();
    Ok(nc_count)
}

/// Fastq parser failed to read Nanopore dataset since read is too long.
pub fn fastq_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut nc_count = NCount::new();
    let reader = open_bufreader(path).map(fastq::Parser::new)?;
    reader.each(|record| {
        record.seq().iter().for_each(|c| nc_count.add(c, 1));
        true
    })?;
    // nc_count.print();
    Ok(nc_count)
}

pub fn fastq_parallel_parse(path: &str, n_threads: usize) -> Result<NCount, std::io::Error> {
    let reader = open_bufreader(path).map(fastq::Parser::new)?;
    let mut results: Vec<NCount> = reader
        .parallel_each(n_threads, |record_sets| {
            let mut nc_count = NCount::new();
            for records in record_sets {
                records.iter().for_each(|record| {
                    record.seq().iter().for_each(|c| nc_count.add(c, 1));
                });
            }
            nc_count
        })
        .expect("fastq parallel_each failed...");
    let merged = merge_nc_count(&mut results).expect("fastq parallel_each merge result failed");
    // merged.print();
    Ok(merged)
}
