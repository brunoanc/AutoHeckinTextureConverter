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
    pub opaque_settings: OpaqueSettings,
    pub alpha_settings: AlphaSettings
}

impl CompressBlockParams {
    pub const fn ultrafast() -> Self {
        Self {
            max_partitions_mode: [16, 64, 64, 64, 0, 0, 0, 64],
            weights: [128, 64, 16, 256],
            uber_level: 0,
            refinement_passes: 1,
            mode4_rotation_mask: 1 + 4,
            mode4_index_mask: 3,
            mode5_rotation_mask: 1,
            uber1_mask: 7,
            perceptual: true,
            pbit_search: false,
            mode6_only: true,
            unused0: false,
            opaque_settings: OpaqueSettings {
                max_mode13_partitions_to_try: 1,
                max_mode0_partitions_to_try: 1,
                max_mode2_partitions_to_try: 1,
                use_mode: [true; 7],
                unused1: false
            },
            alpha_settings: AlphaSettings {
                max_mode7_partitions_to_try: 1,
                mode67_error_weight_mul: [1; 4],
                use_mode4: true,
                use_mode5: true,
                use_mode6: true,
                use_mode7: false,
                use_mode4_rotation: true,
                use_mode5_rotation: true,
                unused2: false,
                unused3: false
            }
        }
    }
}

#[link(name = "bc7e", kind = "static")]
extern "C" {
    #[link_name = "bc7e_compress_block_init"]
    pub fn compress_block_init();
    #[link_name = "bc7e_compress_blocks"]
    pub fn compress_blocks(
        num_blocks: u32, blocks: *mut u64, pixels_rgba: *const u32, comp_params: *const CompressBlockParams
    );
}
