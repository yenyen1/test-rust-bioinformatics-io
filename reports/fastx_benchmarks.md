## Summary of FASTX features

When processing FASTA/FASTQ files, I usually need to iterate through the entire file. For each record, I read its contents and perform various computations or modifications. For example, I may read the sequence of each record to count the occurrences of each nucleotide; extract subsequences that meet specific criteria; decide whether to filter out a record based on the sequence; or modify individual bases in the sequence according to their corresponding quality scores.

| Crate | Implementation | Version | FASTA | FASTQ | Faidx | FASTX | Async | Parallel | Compression | SR | LR | multi-line FASTA |
|:-----:|:-:|:-------:|:-----:|:-----:|:-----:|:-----:|:-----:|:--------:|:--------------------------:|:--:|:--:|:---------------:|
| rust-bio      | Pure rust                  | v3.0.0  | V | V | V |   |   |   |                      | V | V | V |
| noodles-fasta | Pure rust                  | v0.57.0 | V |   | V |   | V |   |                      | V | V | V |
| noodles-fastq | Pure rust                  | v0.21.0 |   | V | V |   | V |   |                      | V | V |   |
| fastq         | C bindings (lz4)           | v0.6.0  |   | V |   |   |   | V | gz, lz4              | V |   |   |
| seq_io        | Pure rust                  | v0.3.4  | V | V |   |   |   | V |                      | V | V | V |
| fxread        | C bindings (bzip2/xz/zstd) | v0.2.14 | V | V |   | V |   |   | bzip2, gz, xz2, zstd | V | V |   | 
| needletail    | C bindings (bzip2/xz/zstd) | v0.6.3  | V | V |   | V |   |   | bzip2, gz, xz2, zstd | V | V | V |
| kseq          | Pure rust                  | v0.5.3  | V | V |   | V |   |   | gz, fofn             | V | V | V |
------
<br>

- **Implementation** : Indicates how the library is implemented, based on the dependencies in its `Cargo.toml`. 
- **FASTA** / **FASTQ** : Providesreader and writer for  FASTA / FASTQ files.  
- **Faidx** : Provides an indexed FASTA reader.
- **FASTX** / **Compression** : Automatically detects FASTA or FASTQ format / compressed file types.
- **Parallel** : Provides funtionality to read records using multiple threads.
- **Async** / **SR** / **LR** / **multi-line FASTA**: Supports asynchronous operations / short-read data / long-read data / multi-line FASTA file.
<br><br><br>


## FASTA Reader Report
 
In this report, I use a simple example to compare the runtime: iterate through the entire FASTA file and counting how many times each nucleotide (A, C, T, G, N) appears. 

This report summarizes the average runtime measured with `Criterion.rs` (sample size = 30). Column names follow the format: `{ReadType}_{CompressionType}_{Environment}`.
<br><br> 

| library | LR_FA_E1 | LR_GZ_E1 | SR_FA_E1 | SR_GZ_E1 | LR_FA_E2 | LR_GZ_E2 | SR_FA_E2 | SR_GZ_E2 | 
|:-------:|:--------:|:--------:|:--------:|:--------:|:--------:|:--------:|:-------:|:--------:|
| bio:io     | 2.20s | 4.58s | 4.14s | 7.60s |  |  |  |  |
| noodles    | 2.26s | 4.62s | 5.70s | 9.18s |  |  |  |  |
| seq_io     | <span style="color:green">1.98s</span> | <span style="color:green">3.84s</span> | <span style="color:green">2.58s</span> | <span style="color:green">5.36s</span> |  |  |  |  |
| fxread     | 2.16s | 4.50s | 3.70s | 7.07s |  |  |  |  |
| needletail | <span style="color:green">2.00s</span> | <span style="color:green">3.90s</span> | <span style="color:green">2.61s</span> | <span style="color:green">5.45s</span> |  |  |  |  |
| kseq       | <span style="color:green">2.02s</span> | <span style="color:green">3.92s</span> | 2.96s | 5.69s |  |  |  |  |
<br>

- [FASTA Violin Plots](fastx_plots.md#fasta-violin-plots)

<br>

## FASTQ Reader Report

In this report, I use a simple example to compare the runtime: iterate through the entire FASTQ file and counting how many times each nucleotide (A, C, T, G, N) appears. 

This report summarizes the average runtime measured with `Criterion.rs` (sample size = 30). Column names follow the format: `{ReadType}_{CompressionType}_{Environment}`.
<br><br>   

| library | LR_FQ_E1 | LR_GZ_E1 | SR_FQ_E1 | SR_GZ_E1 | LR_FQ_E2 | LR_GZ_E2 | SR_FQ_E2 | SR_GZ_E2 | 
|:-------:|:--------:|:--------:|:--------:|:--------:|:--------:|:--------:|:-------:|:--------:|
| bio:io     | 2.57s | 9.46s | 4.95s | 12.44s |  |  |  |  |
| noodles    | 2.50s | 9.35s | 4.59s | 12.05s |  |  |  |  |
| seq_io     | 2.09s | 8.24s | 2.79s |  8.93s |  |  |  |  |
| fxread     | 2.41s | 9.30s | 4.07s | 11.53s |  |  |  |  |
| needletail | 2.09s | 8.25s | 2.85s |  8.98s |  |  |  |  |
| kseq       | 2.15s | 8.34s | 3.36s |  9.33s |  |  |  |  |
| fastq      |   -   |   -   | 2.82s |  6.15s |  |  |  |  |
<br>

- [FASTQ Violin Plots](fastx_plots.md#fastq-violin-plots)
<br><br>

 [Back to ReadMe](../readme.md)