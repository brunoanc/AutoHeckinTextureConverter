fn main() {
    // Set search path
    println!("cargo:rustc-link-search=native=./lib/");

    #[cfg(target_os = "linux")]
    // Link to c++ library
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
