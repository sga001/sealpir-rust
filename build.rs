extern crate gcc;
extern crate cmake;

use std::env;

fn main() {
    gcc::Build::new()
        .file("sealpir/pir.cpp")
        .file("sealpir/pir_server.cpp")
        .file("sealpir/pir_client.cpp")
        .file("sealpir-bindings/pir_rust.cpp")
        .include("sealpir-bindings/")
        .include("sealpir/")
        .include("deps/SEAL_2.3.1/SEAL/")
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-std=c++17")
        .flag("-fopenmp")
        .pic(true)
        .cpp(true)
        .compile("libsealpir.a");

    // Compile and link SEAL
    let dst = cmake::Config::new("deps/SEAL_2.3.1/SEAL/")
//        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .build();

    println!("cargo:rustc-link-search={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=seal");
}
