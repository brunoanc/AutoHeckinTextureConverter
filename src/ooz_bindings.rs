// ooz library bindings
pub mod ooz {
    extern "C" {
        pub fn Kraken_Compress(src: *mut u8, src_len: usize, dst: *mut u8, level: i32) -> i32;
    }
}
