extern crate bindgen;

use std::env::{var, consts::OS};
use std::path::{Path, PathBuf};

// Build script
fn main() {
    // Get lib name and path
    let (lib, lib_path) = match OS {
        "windows" => ("oo2core_win64", "./lib/oo2core_win64.lib"),
        "linux" => ("oo2corelinux64", "./lib/liboo2corelinux64.a"),
        "macos" => ("oo2coremac64", "./lib/liboo2coremac64.a"),
        _ => panic!("Unsupported OS")
    };

    // Check if the oodle lib exists
    if !Path::new(lib_path).exists() {
        panic!("Missing oodle lib file ({})", lib_path);
    }

    // Link to oodle
    println!("cargo:rustc-link-search=native=./lib/");
    println!("cargo:rustc-link-lib=static={}", lib);

    // Link to c++ library
    if OS == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    else if OS == "macos" {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    // Invalidate the built crate if the oodle lib/wrapper changes
    println!("cargo:rerun-if-changed={}", lib_path);
    println!("cargo:rerun-if-changed=lib/oodle.h");

    // Create bindings for oodle
    let bindings = bindgen::Builder::default()
        .header("lib/oodle.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to $OUT_DIR/oodle_bindings.rs.
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("oodle_bindings.rs"))
        .expect("Couldn't write bindings!");
}
