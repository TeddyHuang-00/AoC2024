# Advent of Code 2024 in Rust

This repository contains my solutions for the Advent of Code 2024 in Rust. I'm using this as an opportunity to learn Rust, so the code might not be the most idiomatic Rust code.

## Performance

Ergonomics and readability are more important to me than performance, so I'm not going to spend a lot of time optimizing the code. That being said, I'm running benchmarks for each day's solution for reference. Due to the nature of the Advent of Code puzzles, the input vary from person to person, so the benchmarks are not representative of the performance of the code in general. For the record, I'm running the benchmarks on a PC with an AMD Ryzen 9 7950X3D 16-Core Processor @ 5.7 GHz and 128 GB of RAM.

| Day | Part 1 (micro seconds) | Part 2 (micro seconds) | Method                               |
| --: | ---------------------- | ---------------------- | ------------------------------------ |
|   1 | 130.84 ± 19.30         | 105.85 ± 18.96         | B Tree + counting                    |
|   2 | 95.70 ± 2.82           | 145.03 ± 3.32          | Brute force                          |
|   3 | 151.32 ± 7.61          | 215.90 ± 4.96          | Regular expression                   |
|   4 | 663.44 ± 22.55         | 507.05 ± 12.22         | Matrix stacking & matching           |
|   5 | 931.95 ± 65.25         | 2460.15 ± 69.06        | Topological sort                     |
|   6 | 377.85 ± 26.83         | 81214.27 ± 761.50      | Brute force with parallel            |
|   7 | 1204.90 ± 166.41       | 44482.84 ± 2602.91     | Hash set + enumeration with parallel |
