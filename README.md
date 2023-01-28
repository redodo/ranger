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
real        0.189       0.014       0.165       0.194       0.221
user        0.164       0.015       0.134       0.162       0.194
sys         0.024       0.008       0.007       0.023       0.050
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
