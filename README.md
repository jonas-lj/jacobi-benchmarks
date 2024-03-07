# jacobi-benchmarks

This repo contains a new, optimised implementation of an algorithm to compute the Jacobi symbol.

The algorithm is an optimisation of the one often presented in textbooks, and also on [Wikipedia](https://en.wikipedia.org/wiki/Jacobi_symbol#Calculating_the_Jacobi_symbol).

The optimised version, `jacobi_new`, is 70% faster, and it is 40% faster than the only other Rust implementation available
for large integers which is found in the [num-bigint-dig](https://github.com/rust-num/num-bigint) crate.

To run the benchmarks, run `cargo bench` in the root of the repo.