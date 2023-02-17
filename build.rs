use std::env::consts;

fn main() {
    // Set search path
    println!("cargo:rustc-link-search=native=./lib/");

    // Link to c++ library
    if consts::OS == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
