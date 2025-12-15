#[cfg(test)]

mod tests {

    use test_bioinformatics_io::fasta_parser::{
        bio_parse, noodles_parse, seq_io_parallel_parse, seq_io_parse,
    };
    use test_bioinformatics_io::fastq_parser::{fxread_parse, kseq_parse, needletail_parse};

    const LR_PATH: &str = "tests/data/GM24385_1_subset_100.fasta";
    const LR_PATH_GZ: &str = "tests/data/GM24385_1_subset_100.fasta.gz";
    const SR_PATH: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fasta";
    const SR_PATH_GZ: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fasta.gz";
    const MULTILINE_PATH: &str = "tests/data/multiline.fasta";
    const LR_COUNT: [u64; 5] = [230450, 142423, 157285, 234587, 0];
    const SR_COUNT: [u64; 5] = [6959, 5384, 5382, 7134, 58];
    const ML_COUNT: [u64; 5] = [5, 5, 5, 5, 5];

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

    #[test]
    fn test_noodles_parse() {
        let nc_count = noodles_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_seq_io_parse() {
        let nc_count = seq_io_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());

        let nc_count = seq_io_parallel_parse(LR_PATH, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(LR_PATH_GZ, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH_GZ, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fxread_parse() {
        let nc_count = fxread_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_needetail_parse() {
        let nc_count = needletail_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_kseq_parse() {
        let nc_count = kseq_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fasta_multiline_parse() {
        let nc_count = bio_parse(MULTILINE_PATH);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(MULTILINE_PATH);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(MULTILINE_PATH);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(MULTILINE_PATH, 4);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        // let nc_count = fxread_parse(MULTILINE_PATH);
        // assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(MULTILINE_PATH);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(MULTILINE_PATH);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
    }
}
