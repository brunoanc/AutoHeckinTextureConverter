extern crate texpresso;

use texpresso::Format;

// Texture material kind for bimage enum
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum TextureMaterialKind {
    TmkNone                 = 0x0,
    TmkAlbedo               = 0x1,
    TmkSpecular             = 0x2,
    TmkNormal               = 0x3,
    TmkSmoothness           = 0x4,
    TmkCover                = 0x5,
    TmkSssmask              = 0x6,
    TmkColormask            = 0x7,
    TmkBloommask            = 0x8,
    TmkHeightmap            = 0x9,
    TmkDecalalbedo          = 0xA,
    TmkDecalnormal          = 0xB,
    TmkDecalspecular        = 0xC,
    TmkLightproject         = 0xD,
    TmkParticle             = 0xE,
    TmkUnused1              = 0xF,
    TmkUnused2              = 0x10,
    TmkLightmap             = 0x11,
    TmkUi                   = 0x12,
    TmkFont                 = 0x13,
    TmkLegacyFlashUi        = 0x14,
    TmkLightmapDirectional  = 0x15,
    TmkBlendmask            = 0x16,
    TmkCount                = 0x17
}

impl TextureMaterialKind {
    // Get texture material kind
    pub fn from_filename(file_name: String, stripped_file_name: String, format: TextureFormat) -> TextureMaterialKind {
        // Get material kind from filename
        if file_name.contains("$mtlkind=ui") {
            TextureMaterialKind::TmkUi
        }
        else if file_name.contains("$mtlkind=decalnormal") {
            TextureMaterialKind::TmkDecalnormal
        }
        else if file_name.contains("$mtlkind=decalalbedo") {
            TextureMaterialKind::TmkDecalalbedo
        }
        else if file_name.contains("$mtlkind=decalspecular") {
            TextureMaterialKind::TmkDecalspecular
        }
        else if file_name.contains("$mtlkind=particle") {
            TextureMaterialKind::TmkParticle
        }
        else if file_name.contains("$mtlkind=heightmap") {
            TextureMaterialKind::TmkHeightmap
        }
        else if stripped_file_name.ends_with("_n") || file_name.ends_with("_Normal") {
            TextureMaterialKind::TmkNormal
        }
        else if stripped_file_name.ends_with("_s") {
            TextureMaterialKind::TmkSpecular
        }
        else if stripped_file_name.ends_with("_g") {
            TextureMaterialKind::TmkSmoothness
        }
        else if stripped_file_name.ends_with("_e") {
            TextureMaterialKind::TmkBloommask
        }
        else if stripped_file_name.ends_with("_h") {
            TextureMaterialKind::TmkHeightmap
        }
        else if stripped_file_name.ends_with("_sss") {
            TextureMaterialKind::TmkSssmask
        }
        else if format == TextureFormat::FmtBc1Srgb {
            TextureMaterialKind::TmkAlbedo
        }
        else {
            TextureMaterialKind::TmkNone
        }
    }
}

// DDS texture formats used by bimage
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TextureFormat {
    FmtNone            = 0x0,
    FmtRgba32f         = 0x1,
    FmtRgba16f         = 0x2,
    FmtRgba8           = 0x3,
    FmtArgb8           = 0x4,
    FmtAlpha           = 0x5,
    FmtL8a8Deprecated  = 0x6,
    FmtRg8             = 0x7,
    FmtLum8Deprecated  = 0x8,
    FmtInt8Deprecated  = 0x9,
    FmtBc1             = 0xA,
    FmtBc3             = 0xB,
    FmtDepth           = 0xC,
    FmtDepthStencil    = 0xD,
    FmtX32f            = 0xE,
    FmtY16fX16f        = 0xF,
    FmtX16             = 0x10,
    FmtY16X16          = 0x11,
    FmtRgb565          = 0x12,
    FmtR8              = 0x13,
    FmtR11fg11fb10f    = 0x14,
    FmtX16f            = 0x15,
    FmtBc6hUf16        = 0x16,
    FmtBc7             = 0x17,
    FmtBc4             = 0x18,
    FmtBc5             = 0x19,
    FmtRg16f           = 0x1A,
    FmtR10g10b10a2     = 0x1B,
    FmtRg32f           = 0x1C,
    FmtR32Uint         = 0x1D,
    FmtR16Uint         = 0x1E,
    FmtDepth16         = 0x1F,
    FmtRgba8Srgb       = 0x20,
    FmtBc1Srgb         = 0x21,
    FmtBc3Srgb         = 0x22,
    FmtBc7Srgb         = 0x23,
    FmtBc6hSf16        = 0x24,
    FmtAstc4x4         = 0x25,
    FmtAstc4x4Srgb     = 0x26,
    FmtAstc5x4         = 0x27,
    FmtAstc5x4Srgb     = 0x28,
    FmtAstc5x5         = 0x29,
    FmtAstc5x5Srgb     = 0x2A,
    FmtAstc6x5         = 0x2B,
    FmtAstc6x5Srgb     = 0x2C,
    FmtAstc6x6         = 0x2D,
    FmtAstc6x6Srgb     = 0x2E,
    FmtAstc8x5         = 0x2F,
    FmtAstc8x5Srgb     = 0x30,
    FmtAstc8x6         = 0x31,
    FmtAstc8x6Srgb     = 0x32,
    FmtAstc8x8         = 0x33,
    FmtAstc8x8Srgb     = 0x34,
    FmtDepth32f        = 0x35,
    FmtBc1ZeroAlpha    = 0x36,
    FmtNextAvailable   = 0x37
}

impl TextureFormat {
    // Get block size for format
    pub fn block_size(&self) -> Option<u32> {
        match self {
            TextureFormat::FmtBc1Srgb => Some(8),
            TextureFormat::FmtBc3 => Some(16),
            TextureFormat::FmtBc4 => Some(8),
            TextureFormat::FmtBc5 => Some(16),
            TextureFormat::FmtBc7 => Some(16),
            _ => None
        }
    }

    // Convert to texpresso format for compression
    pub fn as_texpresso_format(&self) -> Result<Format, String> {
        match self {
            TextureFormat::FmtBc1Srgb => Ok(Format::Bc1),
            TextureFormat::FmtBc3 => Ok(Format::Bc3),
            TextureFormat::FmtBc4 => Ok(Format::Bc4),
            TextureFormat::FmtBc5 => Ok(Format::Bc5),
            _ => Err("Unsupported target BCn format".into())
        }
    }
}

// BIM header
pub struct BIMHeader {
    pub signature: [u8; 3],
    pub version: u8,
    pub texture_type: u32,
    pub texture_material_kind: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub depth: u32,
    pub mip_count: u32,
    pub mip_level: i64,
    pub unk_float_1: f32,
    pub bool_is_environment_map: u8,
    pub texture_format: u32,
    pub always_7: u32,
    pub null_padding: u32,
    pub atlas_padding: i16,
    pub bool_is_streamed: u8,
    pub unk_bool: u8,
    pub bool_no_mips: u8,
    pub bool_fft_bloom: u8,
    pub streamdb_mip_count: u32
}

impl Default for BIMHeader {
    fn default() -> Self {
        Self {
            signature: [0x42, 0x49, 0x4D],
            version: 0x15,
            texture_type: 0,
            texture_material_kind: 0,
            pixel_width: 0,
            pixel_height: 0,
            depth: 0,
            mip_count: 0,
            mip_level: 0,
            unk_float_1: 1.0,
            bool_is_environment_map: 0,
            texture_format: 0,
            always_7: 7,
            null_padding: 0,
            atlas_padding: 0,
            bool_is_streamed: 0,
            unk_bool: 0,
            bool_no_mips: 0,
            bool_fft_bloom: 0,
            streamdb_mip_count: 0
        }
    }
}

impl BIMHeader {
    // Convert BIMHeader to bytes representation
    pub fn to_bytes(&self) -> [u8; 63] {
        let mut bytes = [0_u8; 63];

        bytes[0..3].copy_from_slice(&self.signature);
        bytes[3] = self.version;
        bytes[4..8].copy_from_slice(&self.texture_type.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.texture_material_kind.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.pixel_width.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.pixel_height.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.depth.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.mip_count.to_le_bytes());
        bytes[28..36].copy_from_slice(&self.mip_level.to_le_bytes());
        bytes[36..40].copy_from_slice(&self.unk_float_1.to_le_bytes());
        bytes[40] = self.bool_is_environment_map;
        bytes[41..45].copy_from_slice(&self.texture_format.to_le_bytes());
        bytes[45..49].copy_from_slice(&self.always_7.to_le_bytes());
        bytes[49..53].copy_from_slice(&self.null_padding.to_le_bytes());
        bytes[53..55].copy_from_slice(&self.atlas_padding.to_le_bytes());
        bytes[55] = self.bool_is_streamed;
        bytes[56] = self.unk_bool;
        bytes[57] = self.bool_no_mips;
        bytes[58] = self.bool_fft_bloom;
        bytes[59..63].copy_from_slice(&self.streamdb_mip_count.to_le_bytes());

        bytes
    }
}

// BIM mipmap
pub struct BIMMipMap {
    pub mip_level: i64,
    pub mip_pixel_width: u32,
    pub mip_pixel_height: u32,
    pub unknown_flag: u32,
    pub decompressed_size: u32,
    pub flag_is_compressed: u32,
    pub compressed_size: u32,
    pub cumulative_size_streamdb: u32
}

impl Default for BIMMipMap {
    fn default() -> Self {
        Self {
            mip_level: 0,
            mip_pixel_width: 0,
            mip_pixel_height: 0,
            unknown_flag: 1,
            decompressed_size: 0,
            flag_is_compressed: 0,
            compressed_size: 0,
            cumulative_size_streamdb: 0
        }
    }
}

impl BIMMipMap {
    // Convert BIMMipMap to bytes representation
    pub fn to_bytes(&self) -> [u8; 36] {
        let mut bytes = [0_u8; 36];

        bytes[0..8].copy_from_slice(&self.mip_level.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.mip_pixel_width.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.mip_pixel_height.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.unknown_flag.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.decompressed_size.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.flag_is_compressed.to_le_bytes());
        bytes[28..32].copy_from_slice(&self.compressed_size.to_le_bytes());
        bytes[32..36].copy_from_slice(&self.cumulative_size_streamdb.to_le_bytes());

        bytes
    }
}
