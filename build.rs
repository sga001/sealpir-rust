extern crate gcc;
use std::env;

fn main() {
    gcc::Build::new()
        .file("SealPIR/pir.cpp")
        .file("SealPIR/pir_server.cpp")
        .file("SealPIR/pir_client.cpp")
        .file("sealpir-bindings/pir_rust.cpp")
        .include("sealpir-bindings/")
        .include("SealPIR/")
        .include("deps/SEAL/SEAL/")
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-std=c++11")
        .flag("-fopenmp")
        .pic(true)
        .cpp(true)
        .compile("libsealpir.a");

    let link_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/deps/SEAL/bin/", link_dir);
    println!("cargo:rustc-link-lib=static=seal");
}
