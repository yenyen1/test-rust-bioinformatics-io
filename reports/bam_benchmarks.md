# BAM I/O benchmark report

This report summarizes the performance benchmarks for the project using `Criterion.rs`.
Benchmark the performance of reading a BAM file consecutively.

## BAM I/O libraries

| Crate |   | Version | Indexed Reader | RecordBuffer | Pileup | Multi-Thread Modes | Async |
|:-----:|:-:|:-------:|:--------------:|:------------:|:------:|:-------------------:|:-----:|
| rust-htslib::bam | HTSlib bindings | v0.51.0 | V | V | V |  |  |
| noodles-bam | Pure Rust | v0.84.0 | V | V |  |  | V |
| bam | Pure Rust | v0.1.4 | V |  | V | V |  |


## Datasets
### 1. Download AshkenazimTrio son (HG002) sequencing data from GIAB 

| Read | Dataset | File Name | File Size | 
|:----:|:-------:|:---------:|:---------:|
|  Short-read (SR)  |  NIST_Illumina_2x250bps  | HG002.GRCh38.2x250.bam | 122G |
|  Long-read (LR)  |  UCSC_Ultralong_OxfordNanopore_Promethion  | HG002_GRCh38_ONT-UL_UCSC_20200508.phased.bam | 175G | 

- **GRCh38 reference**: [GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.gz](https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/seqs_for_alignment_pipelines.ucsc_ids/GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.gz)


### 2. Sample subset from the original BAM files

- Because the dataset is too large to iterate, I extracted a subset for benchmarking.
    ```
    # Sampling SR dataset with sample fraction 0.01
    samtools view --subsample 0.01 --subsample-seed 11 -b ${BAM} > ${SUBSET_BAM}

    # Indexing BAM
    samtools index ${SUBSET_BAM}

    # Calculate BAM statistics
    samtools stats ${SUBSET_BAM} | grep ^SN | cut -f 2-

    ```
- The stats of subsets:
    | Read | Subset Name | File Size | #Sequences | #Reads mapped | #Reads unmapped | #Reads duplicated | #Reads MQ0 |
    |:----:|:-----------:|:---------:|:----------:|:-------------:|:---------------:|:-----------------:|:----------:|
    | SR | HG002.GRCh38.2x250.bam | 1.6G | 8,835,692 | 8,240,838 | 594,854 | 0 | 642 |


## Environment

|  | Env1 | Env2 |
|------|-------|-------|
| CPU Model | Apple M1 Pro | AMD Ryzen 5 5560U with Radeon Graphics |
| CPU Cores/Tgreads | 8/8 (6p+2e) | 6/2 |
| CPU Architecture | ARM64 | x86_64 |
| Memory | 16 GB (LPDDR5) | 8G + 16 GB (DDR4) 3200 MT/s |
| Storage | 494.38 GB (APPLE SSD AP0512R) | 512 GB (INTEL SSDPEKNU512GZ) NVMe PCIe |
| OS | macOS Sequoia 15.6.1  | Ubuntu 22.04.5 |
| Rust Version  | 1.89.0  | 1.91.1 |
| Criterion Version  | 0.7.0  | 0.7.0 |


## Results


## Conclusion