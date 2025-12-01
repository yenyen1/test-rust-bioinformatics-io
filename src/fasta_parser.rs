use crate::utils::{open_bufreader, NCount};

use bio::io::fasta as bio_fa;
use noodles_fasta::io as noodle_fa;
use seq_io::fasta::Record;
use seq_io::fasta as seqio_fa;
use seq_io::parallel as seqio_parallel;


pub fn noodles_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut reader = open_bufreader(path).map(noodle_fa::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record?;
        let seq = result.sequence();
        seq.as_ref().iter().for_each(|e| nc_count.add(e, 1));
    }
    nc_count.print();
    Ok(nc_count)
}

pub fn bio_parse(path: &str) -> Result<NCount, std::io::Error> {
    let reader = open_bufreader(path).map(bio_fa::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let result = record?;
        let seq = result.seq();
        seq.iter().for_each(|e| nc_count.add(e, 1));
    }
    nc_count.print();
    Ok(nc_count)
}

pub fn seqio_parse(path: &str) -> Result<NCount, std::io::Error> {
    let nc_count = NCount::new();
    Ok(nc_count)
}

pub fn seqio_parallel_parse(path: &str) -> Result<NCount, std::io::Error> {
    let reader = seqio_fa::Reader::from_path(path).unwrap();
    let mut out = seqio_parallel::read_parallel(reader, 4, 2, |record_set| {
        let mut nc_count = NCount::new();
        for record in record_set.into_iter() {
            let seq = record.seq().iter().for_each(|c| nc_count.add(c, 1));
        }
        nc_count
    }, |result_sets| {
        let mut merged = NCount::new();
        while let Some(result) = result_sets.next() {
            let (_, nc_count) = result.unwrap();
            merged.merge(&nc_count);
        }
        merged
    });

    out.print();
    Ok(out)
}