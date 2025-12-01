use crate::utils::{NCount, merge_nc_count, open_bufreader};

use bio::io::fastq as bio_fq;
use fastq::{self, Record};
use noodles_fastq::io as noodle_fq;

pub fn bio_test_file_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut nc_count = NCount::new();
    let reader = bio_fq::Reader::from_file(path).expect("bio fastq parser failed");
    for record in reader.records() {
        let record = record.unwrap();
        record.seq().iter().for_each(|c| nc_count.add(c, 1));
    }
    Ok(nc_count)
}

pub fn bio_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut nc_count = NCount::new();
    let reader = open_bufreader(path).map(bio_fq::Reader::new)?;
    
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

/// Fastq parser failed to read Nanopore dataset since read is too long. (buffersize not enough)
/// It does not validate that the sequence or the quality contain only allowed characters.
/// multithreaded setup not support paired-end reads.
/// 
/// fastq::parse_path can detect gzip, lz4 or plain FASTQ files.
/// It is more stable than and better performance than detect file type with file path (utils::open_bufreader).
pub fn fastq_parse(path: &str) -> Result<NCount, std::io::Error> {
    let mut nc_count = NCount::new();
    fastq::parse_path(Some(path), | reader| {
        reader.each(|record| {
            record.seq().iter().for_each(|c| nc_count.add(c, 1));
            true
        }).expect("fastq parse path");
    })?;
    // nc_count.print();
    Ok(nc_count)
}


pub fn fastq_parallel_parse(path: &str, n_threads: usize) -> Result<NCount, std::io::Error> {
    fastq::parse_path(Some(path), | reader| {
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
        merged
        })
}


