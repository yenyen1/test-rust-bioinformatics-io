use crate::utils::NCount;

use std::fs::File;
use std::io;
use bam::BamReader as bam_reader;
use noodles_bam::io::Reader as noodles_reader;
use rust_htslib::bam::{self as htslib_reader, Read};

pub fn bam_parse(path: &str, thread: u16) -> io::Result<NCount> {
    let reader = bam_reader::from_path(path, thread)?;
    let mut nc_count = NCount::new();
    for record in reader {
        let record = record?;
        record
            .sequence()
            .to_vec()
            .iter()
            .for_each(|c| nc_count.add(c, 1));
    }
    Ok(nc_count)
}

pub fn htslib_parse(path: &str) -> io::Result<NCount> {
    let mut reader = htslib_reader::Reader::from_path(path).unwrap();
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let record = record.unwrap();
        record.seq().as_bytes().iter().for_each(|c| nc_count.add(c, 1));
    }
    Ok(nc_count)
}

pub fn noodles_parse(path: &str) -> io::Result<NCount> {
    let mut reader = File::open(path).map(noodles_reader::new)?;
    let mut nc_count = NCount::new();
    let _header = reader.read_header()?;
    for record in reader.records() {
        let record = record?;
        record
            .sequence()
            .iter()
            .for_each(|c| nc_count.add(&c, 1));
    }
    Ok(nc_count)
}
