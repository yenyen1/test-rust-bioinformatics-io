# Benchmark bioinformatics I/O libraries in Rust

**Results:**
- [Benchmarking FASTX I/O Readers (bio, noodles)](reports/fastx_benchmarks.md)
- [Benchmarking BAM I/O Readers (rust-htslib, noodles, bam)](reports/bam_benchmarks.md)

## Bioinformatics I/O libraries in crates.io

Compare the following I/O libraries published on `crates.io`:

- **bio**: https://crates.io/crates/bio (FASTA/FASTQ)
- **fastq**: https://crates.io/crates/fastq (FASTQ)
- **seq-io**: https://crates.io/crates/seq_io (FASTA/FASTQ)
- **fxread**: https://crates.io/crates/fxread (FASTA/FASTQ) 
- **needletail**: https://crates.io/crates/needletail (FASTA/FASTQ)
- **kseq**: https://crates.io/crates/kseq (FASTA/FASTQ)
- **noodles**: https://crates.io/crates/noodles (FASTA/FASTQ, SAM/BAM/CRAM, VCF/BCF)
- **rust-htslib**: https://crates.io/crates/rust-htslib (FASTA, SAM/BAM/CRAM, VCF/BCF)
- **bam**: https://crates.io/crates/bam (BAM)


## Dataset

This repository uses sequencing data from Genome in a Bottle (GIAB) project. AshkenazimTrio son (HG002):
- **GIAB**: [https://ftp.ncbi.nlm.nih.gov/ReferenceSamples/giab/data/AshkenazimTrio/HG002_NA24385_son/](https://ftp.ncbi.nlm.nih.gov/ReferenceSamples/giab/data/AshkenazimTrio/HG002_NA24385_son/)

The alignments use GRCh38: 
- **GRCh38**: [https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/seqs_for_alignment_pipelines.ucsc_ids/GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.gz](https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/seqs_for_alignment_pipelines.ucsc_ids/GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.gz)


