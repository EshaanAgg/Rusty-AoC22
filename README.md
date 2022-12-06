# Advent of Code '22

This repository contains my solutions for AoC'22 problems, written in the language that will save us from `cpp`.

## Running the code

The project organisation uses `cargo` as it's package manager.

The `main` script of the module benchmarks and executes the imported functions.

To view the same for any day, just import the module `day<number>` in `main.rs`, and then call the `bench` function on the `part1` or `part2` function of that module.

Run the script by:

```
cargo build && cargo run
```

## Benchmarks

The execution times of all the scripts on my `Intel I7` machine are listed as follows:

| Day | Time Part A | Time Part B |
| --- | ----------- | ----------- |
| 1   | `251.599us` | `286.452us` |
| 2   | `1.22323ms` | `1.01850ms` |
| 3   | `3.27246ms` | `3.93728ms` |
| 4   | `1.30294ms` | `1.24063ms` |
| 5   | `3.00914ms` | `3.14152ms` |
| 6   | `2.31388ms` | `8.76599ms` |

## Merry Hacking!!
