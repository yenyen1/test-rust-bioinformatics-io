#[cfg(test)]

mod tests {
    use test_bioinformatics_io::fastq_parser::{bio_parse};

    const LR_PATH: &str = "tests/data/GM24385_1_subset_100.fastq";
    const LR_PATH_GZ: &str = "tests/data/GM24385_1_subset_100.fastq.gz";
    const SR_PATH: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq";
    const SR_PATH_GZ: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq.gz";
    const LR_COUNT: [u64; 5] = [230450, 142423, 157285, 234587, 0];
    const SR_COUNT: [u64; 5] = [6959, 5384, 5382, 7134, 58];

    #[test]
    fn test_bio_parse() {
        let nc_count = bio_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());

    }
    
}