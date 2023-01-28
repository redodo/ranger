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
$ multitime -n20 -q -s0 -i "cat samples/1m.txt" ranger
===> multitime results
1: -i "cat samples/1m.txt" -q ranger
            Mean        Std.Dev.    Min         Median      Max
real        0.185       0.011       0.167       0.192       0.196
user        0.160       0.013       0.138       0.165       0.177
sys         0.025       0.004       0.016       0.026       0.030
```

[edelooff/carrange](https://github.com/edelooff/carrange) (C++):

```bash
$ multitime -n20 -q -s0 -i "cat samples/1m.txt" composer
===> multitime results
1: -i "cat samples/1m.txt" -q composer
            Mean        Std.Dev.    Min         Median      Max
real        0.205       0.013       0.178       0.210       0.217
user        0.178       0.014       0.150       0.179       0.200
sys         0.026       0.008       0.013       0.026       0.043
```

[Gradecak/rs-bouquets](https://github.com/Gradecak/rs-bouquets) (Rust):

```bash
$ multitime -n20 -q -s0 -i "cat samples/1m.txt" rs-bouquets
===> multitime results
1: -i "cat samples/1m.txt" -q rs-bouquets
            Mean        Std.Dev.    Min         Median      Max
real        0.402       0.017       0.374       0.411       0.431
user        0.376       0.020       0.346       0.381       0.407
sys         0.024       0.009       0.010       0.025       0.045
```
