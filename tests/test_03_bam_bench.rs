#[cfg(test)]

mod tests {
    use test_bioinformatics_io::bam_parser::{bam_parse, htslib_parse, noodles_parse};

    const LR_PATH: &str = "tests/data/HG002_GRCh38_ONT-UL_UCSC_20200508.phased.test.bam";
    const SR_PATH: &str = "tests/data/HG002.GRCh38.2x250.test.bam";
    const LR_COUNT: [u64; 5] = [2924571, 1901837, 1990958, 2919782, 0];
    const SR_COUNT: [u64; 5] = [6529935, 4461329, 4457240, 6522471, 5298];

    #[test]
    fn test_bam_parse() {
        let nc_count = bam_parse(LR_PATH, 1);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = bam_parse(SR_PATH, 1);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_htslib_parse() {
        let nc_count = htslib_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = htslib_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_noodles_parse() {
        let nc_count = noodles_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

}
