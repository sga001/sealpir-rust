use std::env;

fn main() {
    let path = env::current_dir().unwrap();

    let mut pir_cpp = path.clone();
    pir_cpp.push("sealpir/pir.cpp");

    let mut pir_server = path.clone();
    pir_server.push("sealpir/pir_server.cpp");

    let mut pir_client = path.clone();
    pir_client.push("sealpir/pir_client.cpp");

    let mut pir_bindings_file = path.clone();
    pir_bindings_file.push("sealpir-bindings/pir_rust.cpp");

    let mut pir_bindings = path.clone();
    pir_bindings.push("sealpir-bindings");

    let mut sealpir_dir = path.clone();
    sealpir_dir.push("sealpir");

    let mut seal_dir = path;
    seal_dir.push("deps/SEAL_2.3.1/SEAL/");

    cc::Build::new()
        .file(pir_cpp)
        .file(pir_server)
        .file(pir_client)
        .file(pir_bindings_file)
        .include(pir_bindings)
        .include(sealpir_dir)
        .include(seal_dir.clone())
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-std=c++17")
        .flag("-fopenmp")
        .pic(true)
        .cpp(true)
        .compile("libsealpir.a");

    // Compile and link SEAL
    let dst = cmake::Config::new(seal_dir)
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .build();

    println!("cargo:rustc-link-search={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=seal");
}
