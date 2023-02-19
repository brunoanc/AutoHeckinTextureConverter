// bc7e library bindings

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct OpaqueSettings {
    pub max_mode13_partitions_to_try: u32,
    pub max_mode0_partitions_to_try: u32,
    pub max_mode2_partitions_to_try: u32,
    pub use_mode: [bool; 7usize],
    pub unused1: bool
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct AlphaSettings {
    pub max_mode7_partitions_to_try: u32,
    pub mode67_error_weight_mul: [u32; 4usize],
    pub use_mode4: bool,
    pub use_mode5: bool,
    pub use_mode6: bool,
    pub use_mode7: bool,
    pub use_mode4_rotation: bool,
    pub use_mode5_rotation: bool,
    pub unused2: bool,
    pub unused3: bool
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CompressBlockParams {
    pub max_partitions_mode: [u32; 8usize],
    pub weights: [u32; 4usize],
    pub uber_level: u32,
    pub refinement_passes: u32,
    pub mode4_rotation_mask: u32,
    pub mode4_index_mask: u32,
    pub mode5_rotation_mask: u32,
    pub uber1_mask: u32,
    pub perceptual: bool,
    pub pbit_search: bool,
    pub mode6_only: bool,
    pub unused0: bool,
    pub opaque_settings:OpaqueSettings,
    pub alpha_settings: AlphaSettings
}

#[link(name = "bc7e", kind = "static")]
extern "C" {
    #[link_name = "bc7e_compress_block_init"]
    pub fn compress_block_init();
    #[link_name = "bc7e_compress_block_params_init_ultrafast"]
    pub fn compress_block_params_init_ultrafast(comp_params: *mut CompressBlockParams, perceptual: bool);
    #[link_name = "bc7e_compress_blocks"]
    pub fn compress_blocks(num_blocks: u32, blocks: *mut u64,
        pixels_rgba: *const u32, comp_params: *const CompressBlockParams);
}
