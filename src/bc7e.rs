// bc7e library bindings

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _anon0_ {
    pub m_max_mode13_partitions_to_try: u32,
    pub m_max_mode0_partitions_to_try: u32,
    pub m_max_mode2_partitions_to_try: u32,
    pub m_use_mode: [bool; 7usize],
    pub m_unused1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _anon1_ {
    pub m_max_mode7_partitions_to_try: u32,
    pub m_mode67_error_weight_mul: [u32; 4usize],
    pub m_use_mode4: bool,
    pub m_use_mode5: bool,
    pub m_use_mode6: bool,
    pub m_use_mode7: bool,
    pub m_use_mode4_rotation: bool,
    pub m_use_mode5_rotation: bool,
    pub m_unused2: bool,
    pub m_unused3: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bc7e_compress_block_params {
    pub m_max_partitions_mode: [u32; 8usize],
    pub m_weights: [u32; 4usize],
    pub m_uber_level: u32,
    pub m_refinement_passes: u32,
    pub m_mode4_rotation_mask: u32,
    pub m_mode4_index_mask: u32,
    pub m_mode5_rotation_mask: u32,
    pub m_uber1_mask: u32,
    pub m_perceptual: bool,
    pub m_pbit_search: bool,
    pub m_mode6_only: bool,
    pub m_unused0: bool,
    pub m_opaque_settings: _anon0_,
    pub m_alpha_settings: _anon1_,
}

extern "C" {
    pub fn bc7e_compress_block_init();
    pub fn bc7e_compress_block_params_init_ultrafast(p: *mut bc7e_compress_block_params, perceptual: bool);
    pub fn bc7e_compress_blocks(num_blocks: u32, pBlocks: *mut u64,
        pPixelsRGBA: *const u32, pComp_params: *const bc7e_compress_block_params);
}
