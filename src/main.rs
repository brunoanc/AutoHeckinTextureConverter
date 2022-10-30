include!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/oodle_bindings.rs"));

extern crate image;
extern crate ispc_texcomp;

#[cfg(target_os = "windows")]
extern crate windows_sys;

#[macro_use]
extern crate ispc;

ispc_module!(bc7e);

use std::env::{args, var};
use std::process::exit;
use std::cmp::{min, max};
use std::error::Error;
use std::ffi::c_void;
use std::fs::File;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};
use std::sync::{Mutex, Arc};
use std::thread;
use ispc_texcomp::{bc1, bc3, bc4, bc5, bc6h, RgbaSurface};
use image::{DynamicImage, GenericImageView, imageops::FilterType, io::Reader};

#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Console::GetConsoleProcessList;
#[cfg(target_os = "windows")]
use std::io::{stdin, Read};

// DXGI format types
#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum DxgiFormat {
    Unknown                     = 0,
    R32G32B32A32_Typeless       = 1,
    R32G32B32A32_Float          = 2,
    R32G32B32A32_UInt           = 3,
    R32G32B32A32_SInt           = 4,
    R32G32B32_Typeless          = 5,
    R32G32B32_Float             = 6,
    R32G32B32_UInt              = 7,
    R32G32B32_SInt              = 8,
    R16G16B16A16_Typeless       = 9,
    R16G16B16A16_Float          = 10,
    R16G16B16A16_UNorm          = 11,
    R16G16B16A16_UInt           = 12,
    R16G16B16A16_SNorm          = 13,
    R16G16B16A16_SInt           = 14,
    R32G32_Typeless             = 15,
    R32G32_Float                = 16,
    R32G32_UInt                 = 17,
    R32G32_SInt                 = 18,
    R32G8X24_Typeless           = 19,
    D32_Float_S8X24_UInt        = 20,
    R32_Float_X8X24_Typeless    = 21,
    X32_Typeless_G8X24_UInt     = 22,
    R10G10B10A2_Typeless        = 23,
    R10G10B10A2_UNorm           = 24,
    R10G10B10A2_UInt            = 25,
    R11G11B10_Float             = 26,
    R8G8B8A8_Typeless           = 27,
    R8G8B8A8_UNorm              = 28,
    R8G8B8A8_UNorm_sRGB         = 29,
    R8G8B8A8_UInt               = 30,
    R8G8B8A8_SNorm              = 31,
    R8G8B8A8_SInt               = 32,
    R16G16_Typeless             = 33,
    R16G16_Float                = 34,
    R16G16_UNorm                = 35,
    R16G16_UInt                 = 36,
    R16G16_SNorm                = 37,
    R16G16_SInt                 = 38,
    R32_Typeless                = 39,
    D32_Float                   = 40,
    R32_Float                   = 41,
    R32_UInt                    = 42,
    R32_SInt                    = 43,
    R24G8_Typeless              = 44,
    D24_UNorm_S8_UInt           = 45,
    R24_UNorm_X8_Typeless       = 46,
    X24_Typeless_G8_UInt        = 47,
    R8G8_Typeless               = 48,
    R8G8_UNorm                  = 49,
    R8G8_UInt                   = 50,
    R8G8_SNorm                  = 51,
    R8G8_SInt                   = 52,
    R16_Typeless                = 53,
    R16_Float                   = 54,
    D16_UNorm                   = 55,
    R16_UNorm                   = 56,
    R16_UInt                    = 57,
    R16_SNorm                   = 58,
    R16_SInt                    = 59,
    R8_Typeless                 = 60,
    R8_UNorm                    = 61,
    R8_UInt                     = 62,
    R8_SNorm                    = 63,
    R8_SInt                     = 64,
    A8_UNorm                    = 65,
    R1_UNorm                    = 66,
    R9G9B9E5_SharedExp          = 67,
    R8G8_B8G8_UNorm             = 68,
    G8R8_G8B8_UNorm             = 69,
    BC1_Typeless                = 70,
    BC1_UNorm                   = 71,
    BC1_UNorm_sRGB              = 72,
    BC2_Typeless                = 73,
    BC2_UNorm                   = 74,
    BC2_UNorm_sRGB              = 75,
    BC3_Typeless                = 76,
    BC3_UNorm                   = 77,
    BC3_UNorm_sRGB              = 78,
    BC4_Typeless                = 79,
    BC4_UNorm                   = 80,
    BC4_SNorm                   = 81,
    BC5_Typeless                = 82,
    BC5_UNorm                   = 83,
    BC5_SNorm                   = 84,
    B5G6R5_UNorm                = 85,
    B5G5R5A1_UNorm              = 86,
    B8G8R8A8_UNorm              = 87,
    B8G8R8X8_UNorm              = 88,
    R10G10B10_XR_Bias_A2_UNorm  = 89,
    B8G8R8A8_Typeless           = 90,
    B8G8R8A8_UNorm_sRGB         = 91,
    B8G8R8X8_Typeless           = 92,
    B8G8R8X8_UNorm_sRGB         = 93,
    BC6H_Typeless               = 94,
    BC6H_UF16                   = 95,
    BC6H_SF16                   = 96,
    BC7_Typeless                = 97,
    BC7_UNorm                   = 98,
    BC7_UNorm_sRGB              = 99,
    AYUV                        = 100,
    Y410                        = 101,
    Y416                        = 102,
    NV12                        = 103,
    P010                        = 104,
    P016                        = 105,
    Format_420_Opaque           = 106,
    YUY2                        = 107,
    Y210                        = 108,
    Y216                        = 109,
    NV11                        = 110,
    AI44                        = 111,
    IA44                        = 112,
    P8                          = 113,
    A8P8                        = 114,
    B4G4R4A4_UNorm              = 115,
    P208                        = 130,
    V208                        = 131,
    V408                        = 132
}

impl DxgiFormat {
    // Get block size for the given BCn format
    fn get_block_size(&self) -> Option<u32> {
        match *self {
            DxgiFormat::BC1_Typeless |
            DxgiFormat::BC1_UNorm |
            DxgiFormat::BC1_UNorm_sRGB
            => Some(8),

            DxgiFormat::BC2_Typeless |
            DxgiFormat::BC2_UNorm |
            DxgiFormat::BC2_UNorm_sRGB |
            DxgiFormat::BC3_Typeless |
            DxgiFormat::BC3_UNorm |
            DxgiFormat::BC3_UNorm_sRGB
            => Some(16),

            DxgiFormat::BC4_Typeless |
            DxgiFormat::BC4_UNorm |
            DxgiFormat::BC4_SNorm
            => Some(8),

            DxgiFormat::BC5_Typeless |
            DxgiFormat::BC5_UNorm |
            DxgiFormat::BC5_SNorm |
            DxgiFormat::BC6H_Typeless |
            DxgiFormat::BC6H_UF16 |
            DxgiFormat::BC6H_SF16 |
            DxgiFormat::BC7_Typeless |
            DxgiFormat::BC7_UNorm |
            DxgiFormat::BC7_UNorm_sRGB
            => Some(16),

            _ => None,
        }
    }
}

// Texture material kind for bimage enum
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum TextureMaterialKind {
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

// DDS texture formats used by bimage
#[derive(Copy, Clone)]
#[allow(dead_code)]
enum TextureFormat {
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

// BIM header
#[derive(Copy, Clone)]
struct BIMHeader {
    signature: [u8; 3],
    version: u8,
    texture_type: i32,
    texture_material_kind: i32,
    pixel_width: i32,
    pixel_height: i32,
    depth: i32,
    mip_count: i32,
    mip_level: i64,
    unk_float_1: f32,
    bool_is_environment_map: u8,
    texture_format: i32,
    always_7: i32,
    null_padding: i32,
    atlas_padding: i16,
    bool_is_streamed: u8,
    unk_bool: u8,
    bool_no_mips: u8,
    bool_fft_bloom: u8,
    streamdb_mip_count: i32
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
    fn to_bytes(&self) -> [u8; 63] {
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
#[derive(Copy, Clone)]
struct BIMMipMap {
    mip_level: i64,
    mip_pixel_width: i32,
    mip_pixel_height: i32,
    unknown_flag: i32,
    decompressed_size: i32,
    flag_is_compressed: i32,
    compressed_size: i32,
    cumulative_size_streamdb: i32
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
    fn to_bytes(&self) -> [u8; 36] {
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

// Get size of given mipmap
#[inline(always)]
fn get_mipmap_size(width: u32, height: u32, format: DxgiFormat) -> Option<u32> {
    Some(max(1, (width + 3) / 4) * max(1, (height + 3) / 4) * format.get_block_size()?)
}

// Insert a slice at a specific location in a vec
// From https://gist.github.com/frozolotl/22a051baa5153b92e0b0207ad462ec12
pub fn insert_slice_at<T: Copy>(vec: &mut Vec<T>, index: usize, slice: &[T]) {
    unsafe {
        assert!(index <= vec.len());
        vec.reserve(slice.len());
        let insert_ptr = vec.as_mut_ptr().offset(index as isize);
        std::ptr::copy(insert_ptr, insert_ptr.offset(slice.len() as isize), vec.len() - index);
        std::ptr::copy_nonoverlapping(slice.as_ptr(), insert_ptr, slice.len());
        vec.set_len(vec.len() + slice.len());
    }
}

// Get equivalent BIM format from DXGI format
fn dxgi_to_bim_format(format: DxgiFormat) -> Result<TextureFormat, String> {
    match format {
        DxgiFormat::BC1_UNorm => Ok(TextureFormat::FmtBc1Srgb),
        DxgiFormat::BC3_UNorm => Ok(TextureFormat::FmtBc3),
        DxgiFormat::BC4_UNorm => Ok(TextureFormat::FmtBc4),
        DxgiFormat::BC5_UNorm => Ok(TextureFormat::FmtBc5),
        DxgiFormat::BC6H_UF16 => Ok(TextureFormat::FmtBc6hUf16),
        DxgiFormat::BC7_UNorm => Ok(TextureFormat::FmtBc7),
        _ => {
            return Err("Unsupported target BCn format".into());
        }
    }
}

// Get texture material kind
fn get_texture_material_kind(mut file_name: String, format: DxgiFormat) -> TextureMaterialKind {
    let material_kind: TextureMaterialKind;

    if DxgiFormat::BC7_UNorm != format && DxgiFormat::BC3_UNorm != format  {
        // Strip extensions and $ properties
        let stripped_dollar_section = file_name.split('$').next().unwrap();
        file_name = stripped_dollar_section.split('.').next().unwrap().to_string();
    }

    // Get material kind from filename
    if file_name.ends_with("_n") || file_name.ends_with("_Normal") {
        material_kind = TextureMaterialKind::TmkNormal;
    }
    else if file_name.ends_with("_s") {
        material_kind = TextureMaterialKind::TmkSpecular;
    }
    else if file_name.ends_with("_g") {
        material_kind = TextureMaterialKind::TmkSmoothness;
    }
    else if file_name.ends_with("_e") {
        material_kind = TextureMaterialKind::TmkBloommask;
    }
    else if file_name.ends_with("_h") {
        material_kind = TextureMaterialKind::TmkHeightmap;
    }
    else if file_name.ends_with("_sss") {
        material_kind = TextureMaterialKind::TmkSssmask;
    }
    else if file_name.contains("mtlkind=ui") {
        material_kind = TextureMaterialKind::TmkUi;
    }
    else if file_name.contains("mtlkind=decalnormal") {
        material_kind = TextureMaterialKind::TmkDecalnormal;
    }
    else if file_name.contains("mtlkind=decalalbedo") {
        material_kind = TextureMaterialKind::TmkDecalalbedo;
    }
    else if file_name.contains("mtlkind=decalspecular") {
        material_kind = TextureMaterialKind::TmkDecalspecular;
    }
    else if file_name.contains("mtlkind=particle") {
        material_kind = TextureMaterialKind::TmkParticle;
    }
    else if DxgiFormat::BC1_UNorm == format {
        material_kind = TextureMaterialKind::TmkAlbedo;
    }
    else {
        material_kind = TextureMaterialKind::TmkNone;
    }

    material_kind
}

// Compress data with oodle
fn oodle_compress(mut vec: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create output byte vec
    let mut comp_len = vec.len() + 274 * ((vec.len() + 0x3FFFF) / 0x40000);
    let mut comp_vec = vec![0_u8; comp_len + 16];

    // Add magic and decompressed size
    comp_vec[0..8].copy_from_slice(&[0x44, 0x49, 0x56, 0x49, 0x4E, 0x49, 0x54, 0x59]);
    comp_vec[8..16].copy_from_slice(&(vec.len() as u64).to_le_bytes());

    // Compress using oodle
    unsafe {
        comp_len = OodleLZ_Compress(8, vec.as_mut_ptr() as *mut c_void, vec.len() as i32,
                                    comp_vec.as_mut_ptr().add(16) as *mut c_void, 4,
                                    null(), null(), null(), null_mut(), 0) as usize;
    }

    if comp_len <= 0 {
        return Err("Failed to compress texture using oodle".into());
    }

    // Cut off unused bytes
    comp_vec.truncate(comp_len + 16);

    Ok(comp_vec)
}

// Compress texture into dds with mipmaps (no header)
fn convert_to_bimage(src_img: DynamicImage, file_name: String, format: DxgiFormat, compress: bool) -> Result<Vec<u8>, Box<dyn Error>> {
    // Get width and height
    let (width, height) = src_img.dimensions();

    // Get mipmap count
    let mipmap_count = 1 + f64::from(max(width, height)).log2().floor() as u32;

    // BIM bytes
    let mut bim: Vec<u8> = Vec::new();

    // Create BIM header and append it to bim
    bim.extend_from_slice(&BIMHeader {
        pixel_width: width as i32,
        pixel_height: height as i32,
        mip_count: mipmap_count as i32,
        texture_format: dxgi_to_bim_format(format)? as i32,
        texture_material_kind: get_texture_material_kind(file_name, format) as i32,
        ..Default::default()
    }.to_bytes());

    // Create all mipmaps
    let mut handles = Vec::new();
    let src_img_arc = Arc::new(src_img);

    for i in 0..mipmap_count {
        let img = Arc::clone(&src_img_arc);

        let handle = thread::spawn(move || {
            let power = 2_u32.pow(i);

            // Get the mip's width and height
            let mut mip_width = width / power;
            let mut mip_height = height / power;

            // Make sure they're not 0
            if mip_width == 0 {
                mip_width = 1;
            }

            if mip_height == 0 {
                mip_height = 1;
            }

            // Get resized image
            let mip_img = img.resize_exact(mip_width, mip_height, FilterType::Triangle);
            let mut mip_img_bytes = mip_img.as_bytes().to_vec();

            // Get division remainder
            let width_missing = 4 - mip_width % 4;
            let height_missing = 4 - mip_height % 4;

            // Add horizontal padding bytes
            if width_missing != 4 {
                let new_mip_width = mip_width + width_missing;
                let stride = new_mip_width as usize * 4;

                // Iterate through rows
                for mut i in (0..stride * mip_height as usize).step_by(stride) {
                    i = i + mip_width as usize * 4;

                    // Repeat the last pixel
                    let mut last_pixel = vec![0_u8; width_missing as usize * 4];

                    for j in 0..width_missing as usize {
                        last_pixel[j * 4..j * 4 + 4].copy_from_slice(&mip_img_bytes[i - 4..i]);
                    }

                    insert_slice_at(&mut mip_img_bytes, i, &last_pixel);
                }

                mip_width = new_mip_width;
            }

            // Add vertical padding bytes
            if height_missing != 4 {
                // Get last row of pixels
                let mut last_row = vec![0_u8; mip_width as usize * 4];
                let size = mip_img_bytes.len();
                last_row.copy_from_slice(&mip_img_bytes[size - mip_width as usize * 4..size]);

                // Duplicate last row
                for i in 0..height_missing as usize {
                    insert_slice_at(&mut mip_img_bytes, size + mip_width as usize * 4 * i, &last_row);
                }

                mip_height = mip_height + height_missing;
            }

            // Construct surface
            let surface = RgbaSurface {
                width: mip_width,
                height: mip_height,
                stride: mip_width * 4,
                data: &mip_img_bytes
            };

            // Init bc7 encoder
            unsafe {
                bc7e::bc7e_compress_block_init();
            }

            // Compress into bcn format
            let mip_size = get_mipmap_size(mip_width, mip_height, format).unwrap();

            let mip_bytes = match format {
                DxgiFormat::BC1_UNorm => bc1::compress_blocks(&surface),
                DxgiFormat::BC3_UNorm => bc3::compress_blocks(&surface),
                DxgiFormat::BC4_UNorm => bc4::compress_blocks(&surface),
                DxgiFormat::BC5_UNorm => bc5::compress_blocks(&surface),
                DxgiFormat::BC6H_UF16 => bc6h::compress_blocks(&bc6h::very_fast_settings(), &surface),
                DxgiFormat::BC7_UNorm => {
                    // Compress options
                    let mut p = bc7e::bc7e_compress_block_params {
                        m_max_partitions_mode: [0; 8],
                        m_weights: [0; 4],
                        m_uber_level: 0,
                        m_refinement_passes: 0,
                        m_mode4_rotation_mask: 0,
                        m_mode4_index_mask: 0,
                        m_mode5_rotation_mask: 0,
                        m_uber1_mask: 0,
                        m_perceptual: false,
                        m_pbit_search: false,
                        m_mode6_only: false,
                        m_unused0: false,
                        m_opaque_settings: bc7e::_anon0_ {
                            m_max_mode13_partitions_to_try: 0,
                            m_max_mode0_partitions_to_try: 0,
                            m_max_mode2_partitions_to_try: 0,
                            m_use_mode: [false; 7],
                            m_unused1: false,
                        },
                        m_alpha_settings: bc7e::_anon1_ {
                            m_max_mode7_partitions_to_try: 0,
                            m_mode67_error_weight_mul: [0; 4],
                            m_use_mode4: false,
                            m_use_mode5: false,
                            m_use_mode6: false,
                            m_use_mode7: false,
                            m_use_mode4_rotation: false,
                            m_use_mode5_rotation: false,
                            m_unused2: false,
                            m_unused3: false,
                        }
                    };

                    unsafe {
                        bc7e::bc7e_compress_block_params_init_ultrafast(&mut p, true);
                    }

                    // Compress blocks 64 per 64
                    let blocks_x = (mip_width / 4) as usize;
	                let blocks_y = (mip_height / 4) as usize;
                    let mut packed_blocks = vec![0_u8; blocks_x * blocks_y * 16];

                    for by in 0..blocks_y {
                        let n = 64;

                        for bx in (0..blocks_x).step_by(n) {
                            let num_blocks_to_process = min(blocks_x - bx, n);

                            let mut pixels = vec![0_u8; 64 * n];

                            // Get blocks
                            for b in 0..num_blocks_to_process {
                                for y in 0_usize..4_usize {
                                    let coord_x = (bx + b) * 16;
                                    let coord_y = by * 16 + y * 4;
                                    let start = coord_x + mip_width as usize * coord_y;
                                    pixels[b * 64 + y * 16..b * 64 + y * 16 + 16].copy_from_slice(&surface.data[start..start + 16]);
                                }
                            }

                            // Compress using BC7
                            unsafe {
                                bc7e::bc7e_compress_blocks(num_blocks_to_process as u32, packed_blocks.as_mut_ptr().offset((bx + by * blocks_x) as isize * 16) as *mut u64, pixels.as_mut_ptr() as *mut u32, &p);
                            }
                        }
                    }

                    packed_blocks
                },
                _ => vec![0]
            };

            let bim_mip = BIMMipMap {
                mip_level: i as i64,
                mip_pixel_width: mip_img.dimensions().0 as i32,
                mip_pixel_height: mip_img.dimensions().1 as i32,
                decompressed_size: mip_size as i32,
                compressed_size: mip_size as i32,
                ..Default::default()
            };

            (mip_bytes, bim_mip.to_bytes(), mip_size as i32)
        });

        handles.push(handle);
    }

    let mut texture = Vec::new();
    let mut bim_mip_cumulative_size: i32 = 0;

    // Join all threads
    for handle in handles {
        let mut mipmap = handle.join().unwrap();

        // Append texture bytes
        texture.append(&mut mipmap.0);

        // Change cumulative size
        let mut bim_mip = mipmap.1;
        bim_mip[32..36].copy_from_slice(&bim_mip_cumulative_size.to_le_bytes());
        bim_mip_cumulative_size += mipmap.2;

        // Append mip bytes
        bim.extend_from_slice(&bim_mip);
    }

    // Change last bytes
    if format == DxgiFormat::BC5_UNorm {
        texture.truncate(texture.len() - 16);
        texture.extend_from_slice(&[0x87, 0x86, 0x49, 0x92, 0x24, 0x49, 0x92, 0x24, 0x86, 0x85, 0x49, 0x92, 0x24, 0x49, 0x92, 0x2]);
    }
    else {
        texture.truncate(texture.len() - 4);
        texture.extend_from_slice(&[0_u8, 0_u8, 0_u8, 0_u8]);
    }

    // Add dds bytes to bim
    bim.append(&mut texture);

    // Compress bim texture with oodle
    let comp_bim = match compress {
        true => oodle_compress(bim)?,
        false => bim
    };

    Ok(comp_bim)
}

// Load textures, convert them to bim, and compress them
fn handle_textures(paths: Vec<String>) -> i32 {
    // Thread handles
    let mut handles = Vec::new();

    // Mutex for thread handling
    let mtx = Arc::new(Mutex::new(0));

    // Iterate through args
    for path in paths {
        let mtx = Arc::clone(&mtx);

        let handle = thread::spawn(move || {
            let mut output = String::new();
            write!(&mut output, "\n").unwrap();

            // Get texture's format and stripped filename
            let file_path = Path::new(&path);
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            let stripped_file_name = file_name.split('$').next().unwrap().split('.').next().unwrap();

            write!(&mut output, "Converting '{}'...\n", file_name).unwrap();

            // Check if given path exists and is a file
            if !file_path.is_file() {
                write!(&mut output, "ERROR: '{}' was not found.\n", path).unwrap();
                return (output, false);
            }

            // Get target format
            let mut format = DxgiFormat::BC1_UNorm;

            if file_name.contains("$bc7") {
                format = DxgiFormat::BC7_UNorm;
            }
            else if file_name.contains("$bc3") {
                format = DxgiFormat::BC3_UNorm;
            }
            else if stripped_file_name.ends_with("_n") || stripped_file_name.ends_with("_Normal") {
                format = DxgiFormat::BC5_UNorm;
            }

            // Load image
            let src_reader = match Reader::open(file_path).and_then(|r| r.with_guessed_format()) {
                Ok(reader) => reader,
                Err(e) => {
                    write!(&mut output, "ERROR: Failed to load '{}': {}\n", path, e).unwrap();
                    return (output, false);
                }
            };

            let src_img = match src_reader.decode() {
                Ok(img) => DynamicImage::ImageRgba8(img.into_rgba8()),
                Err(e) => {
                    write!(&mut output, "ERROR: Failed to load '{}': {}\n", path, e).unwrap();
                    return (output, false);
                }
            };

            // Check if image should be compressed
            let compress = var("AUTOHECKIN_SKIP_COMPRESSION").is_err();

            // Convert image to bimage format
            let bim_bytes = match convert_to_bimage(src_img, file_name.clone(), format, compress) {
                Ok(vec) => vec,
                Err(e) => {
                    write!(&mut output, "ERROR: Failed to convert '{}' to DDS: {}\n", path, e).unwrap();
                    return (output, false);
                }
            };

            // Get output filename
            let new_extension: &str;

            if !file_name.contains('$') {
                if file_path.file_stem().unwrap().to_str().unwrap().ends_with(".png") {
                    new_extension = "png";
                }
                else {
                    new_extension = "tga"
                }
            }
            else {
                let curr_extension = Path::new(&file_name).extension().unwrap().to_str().unwrap();

                if curr_extension.contains('$') {
                    new_extension = curr_extension;
                }
                else {
                    new_extension = "";
                }
            }

            let new_file_path: PathBuf;

            // Prevent overwriting
            if file_path.with_extension(new_extension).exists() {
                // Append -i, with the least possible number
                let trunc_path = file_path.with_extension("").to_str().unwrap().to_string();
                let mut i = 2_u32;

                let dot = match new_extension {
                    "" => "",
                    _ => "."
                };

                while Path::new(&(trunc_path.clone() + "-" + &i.to_string() + dot + new_extension)).exists() {
                    i += 1;
                }

                new_file_path = PathBuf::from(&(trunc_path + "-" + &i.to_string() + dot + new_extension));
            }
            else {
                new_file_path = file_path.with_extension(new_extension);
            }

            // Get filename
            let new_file_name = new_file_path.file_name().unwrap().to_str().unwrap();

            // Lock mtx
            let mtx = mtx.lock().unwrap();

            // Write output file
            let mut output_file = match File::create(new_file_path.to_str().unwrap()) {
                Ok(f) => f,
                Err(e) => {
                    write!(&mut output, "ERROR: Failed to create output file: {}\n", e).unwrap();
                    return (output, false);
                }
            };

            match output_file.write(&bim_bytes) {
                Ok(_) => (),
                Err(e) => {
                    write!(&mut output, "ERROR: Failed to write to output file: {}\n", e).unwrap();
                    return (output, false);
                }
            }

            // Remove mtx lock
            drop(mtx);

            write!(&mut output, "Successfully converted '{}' into '{}'.\n", file_name, new_file_name).unwrap();

            (output, true)
        });

        handles.push(handle);
    }

    let mut failures = 0;

    // Join threads
    for handle in handles {
        let output = handle.join().unwrap();
        print!("{}", output.0);

        // Check if conversion succeeded
        if !output.1 {
            failures += 1;
        }
    }

    failures
}

// Simulates the 'pause' system command on Windows
#[cfg(target_os = "windows")]
fn press_any_key() {
    // Get process count
    let process_count: u32;

    unsafe {
        let mut buffer = [0_u32, 1];
        process_count = GetConsoleProcessList(buffer.as_mut_ptr(), 1);
    }

    // If there's only one process, we're not running from terminal
    if process_count == 1 {
        println!("\nPress any key to exit...");
        let mut stdin = stdin();
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}

fn main() {
    // Print program name
    println!("Auto Heckin' Texture Converter Rust Rewrite by PowerBall253 :D");

    // Get args
    let mut args: Vec<String> = args().collect();
    let program = args[0].clone();
    args.remove(0);

    // Display help if no arguments are provided
    if args.len() == 0 {
        println!("\nUsage:");
        println!("{} [texture1] [texture2] [...]\n", program);
        println!("Alternatively, drag files onto this executable.");

        // Exit
        #[cfg(target_os = "windows")]
        press_any_key();

        return;
    }

    // Convert textures
    let failures = handle_textures(args);
    println!("\nDone.");

    // Exit
    #[cfg(target_os = "windows")]
    press_any_key();

    exit(failures);
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mipmap_size() {
        assert_eq!(get_mipmap_size(1024, 2048, DxgiFormat::BC1_UNorm), Some(1048576));
        assert_eq!(get_mipmap_size(720, 560, DxgiFormat::BC3_UNorm), Some(403200));
        assert_eq!(get_mipmap_size(576, 254, DxgiFormat::BC5_UNorm), Some(147456));
        assert_eq!(get_mipmap_size(2946, 822, DxgiFormat::BC7_UNorm), Some(2429152));
    }

    #[test]
    fn test_get_texture_material_kind() {
        assert_eq!(get_texture_material_kind("symbols_arrow_03a_local.tga$bc7$streamed$mtlkind=decalnormal.png".into(),
            DxgiFormat::BC7_UNorm), TextureMaterialKind::TmkDecalnormal);
        assert_eq!(get_texture_material_kind("glass_textured_orange_n.tga$bc5$streamed.png".into(), DxgiFormat::BC5_UNorm),
            TextureMaterialKind::TmkNormal);
        assert_eq!(get_texture_material_kind("hud_demon_icon_ability_quantumorb.tga$bc3$streamed$mtlkind=particle.png".into(),
            DxgiFormat::BC3_UNorm), TextureMaterialKind::TmkParticle);
        assert_eq!(get_texture_material_kind("test.png".into(), DxgiFormat::BC1_UNorm), TextureMaterialKind::TmkAlbedo);
    }

    #[test]
    fn test_oodle_compress() {
        let test_bytes = vec![0x74, 0x65, 0x73, 0x74, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E];
        let comp_test_bytes = vec![68, 73, 86, 73, 78, 73, 84, 89, 15, 0, 0, 0, 0, 0, 0, 0, 204, 6, 116, 101, 115, 116,
            99, 111, 109, 112, 114, 101, 115, 115, 105, 111, 110];
        assert_eq!(oodle_compress(test_bytes).unwrap(), comp_test_bytes);
    }

    fn helper_convert_to_bimage(file_path: &str, format: DxgiFormat, expected_bim_bytes: [u8; 63]) {
        // Get file name
        let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();

        // Load image
        let src_img = match image::open(file_path) {
            Ok(img) => DynamicImage::ImageRgba8(img.into_rgba8()),
            Err(_) => panic!("Could not load image")
        };

        // Convert image to bimage format
        let bim_bytes = match convert_to_bimage(src_img, file_name.into(), format, false) {
            Ok(vec) => vec,
            Err(_) => panic!("Failed to convert to bimage")
        };

        // Compare to expected result
        assert_eq!(bim_bytes[0..63], expected_bim_bytes);
    }

    #[test]
    fn test_convert_to_bimage_1() {
        let file_path = "./test/symbols_arrow_03a_local.tga$bc7$streamed$mtlkind=decalnormal.png";
        let format = DxgiFormat::BC7_UNorm;
        let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 11, 0, 0, 0, 128, 0, 0, 0, 128,
        0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 23, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper_convert_to_bimage(file_path, format, bim_bytes);
    }

    #[test]
    fn test_convert_to_bimage_2() {
        let file_path = "./test/glass_textured_orange_n.tga$bc5$streamed.png";
        let format = DxgiFormat::BC5_UNorm;
        let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 3, 0, 0, 0, 128, 0, 0, 0, 128,
        0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 25, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper_convert_to_bimage(file_path, format, bim_bytes);
    }

    #[test]
    fn test_convert_to_bimage_3() {
        let file_path = "./test/hud_demon_icon_ability_quantumorb.tga$bc3$streamed.png";
        let format = DxgiFormat::BC3_UNorm;
        let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 11, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper_convert_to_bimage(file_path, format, bim_bytes);
    }

    #[test]
    fn test_convert_to_bimage_4() {
        let file_path = "./test/test.png";
        let format = DxgiFormat::BC1_UNorm;
        let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 1, 0, 0, 0, 0, 8, 0, 0, 0, 8,
        0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 33, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper_convert_to_bimage(file_path, format, bim_bytes);
    }
}
