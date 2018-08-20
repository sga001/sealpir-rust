# SealPIR (Rust): Rust wrappers for SealPIR

SealPIR is a research library and should not be used in production systems. SealPIR allows a client to download an element from a database stored by a server without revealing which element was downloaded. SealPIR was introduced in our [paper](https://eprint.iacr.org/2017/1142.pdf).

SealPIR relies on SEAL v2.3.1. The rest of this README assumes that the SEAL v2.3.1 source code is placed in the folder 
deps/SEAL_2.3.1 within this repository. You can get SEAL v2.3.1 from this [link](http://sealcrypto.org).


# Compiling SealPIR-Rust

SealPIR's Rust wrapper works with [Rust](https://www.rust-lang.org/) nightly (we have tested with Rust 1.30.0). It also depends on the C++ version of SealPIR (included as a submodule) and SEAL (see above). We have tested these versions with g++ 8.1.0.

To compile SealPIR-Rust just do:

```sh
$ git submodule init
$ git submodule update
$ cargo build
```

# Reproducing the results in the paper

If you would like to reproduce the microbenchmarks found in the paper (Figure 9), simply run:

```sh
$ cargo bench [prefix of name of benchmark (or leave blank to run all)]
```

For example, to reproduce the SealPIR entries of the first row of Figure 9 (Query), simply
run: 

```sh
$ cargo bench query
```

To reproduce a single data point, for example the Expand entry for SealPIR where n=262,144, run:

```sh
$ cargo bench expand_d2/262144
```

Note that the reply microbenchmark ("Answer" in Figure 9) already includes the cost of Expand (we subtract this cost in the paper).


You can find the code that runs these benchmarks (and their names) in ``benches/pir.rs``.

To reproduce latency and throughput results, check out the [pir-test](https://github.com/sga001/pir-test) repository (this also has examples on how to use SealPIR in a client-server networked application).
