# Benchmark FASTA/FASTQ IO crates in Rust

There are many crates on `crates.io` for reading FASTA/FASTQ data. Some crates are more general-purpose and provide a wide range of features, such as async support and indexed FASTA support. Others are more specialized, focusing on specific use cases. For example, some crates accelerate full-file iteration by reducing unnecessary checks and minimizing data copies; others provide multi-threaded processing; and some automatically detect FASTA/FASTQ formats and compressed file types and return the appropriate readers.

Depanding on how the code is written, the runtime can vary due to factors such as the number of memory copies, the frequency of system calls, and the use of memory-mapped or lazy loading techniques. It is not practical to inspect every crate in detail. Instead, I summarize their features and benchmark their performance from a user’s perspective, serving as a reference to help choose the appropriate crate for different use cases.

### FASTA/FASTQ Crates
- **bio**: https://crates.io/crates/bio (FASTA/FASTQ)
- **fastq**: https://crates.io/crates/fastq (FASTQ)
- **seq-io**: https://crates.io/crates/seq_io (FASTA/FASTQ)
- **fxread**: https://crates.io/crates/fxread (FASTA/FASTQ) 
- **needletail**: https://crates.io/crates/needletail (FASTA/FASTQ)
- **kseq**: https://crates.io/crates/kseq (FASTA/FASTQ)
- **noodles**: https://crates.io/crates/noodles (FASTA/FASTQ, SAM/BAM/CRAM, VCF/BCF)

### Reports
- [Dataset Preparation](reports/datasets.md) : Benchmarking was performed using real sequencing data from the GIAB project.
- [Benchmark Environment](reports/environment.md)
- [Summary of Features in FASTA/FASTQ Crates](reports/fastx_benchmarks.md#summary-of-fastx-features)
- [Benchmark Report for FASTA Readers](reports/fastx_benchmarks.md#fasta-reader-report) : comparing their average runtime when iterating through an entire FASTA file.
- [Benchmark Report for FASTQ Readers](reports/fastx_benchmarks.md#fasta-reader-report) : comparing their average runtime when iterating through an entire FASTQ file.

