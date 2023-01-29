# Rust solution of bloomon challenge using SIMD

This solution uses 512-bit SIMD vectors to store and compare stem stock to stem
requirements per design. The vectors that are used have 32 lanes of `u16` values. This
allows for every possible species (a to z) to have a maximum of 65535 of stock.

The feasibility of a design can be checked in a couple of operations, regardless of the
amount of stems in a design:

1. Take the maximum amount of stems we can take: `stems = simd_min(stock, design.max_stems)`
2. Check if we have enough stems: `reduce_sum(stems) >= design.total`
3. Check if we at least one of each needed stem: `all(simd_ge(stems, design.min_stems))`

After this we have enough stems. However, since we took the maximum amount possible per
stem, we need to make sure that we return the excess amount of stems we took. This is
done over a simple iteration where stems are returned to the stock until the amount
required by the design is met.

Besides this a number of other optimizations are implemented:

- On input of a stem, only designs with that stem are checked.
- Total stems in stock is separately kept, and designs are skipped when the total stems
  in stock does not meet the design requirements.
- The implicit minimum of a species in a design is increased from 1 where possible. For
  example, *AL5a5* requires at least 5 times *a*, and *BL5a1b5* requires at least 4
  times *a*.
- The maximum species in a design is decreased where possible. For example, *AL10a5*
  can not have more than 5 times *a*, and *BL5a1b5* can not have more than 4 times *a*.

## How to run it

Make a release build:
```bash
cargo build --release
```

Run it:
```bash
target/release/ranger
```

## Comparison to other solutions

*These benchmarks were run on an AMD Ryzen 7 1700.*

This one (Rust):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" ranger
===> multitime results
1: -i "cat samples/1m.txt" -q ranger
            Mean        Std.Dev.    Min         Median      Max
real        0.131       0.012       0.114       0.129       0.151
user        0.110       0.013       0.078       0.111       0.134
sys         0.020       0.009       0.000       0.020       0.044
```

[edelooff/carrange](https://github.com/edelooff/carrange) (C++):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" composer
===> multitime results
1: -i "cat samples/1m.txt" -q composer
            Mean        Std.Dev.    Min         Median      Max
real        0.182       0.013       0.164       0.178       0.204
user        0.160       0.014       0.128       0.160       0.188
sys         0.021       0.009       0.000       0.020       0.040
```

[Gradecak/rs-bouquets](https://github.com/Gradecak/rs-bouquets) (Rust):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" rs-bouquets
===> multitime results
1: -i "cat samples/1m.txt" -q rs-bouquets
            Mean        Std.Dev.    Min         Median      Max
real        0.364       0.013       0.341       0.366       0.387
user        0.342       0.017       0.301       0.344       0.370
sys         0.021       0.008       0.000       0.020       0.047
```

## Benchmark of reference solution

This gets its own section because compiled languages are in a league of their own.

[reference solution](https://github.com/bloomon/code-challenge-verifier/blob/master/reference.py) (Python 3.10):

```bash
$ multitime -n10 -q -s0 -i "cat samples/1m.txt" python3.10 reference.py
===> multitime results
1: -i "cat samples/1m.txt" -q python3.10 reference.py
            Mean        Std.Dev.    Min         Median      Max
real        20.417      0.122       20.154      20.424      20.570
user        20.339      0.136       20.088      20.333      20.522
sys         0.031       0.029       0.006       0.021       0.111
```

Curiously, the reference solution was slower in Python 3.11 🤔

[reference solution](https://github.com/bloomon/code-challenge-verifier/blob/master/reference.py) (Python 3.11):

```bash
$ multitime -n10 -q -s0 -i "cat samples/1m.txt" python3.11 reference.py
===> multitime results
1: -i "cat samples/1m.txt" -q python3.11 reference.py
            Mean        Std.Dev.    Min         Median      Max
real        26.699      0.333       26.163      26.691      27.391
user        26.613      0.348       26.117      26.619      27.332
sys         0.026       0.026       0.000       0.016       0.091
```
