// ooz library bindings

#[link(name = "ooz", kind = "static")]
extern "C" {
    #[link_name = "Kraken_Compress"]
    pub fn kraken_compress(src: *mut u8, src_len: usize, dst: *mut u8, level: i32) -> i32;
}
