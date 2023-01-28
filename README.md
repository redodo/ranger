# Rust solution of bloomon challenge using SIMD

This solution uses 512-bit SIMD vectors to store and compare stem stock to stem
requirements per design. The vectors that are used have 32 lanes of `u16` values. This
allows for every possible species (a to z) to have a maximum of 65535 of stock.

The feasibility of a design can be checked in 5 operations, regardless of the amount of
flowers in a design:

1. Does the stock contain the minimum of flowers in the design (2 operations): `any(stock < design_min_stems)`
2. Take the most stems we can take: `take = min(stock, design_max_stems)`
3. Calculate how many stems we took: `took = reduce_sum(take)`
4. Did we take enough stems: `took >= design_total`

After these operations we are left with a bunch of stems we took and the total amount
of them. Because of the nature of `min(stock, design_max_stems)`, it's very likely that
we took too many stems. In that case we need to remove some stems from our hand. This
operation is quite simple and comes down to iterating over all the stems and returning
the maximum amount we can.

## Comparison to other solutions

*These benchmarks were run on an AMD Ryzen 7 1700.*

This one (Rust):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" ranger
===> multitime results
1: -i "cat samples/1m.txt" -q ranger
            Mean        Std.Dev.    Min         Median      Max
real        0.162       0.014       0.141       0.172       0.190
user        0.140       0.015       0.103       0.138       0.169
sys         0.022       0.008       0.007       0.020       0.041
```

[edelooff/carrange](https://github.com/edelooff/carrange) (C++):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" composer
===> multitime results
1: -i "cat samples/1m.txt" -q composer
            Mean        Std.Dev.    Min         Median      Max
real        0.201       0.013       0.180       0.206       0.224
user        0.179       0.015       0.150       0.178       0.212
sys         0.022       0.008       0.003       0.020       0.047
```

[Gradecak/rs-bouquets](https://github.com/Gradecak/rs-bouquets) (Rust):

```bash
$ multitime -n100 -q -s0 -i "cat samples/1m.txt" rs-bouquets
===> multitime results
1: -i "cat samples/1m.txt" -q rs-bouquets
            Mean        Std.Dev.    Min         Median      Max
real        0.406       0.016       0.377       0.413       0.435
user        0.380       0.018       0.331       0.383       0.413
sys         0.025       0.008       0.007       0.023       0.046
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
