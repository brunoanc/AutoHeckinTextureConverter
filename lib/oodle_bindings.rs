// Oodle decompression bindings

use std::os::raw;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct OodleLZ_CompressOptions {
    pub unused_was_verbosity: raw::c_uint,
    pub min_match_len: raw::c_int,
    pub seek_chunk_reset: raw::c_int,
    pub seek_chunk_len: raw::c_int,
    pub profile: raw::c_int,
    pub dictionary_size: raw::c_int,
    pub space_speed_tradeoff_bytes: raw::c_int,
    pub unused_was_max_huffmans_per_chunk: raw::c_int,
    pub send_quantum_crcs: raw::c_int,
    pub max_local_dictionary_size: raw::c_int,
    pub make_long_range_matcher: raw::c_int,
    pub match_table_size_log2: raw::c_int,
    pub jobify: raw::c_int,
    pub jobify_user_ptr: *mut raw::c_void,
    pub far_match_min_len: raw::c_int,
    pub far_match_offset_log2: raw::c_int,
    pub reserved: [raw::c_uint; 4usize],
}

extern "C" {
    pub fn OodleLZ_Compress(
        compressor: raw::c_int,
        raw_buf: *const raw::c_void,
        raw_len: raw::c_int,
        comp_buf: *mut raw::c_void,
        level: raw::c_int,
        p_options: *const OodleLZ_CompressOptions,
        dictionary_base: *const raw::c_void,
        lrm: *const raw::c_void,
        scratch_mem: *mut raw::c_void,
        scratch_size: raw::c_int,
    ) -> raw::c_int;
}
