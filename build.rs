use std::env::consts::OS;

// Build script
fn main() {
    // Get lib name and path
    let (bc7e_lib, ooz_lib) = match OS {
        "windows" => ("./lib/bc7e.lib", "./lib/ooz.lib"),
        "linux" => ("./lib/libbc7e.a", "./lib/libooz.a"),
        _ => panic!("Unsupported OS")
    };

    // Link to libs
    println!("cargo:rustc-link-search=native=./lib/");
    println!("cargo:rustc-link-lib=static=bc7e");
    println!("cargo:rustc-link-lib=static=ooz");

    // Link to c++ library
    if OS == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    // Invalidate the built crate if the libs change
    println!("cargo:rerun-if-changed={}", bc7e_lib);
    println!("cargo:rerun-if-changed={}", ooz_lib);
}
