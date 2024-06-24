# jacobi-benchmarks

This repo contains a new, optimised implementation of an algorithm to compute the Jacobi symbol.

The algorithm is an optimisation of the one often presented in textbooks, and also on [Wikipedia](https://en.wikipedia.org/wiki/Jacobi_symbol#Calculating_the_Jacobi_symbol).

The optimised version, `jacobi_new`, is 72% faster, and it is 45% faster than the only other Rust implementation available
for large integers which is found in the [num-bigint-dig](https://github.com/rust-num/num-bigint) crate.

The output of the benchmarks from a Macbook Pro M1 is as follows. All timings are in µs.

| Input size (bits)   | 	128     | 256      | 384      | 512      | 768      | 1024     | 2048     | 3072     | 
|---------------------|----------|----------|----------|----------|----------|----------|----------|----------| 
| Base                | 15.58    | 35.476   | 55.605   | 79.086   | 131.5    | 191.9    | 541.61   | 1091.5   | 
| New                 | 3.9589   | 9.883    | 17.15    | 23.423   | 37.026   | 53.229   | 146.26   | 287.22   | 
| num-bigint-dig      | 6.1464   | 17.199   | 32.904   | 45.877   | 75.78    | 105.62   | 259.33   | 521.38   | 
| ------------------- | -------- | -------- | -------- | -------- | -------- | -------- | -------- | -------- | 

To run the benchmarks, run `cargo bench` in the root of the repo.