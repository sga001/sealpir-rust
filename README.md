# SealPIR (Rust): Rust wrappers for SealPIR

SealPIR is a research library and should not be used in production systems. SealPIR allows a client to download an element from a database stored by a server without revealing which element was downloaded. SealPIR was introduced in our [paper](https://eprint.iacr.org/2017/1142.pdf).


SealPIR relies on SEAL. The rest of this README assumes that SEAL is installed in the folder deps/SEAL within this repository. See below for instructions on how to install SEAL.

# Compiling SEAL

SealPIR depends on SEAL v2.3.0-4 and a patch that exposes the substitution operator. You can get SEAL v2.3.0-4 from this [link](http://sealcrypto.org).

Once you have downloaded SEAL, apply the patch SEAL_v2.3.0-4.patch (available in this repository) to it. Here are the exact steps. 

We assume that you are in the SEAL directory, and that you have copied the patch to this directory.

First, convert the SEAL directory into a git repo:

```sh
$ git init
$ git add .
$ git commit -m "SEAL v2.3.0-4"
```
Then, apply the patch:

```sh
$ git am SEAL_v2.3.0-4.patch
```

Finally, compile SEAL (NOTE: gcc-8 is not currently supported):

```sh
$ cd SEAL
$ ./configure CXXFLAGS="-O3 -march=native -fPIC"
$ make clean && make
```

# Compiling SealPIR-Rust

SealPIR's Rust wrapper works with [Rust](https://www.rust-lang.org/) nightly (we have tested with Rust 1.28.0). It also depends on the C++ version of SealPIR (included as a submodule) and SEAL (see above).

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
