extern crate ispc;

use std::env::consts::OS;
use std::path::Path;
use ispc::TargetISA;

// Build script
fn main() {
    // Get lib name and path
    let lib_path = match OS {
        "windows" => "./lib/ooz.lib",
        "linux" => "./lib/libooz.a",
        _ => panic!("Unsupported OS")
    };

    // Check if the ooz lib exists
    if !Path::new(lib_path).exists() {
        panic!("Missing ooz lib file ({})", lib_path);
    }

    // Link to ooz
    println!("cargo:rustc-link-search=native=./lib/");
    println!("cargo:rustc-link-lib=static=ooz");

    // Link to c++ library
    if OS == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    // Invalidate the built crate if the ooz lib changes
    println!("cargo:rerun-if-changed={}", lib_path);

    // Invalidate the built crate if the bc7e ispc file changes
    println!("cargo:rerun-if-changed=lib/bc7e.ispc");

    // Compile bc7e
    ispc::Config::new()
        .file("lib/bc7e.ispc")
        .target_isas(vec![
            TargetISA::SSE2i32x4,
            TargetISA::SSE4i32x4,
            TargetISA::AVX1i32x8,
            TargetISA::AVX2i32x8,
            TargetISA::AVX512KNLi32x16,
            TargetISA::AVX512SKXi32x16])
        .opt_level(2)
        .optimization_opt(ispc::OptimizationOpt::DisableAssertions)
        .optimization_opt(ispc::OptimizationOpt::FastMath)
        .compile("bc7e");
}
