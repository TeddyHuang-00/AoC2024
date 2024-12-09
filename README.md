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
|   6 | 521.66 ± 42.10         | 76062.14 ± 1238.70     | Brute force with parallel            |
|   7 | 1322.73 ± 99.07        | 27690.07 ± 910.65      | Hash set + enumeration with parallel |
|   8 | 41.86 ± 2.02           | 132.83 ± 4.90          | Lattice points of lines              |
|   9 | 277.48 ± 10.21         | 9026.38 ± 27.95        | Double pointer                       |

## Running the code

### Getting usage information

Make sure you have Rust installed. Use the following command to get the help message:

```sh
cargo solve -h
```

### Testing the code

To run the tests, use the following command:

```sh
cargo test
```

### Solving the puzzles

To run the code for a specific day, inputs are to be put in the `inputs` directory with the name `dayXX.txt`, where `XX` is the day number. The following command runs the code for day 1:

```sh
cargo solve day1
```

Should you want to put the input in a different directory, you can use the `-i` flag to specify the input file directory:

```sh
cargo solve -i /path/to/input/dir/ day1
```

To run the code for all days, use the following command:

```sh
cargo solve all
```

### Benchmarking the code

For benchmarking, add the `-b` flag:

```sh
cargo solve -b day1
```

### Creating a new day

To create a new day, use the following command:

```sh
cargo new-day
```
