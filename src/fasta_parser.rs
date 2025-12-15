use crate::utils::{NCount, open_bufreader};

use bio::io::fasta as bio_fa;
use noodles_fasta::io as noodle_fa;
use seq_io::fasta as seq_io_fa;
use seq_io::parallel as seq_io_parallel;
use std::io;

pub fn noodles_parse(path: &str) -> io::Result<NCount> {
    let mut reader = open_bufreader(path).map(noodle_fa::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record?;
        let seq = result.sequence();
        seq.as_ref().iter().for_each(|e| nc_count.add(e, 1));
    }
    Ok(nc_count)
}

pub fn bio_parse(path: &str) -> io::Result<NCount> {
    let reader = open_bufreader(path).map(bio_fa::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let result = record?;
        let seq = result.seq();
        seq.iter().for_each(|e| nc_count.add(e, 1));
    }
    Ok(nc_count)
}

pub fn seq_io_parse(path: &str) -> io::Result<NCount> {
    let mut nc_count = NCount::new();
    let mut reader = open_bufreader(path).map(seq_io_fa::Reader::new)?;
    while let Some(record) = reader.next() {
        let record = record.expect("Seq_io Error reading record");
        record
            .seq_lines()
            .for_each(|s| s.iter().for_each(|c| nc_count.add(c, 1)));
    }
    Ok(nc_count)
}

pub fn seq_io_parallel_parse(path: &str, n_threads: u32) -> io::Result<NCount> {
    let reader = open_bufreader(path).map(seq_io_fa::Reader::new)?;
    let out = seq_io_parallel::read_parallel(
        reader,
        n_threads,
        2,
        |record_set| {
            let mut nc_count = NCount::new();
            for record in record_set.into_iter() {
                record.full_seq().iter().for_each(|c| nc_count.add(c, 1));
            }
            nc_count
        },
        |result_sets| {
            let mut merged = NCount::new();
            while let Some(result) = result_sets.next() {
                let (_, nc_count) = result.unwrap();
                merged.merge(&nc_count);
            }
            merged
        },
    );
    Ok(out)
}
