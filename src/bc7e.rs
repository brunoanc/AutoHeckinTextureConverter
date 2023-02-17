// bc7e library bindings

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpaqueSettings {
    pub max_mode13_partitions_to_try: u32,
    pub max_mode0_partitions_to_try: u32,
    pub max_mode2_partitions_to_try: u32,
    pub use_mode: [bool; 7usize],
    pub unused1: bool
}

impl Default for OpaqueSettings {
    fn default() -> Self {
        Self {
            max_mode13_partitions_to_try: 0,
            max_mode0_partitions_to_try: 0,
            max_mode2_partitions_to_try: 0,
            use_mode: [false; 7],
            unused1: false
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

impl Default for AlphaSettings {
    fn default() -> Self {
        Self {
            max_mode7_partitions_to_try: 0,
            mode67_error_weight_mul: [0; 4],
            use_mode4: false,
            use_mode5: false,
            use_mode6: false,
            use_mode7: false,
            use_mode4_rotation: false,
            use_mode5_rotation: false,
            unused2: false,
            unused3: false
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

impl Default for CompressBlockParams {
    fn default() -> Self {
        Self {
            max_partitions_mode: [0; 8],
            weights: [0; 4],
            uber_level: 0,
            refinement_passes: 0,
            mode4_rotation_mask: 0,
            mode4_index_mask: 0,
            mode5_rotation_mask: 0,
            uber1_mask: 0,
            perceptual: false,
            pbit_search: false,
            mode6_only: false,
            unused0: false,
            opaque_settings: OpaqueSettings::default(),
            alpha_settings: AlphaSettings::default()
        }
    }
}

#[link(name = "bc7e", kind = "static")]
extern "C" {
    #[link_name = "bc7e_compress_block_init"]
    pub fn compress_block_init();
    #[link_name = "bc7e_compress_block_params_init_ultrafast"]
    pub fn compress_block_params_init_ultrafast(p: *mut CompressBlockParams, perceptual: bool);
    #[link_name = "bc7e_compress_blocks"]
    pub fn compress_blocks(num_blocks: u32, pBlocks: *mut u64,
        pPixelsRGBA: *const u32, pComp_params: *const CompressBlockParams);
}
