mod bc7e;
mod bim;
mod ooz;
mod utils;

use std::{
    cmp, env,
    fmt::Write as _,
    fs::File,
    io::Write as _,
    mem,
    num::NonZeroU32,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
    thread
};

use bc7e::CompressBlockParams;
use bim::{BIMHeader, BIMMipMap, TextureFormat, TextureMaterialKind};
use fast_image_resize::{FilterType, Image, MulDiv, PixelType, ResizeAlg, Resizer};
use image::{io::Reader as ImageReader, ImageFormat, RgbaImage};
use texpresso::{Algorithm, Params};

// Compress data with oodle's kraken
fn kraken_compress(vec: &mut Vec<u8>) -> Result<Vec<u8>, String> {
    // Create output byte vec
    let mut comp_len = (vec.len() + 274 * ((vec.len() + 0x3FFFF) / 0x40000)) as i32;
    let mut comp_vec = vec![0_u8; comp_len as usize + 16];

    // Add magic and decompressed size
    comp_vec[0..8].copy_from_slice(&[0x44, 0x49, 0x56, 0x49, 0x4E, 0x49, 0x54, 0x59]);
    comp_vec[8..16].copy_from_slice(&(vec.len() as u64).to_le_bytes());

    // Compress using ooz
    unsafe {
        comp_len = ooz::kraken_compress(vec.as_mut_ptr(), vec.len(), comp_vec.as_mut_ptr().add(16), 4);
    }

    if comp_len > 0 {
        // Cut off unused bytes
        comp_vec.truncate(comp_len as usize + 16);
        Ok(comp_vec)
    }
    else {
        Err("Failed to compress texture using ooz".into())
    }
}

// Compress into BCn format
fn compress_bcn(format: TextureFormat, image: &[u8], width: usize, height: usize) -> Vec<u8> {
    match format {
        TextureFormat::FmtAlpha => {
            // Extract alpha byte
            image.iter().skip(3).step_by(4).copied().collect()
        },
        TextureFormat::FmtBc7 => {
            // Compress blocks 64 per 64
            let blocks_x = width / 4;
            let blocks_y = height / 4;
            let mut packed_blocks = vec![0_u8; blocks_x * blocks_y * 16];

            for by in 0..blocks_y {
                let n = 64;

                for bx in (0..blocks_x).step_by(n) {
                    let num_blocks_to_process = cmp::min(blocks_x - bx, n);

                    let mut pixels = vec![0_u8; 64 * n];

                    // Get blocks
                    for b in 0..num_blocks_to_process {
                        for y in 0_usize..4_usize {
                            let coord_x = (bx + b) * 16;
                            let coord_y = by * 16 + y * 4;
                            let start = coord_x + width * coord_y;
                            pixels[b * 64 + y * 16..b * 64 + y * 16 + 16]
                                .copy_from_slice(&image[start..start + 16]);
                        }
                    }

                    // Compress to BC7 using bc7e
                    static COMPRESS_PARAMS: CompressBlockParams = CompressBlockParams::ultrafast();

                    unsafe {
                        bc7e::compress_blocks(
                            num_blocks_to_process as u32,
                            packed_blocks.as_mut_ptr().add((bx + by * blocks_x) * 16) as *mut u64,
                            pixels.as_mut_ptr() as *mut u32,
                            &COMPRESS_PARAMS
                        );
                    }
                }
            }

            packed_blocks
        },
        _ => {
            // Compression parameters
            static COMPRESS_PARAMS: Params = Params {
                algorithm: Algorithm::RangeFit,
                weights: texpresso::COLOUR_WEIGHTS_PERCEPTUAL,
                weigh_colour_by_alpha: false
            };

            // Compress using texpresso
            let tex_format = format.as_texpresso_format().unwrap();
            let mut compressed = vec![0u8; tex_format.compressed_size(width, height)];
            tex_format.compress(image, width, height, COMPRESS_PARAMS, &mut compressed);

            compressed
        }
    }
}

// Convert texture to bimage format used by the game
fn convert_to_bimage(
    src_img: RgbaImage, file_name: String, stripped_file_name: String, format: TextureFormat, compress: bool
) -> Result<Vec<u8>, String> {
    // Get width and height
    let (width, height) = src_img.dimensions();

    // Get mipmap count
    let mipmap_count = 1 + f64::from(cmp::max(width, height)).log2() as u32;

    // Get upper limit for total texture size (with mipmaps)
    // Derived from sumation of mipmap approx
    let four_power = 4_u32.pow(mipmap_count) as usize;
    let block_size = format.block_size().unwrap() as usize;
    let added_texture_approx =
        (4 * block_size * ((width as usize + 3) / 4) * ((height as usize + 3) / 4) * (four_power - 1))
            / (3 * four_power)
            + block_size * 3;

    // BIM bytes
    let mut texture = Vec::with_capacity(added_texture_approx);
    let mut bim = Vec::with_capacity(
        mem::size_of::<BIMHeader>()
            + mem::size_of::<BIMMipMap>() * mipmap_count as usize
            + added_texture_approx
    );

    // Create BIM header and append it to bim
    bim.extend_from_slice(
        &BIMHeader {
            pixel_width: width,
            pixel_height: height,
            mip_count: mipmap_count,
            texture_format: format as u32,
            texture_material_kind: TextureMaterialKind::from_filename(file_name, stripped_file_name, format)
                as u32,
            ..Default::default()
        }
        .to_bytes()
    );

    // Pointer to src_img bytes
    let mut src_img_buf = src_img.into_raw();

    // Create source container for resize
    let mut resize_src = Image::from_slice_u8(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
        src_img_buf.as_mut_slice(),
        PixelType::U8x4
    )
    .unwrap();

    // Multiply RGB by alpha (needed for resize algorithm)
    let alpha_mul_div = MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut resize_src.view_mut())
        .unwrap();

    let resize_src_arc = Arc::new(resize_src);
    let alpha_mul_div_arc = Arc::new(alpha_mul_div);

    thread::scope(|s| {
        // Create all mipmaps
        let mut handles = Vec::new();

        for i in 0..mipmap_count {
            let resize_src_clone = resize_src_arc.clone();
            let alpha_mul_div_clone = alpha_mul_div_arc.clone();

            let handle = s.spawn(move || {
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

                // Create dest container for resize
                let mut resize_dst = Image::new(
                    NonZeroU32::new(mip_width).unwrap(),
                    NonZeroU32::new(mip_height).unwrap(),
                    resize_src_clone.pixel_type()
                );

                // Resize using Box filter
                let mut resizer = Resizer::new(ResizeAlg::Convolution(FilterType::Box));
                resizer
                    .resize(&resize_src_clone.view(), &mut resize_dst.view_mut())
                    .unwrap();

                // Divide RGB by alpha
                alpha_mul_div_clone
                    .divide_alpha_inplace(&mut resize_dst.view_mut())
                    .unwrap();

                // Get resized bytes
                let mut mip_img_bytes = resize_dst.buffer().to_vec();

                // Get division remainder
                let width_missing = 4 - mip_width % 4;
                let height_missing = 4 - mip_height % 4;

                // Add horizontal padding bytes
                if width_missing != 4 {
                    let new_mip_width = mip_width + width_missing;
                    let stride = new_mip_width as usize * 4;

                    // Iterate through rows
                    for mut i in (0..stride * mip_height as usize).step_by(stride) {
                        i += mip_width as usize * 4;

                        // Repeat the last pixel
                        let mut last_pixel = vec![0_u8; width_missing as usize * 4];

                        for j in 0..width_missing as usize {
                            last_pixel[j * 4..j * 4 + 4].copy_from_slice(&mip_img_bytes[i - 4..i]);
                        }

                        mip_img_bytes.splice(i..i, last_pixel.iter().cloned());
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
                        let insert_index = size + mip_width as usize * 4 * i;
                        mip_img_bytes.splice(insert_index..insert_index, last_row.iter().cloned());
                    }

                    mip_height += height_missing;
                }

                // Compress to BCn format
                let mip_bytes = compress_bcn(format, &mip_img_bytes, mip_width as usize, mip_height as usize);

                // Create mip header
                let bim_mip = BIMMipMap {
                    mip_level: i as i64,
                    mip_pixel_width: mip_width,
                    mip_pixel_height: mip_height,
                    decompressed_size: mip_bytes.len() as u32,
                    compressed_size: mip_bytes.len() as u32,
                    ..Default::default()
                };

                (mip_bytes, bim_mip)
            });

            handles.push(handle);
        }

        let mut bim_mip_cumulative_size = 0_u32;

        // Join all threads
        for handle in handles {
            let mut mipmap = handle.join().unwrap();

            // Append texture bytes
            texture.append(&mut mipmap.0);

            // Change cumulative size
            let mut bim_mip = mipmap.1;
            bim_mip.cumulative_size_streamdb = bim_mip_cumulative_size;
            bim_mip_cumulative_size += bim_mip.compressed_size;

            // Append mip bytes
            bim.extend_from_slice(&bim_mip.to_bytes());
        }
    });

    // Change last bytes
    let texture_len = texture.len();

    if format == TextureFormat::FmtBc5 {
        texture[texture_len - 16..].clone_from_slice(&[
            0x87, 0x86, 0x49, 0x92, 0x24, 0x49, 0x92, 0x24, 0x86, 0x85, 0x49, 0x92, 0x24, 0x49, 0x92, 0x2
        ]);
    }
    else {
        texture[texture_len - 4..].clone_from_slice(&[0_u8, 0_u8, 0_u8, 0_u8]);
    }

    // Add dds bytes to bim
    bim.append(&mut texture);

    // Compress bim texture with kraken
    if compress {
        kraken_compress(&mut bim)
    }
    else {
        Ok(bim)
    }
}

// Load textures, convert them to bim, and compress them
fn handle_textures(paths: Vec<String>) -> u32 {
    let paths_len = paths.len() as u32;

    // Thread handles
    let mut handles = Vec::new();

    // Count successful conversions
    let counter = Arc::new(Mutex::new(0));

    // Iterate through args
    for path in paths {
        let counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut output = String::default();
            writeln!(&mut output).unwrap();

            // Get texture's format and stripped filename
            let file_path = Path::new(&path);
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
            let stripped_file_name = file_name
                .split('$')
                .next()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_owned();

            writeln!(&mut output, "Converting '{}'...", file_name).unwrap();

            // Check if given path exists and is a file
            if !file_path.is_file() {
                writeln!(&mut output, "ERROR: '{}' was not found.", path).unwrap();
                return output;
            }

            // Get target format
            let format = match () {
                _ if file_name.contains("$bc7") => TextureFormat::FmtBc7,
                _ if file_name.contains("$bc5") => TextureFormat::FmtBc5,
                _ if file_name.contains("$bc4") => TextureFormat::FmtBc4,
                _ if file_name.contains("$bc3") => TextureFormat::FmtBc3,
                _ if file_name.contains("$alpha") => TextureFormat::FmtAlpha,
                _ if stripped_file_name.ends_with("_n") => TextureFormat::FmtBc5,
                _ if stripped_file_name.ends_with("_Normal") => TextureFormat::FmtBc5,
                _ => TextureFormat::FmtBc1Srgb
            };

            // Load image
            let mut src_reader = match ImageReader::open(file_path) {
                Ok(reader) => reader,
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to load '{}': {}", path, e).unwrap();
                    return output;
                }
            };

            src_reader.set_format(ImageFormat::Png);

            let src_img = match src_reader.decode() {
                Ok(img) => img.into_rgba8(),
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to load '{}': {}", path, e).unwrap();
                    return output;
                }
            };

            // Check if image should be compressed
            let compress = env::var("AUTOHECKIN_SKIP_COMPRESSION").is_err();

            // Convert image to bimage format
            let bim_bytes =
                match convert_to_bimage(src_img, file_name.clone(), stripped_file_name, format, compress) {
                    Ok(vec) => vec,
                    Err(e) => {
                        writeln!(&mut output, "ERROR: Failed to convert '{}' to DDS: {}", path, e).unwrap();
                        return output;
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

            // Prevent overwriting
            let new_file_path = if file_path.with_extension(new_extension).exists() {
                // Append -i, with the least possible number
                let trunc_path = file_path.with_extension("").to_str().unwrap().to_owned();
                let mut i = 2_u32;

                let dot = match new_extension {
                    "" => "",
                    _ => "."
                };

                while Path::new(&(trunc_path.clone() + "-" + &i.to_string() + dot + new_extension)).exists() {
                    i += 1;
                }

                PathBuf::from(&(trunc_path + "-" + &i.to_string() + dot + new_extension))
            }
            else {
                file_path.with_extension(new_extension)
            };

            // Get filename
            let new_file_name = new_file_path.file_name().unwrap().to_str().unwrap();

            // Adquire lock
            let mut counter = counter.lock().unwrap();

            // Write output file
            let mut output_file = match File::create(new_file_path.to_str().unwrap()) {
                Ok(f) => f,
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to create output file: {}", e).unwrap();
                    return output;
                }
            };

            match output_file.write_all(&bim_bytes) {
                Ok(_) => (),
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to write to output file: {}", e).unwrap();
                    return output;
                }
            }

            writeln!(
                &mut output,
                "Successfully converted '{}' into '{}'.",
                file_name, new_file_name
            )
            .unwrap();
            *counter += 1;
            output
        });

        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        let output = handle.join().unwrap();
        print!("{}", output);
    }

    paths_len - Arc::try_unwrap(counter).unwrap().into_inner().unwrap()
}

fn main() {
    // Print program name
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Auto Heckin' Texture Converter v{} by PowerBall253 :)", VERSION);

    // Get args
    let mut args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    args.remove(0);

    // Display help if no arguments are provided
    if args.is_empty() {
        println!("\nUsage:");
        println!("{} [texture1.png] [texture2.png] [...]\n", program);
        println!("Alternatively, drag files onto this executable.");

        // Exit
        #[cfg(target_os = "windows")]
        utils::press_any_key();

        return;
    }

    // Init bc7 encoder
    unsafe {
        bc7e::compress_block_init();
    }

    // Convert textures
    let failures = handle_textures(args) as i32;
    println!("\nDone.");

    // Exit
    #[cfg(target_os = "windows")]
    utils::press_any_key();

    process::exit(failures);
}

// Tests
#[cfg(test)]
mod test;
