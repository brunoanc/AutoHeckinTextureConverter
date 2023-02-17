extern crate image;
extern crate texpresso;

mod bc7e;
mod bim;
mod utils;
mod ooz;

use std::{env, process, cmp, thread};
use std::fs::File;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc};
use bc7e::CompressBlockParams;
use bim::{TextureMaterialKind, TextureFormat, BIMHeader, BIMMipMap};
use image::{DynamicImage, GenericImageView, imageops::FilterType, io::Reader};
use texpresso::{Algorithm, Params, COLOUR_WEIGHTS_UNIFORM};

// Compress data with oodle's kraken
fn kraken_compress(mut vec: Vec<u8>) -> Result<Vec<u8>, String> {
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

    if comp_len <= 0 {
        return Err("Failed to compress texture using ooz".into());
    }

    // Cut off unused bytes
    comp_vec.truncate(comp_len as usize + 16);

    Ok(comp_vec)
}

// Compress texture into dds with mipmaps (no header)
fn convert_to_bimage(src_img: DynamicImage, file_name: String, stripped_file_name: String,
    format: TextureFormat, compress: bool, compress_params: CompressBlockParams) -> Result<Vec<u8>, String> {
    // Get width and height
    let (width, height) = src_img.dimensions();

    // Get mipmap count
    let mipmap_count = 1 + f64::from(cmp::max(width, height)).log2().floor() as u32;

    // BIM bytes
    let mut bim: Vec<u8> = Vec::new();

    // Create BIM header and append it to bim
    bim.extend_from_slice(&BIMHeader {
        pixel_width: width as i32,
        pixel_height: height as i32,
        mip_count: mipmap_count as i32,
        texture_format: format as i32,
        texture_material_kind: TextureMaterialKind::from_filename(file_name, stripped_file_name, format) as i32,
        ..Default::default()
    }.to_bytes());

    // Create all mipmaps
    let mut handles = Vec::new();
    let src_img_arc = Arc::new(src_img);

    for i in 0..mipmap_count {
        let img = src_img_arc.clone();

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
                    i += mip_width as usize * 4;

                    // Repeat the last pixel
                    let mut last_pixel = vec![0_u8; width_missing as usize * 4];

                    for j in 0..width_missing as usize {
                        last_pixel[j * 4..j * 4 + 4].copy_from_slice(&mip_img_bytes[i - 4..i]);
                    }

                    utils::insert_slice_at(&mut mip_img_bytes, i, &last_pixel);
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
                    utils::insert_slice_at(&mut mip_img_bytes, size + mip_width as usize * 4 * i, &last_row);
                }

                mip_height += height_missing;
            }

            // Compress into bcn format
            let mip_size = format.calculate_mipmap_size(mip_width, mip_height).unwrap();

            let mip_bytes = if format == TextureFormat::FmtBc7 {
                // Compress blocks 64 per 64
                let blocks_x = (mip_width / 4) as usize;
                let blocks_y = (mip_height / 4) as usize;
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
                                let start = coord_x + mip_width as usize * coord_y;
                                pixels[b * 64 + y * 16..b * 64 + y * 16 + 16].copy_from_slice(&mip_img_bytes[start..start + 16]);
                            }
                        }

                        // Compress using BC7
                        unsafe {
                            bc7e::compress_blocks(num_blocks_to_process as u32,
                                packed_blocks.as_mut_ptr().add((bx + by * blocks_x) * 16) as *mut u64,
                                pixels.as_mut_ptr() as *mut u32, &compress_params);
                        }
                    }
                }

                packed_blocks
            }
            else {
                // Compression parameters
                let params = Params {
                    algorithm: Algorithm::RangeFit,
                    weights: COLOUR_WEIGHTS_UNIFORM,
                    weigh_colour_by_alpha: false
                };

                // Compress to BCx format
                let tex_format = format.as_texpresso_format().unwrap();
                let mut compressed = vec![0u8; tex_format.compressed_size(mip_width as usize, mip_height as usize)];
                tex_format.compress(&mip_img_bytes, mip_width as usize, mip_height as usize, params, &mut compressed);

                compressed
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
    if format == TextureFormat::FmtBc5 {
        texture.truncate(texture.len() - 16);
        texture.extend_from_slice(&[0x87, 0x86, 0x49, 0x92, 0x24, 0x49, 0x92, 0x24, 0x86, 0x85, 0x49, 0x92, 0x24, 0x49, 0x92, 0x2]);
    }
    else {
        texture.truncate(texture.len() - 4);
        texture.extend_from_slice(&[0_u8, 0_u8, 0_u8, 0_u8]);
    }

    // Add dds bytes to bim
    bim.append(&mut texture);

    // Compress bim texture with kraken
    let comp_bim = if compress {
        kraken_compress(bim)?
    }
    else {
        bim
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
        let mtx = mtx.clone();

        let handle = thread::spawn(move || {
            let mut output = String::default();
            writeln!(&mut output).unwrap();

            // Get texture's format and stripped filename
            let file_path = Path::new(&path);
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
            let stripped_file_name = file_name.split('$').next().unwrap().split('.').next().unwrap().to_owned();

            writeln!(&mut output, "Converting '{}'...", file_name).unwrap();

            // Check if given path exists and is a file
            if !file_path.is_file() {
                writeln!(&mut output, "ERROR: '{}' was not found.", path).unwrap();
                return (output, false);
            }

            // Get target format
            let mut format = TextureFormat::FmtBc1Srgb;

            if file_name.contains("$bc7") {
                format = TextureFormat::FmtBc7;
            }
            else if file_name.contains("$bc3") {
                format = TextureFormat::FmtBc3;
            }
            else if file_name.contains("$bc4") {
                format = TextureFormat::FmtBc4;
            }
            else if stripped_file_name.ends_with("_n") || stripped_file_name.ends_with("_Normal") {
                format = TextureFormat::FmtBc5;
            }

            // Load image
            let src_reader = match Reader::open(file_path).and_then(|r| r.with_guessed_format()) {
                Ok(reader) => reader,
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to load '{}': {}", path, e).unwrap();
                    return (output, false);
                }
            };

            let src_img = match src_reader.decode() {
                Ok(img) => DynamicImage::ImageRgba8(img.into_rgba8()),
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to load '{}': {}", path, e).unwrap();
                    return (output, false);
                }
            };

            // Init bc7 compress options
            let mut compress_params = CompressBlockParams::default();

            unsafe {
                bc7e::compress_block_params_init_ultrafast(&mut compress_params, true);
            }

            // Check if image should be compressed
            let compress = env::var("AUTOHECKIN_SKIP_COMPRESSION").is_err();

            // Convert image to bimage format
            let bim_bytes = match convert_to_bimage(src_img, file_name.clone(), stripped_file_name, format, compress, compress_params) {
                Ok(vec) => vec,
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to convert '{}' to DDS: {}", path, e).unwrap();
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

            // Lock mtx
            let mtx = mtx.lock().unwrap();

            // Write output file
            let mut output_file = match File::create(new_file_path.to_str().unwrap()) {
                Ok(f) => f,
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to create output file: {}", e).unwrap();
                    return (output, false);
                }
            };

            match output_file.write(&bim_bytes) {
                Ok(_) => (),
                Err(e) => {
                    writeln!(&mut output, "ERROR: Failed to write to output file: {}", e).unwrap();
                    return (output, false);
                }
            }

            // Remove mtx lock
            drop(mtx);

            writeln!(&mut output, "Successfully converted '{}' into '{}'.", file_name, new_file_name).unwrap();

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

fn main() {
    // Print program name
    println!("Auto Heckin' Texture Converter Rust Rewrite by PowerBall253 :D");

    // Get args
    let mut args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    args.remove(0);

    // Display help if no arguments are provided
    if args.is_empty() {
        println!("\nUsage:");
        println!("{} [texture1] [texture2] [...]\n", program);
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
    let failures = handle_textures(args);
    println!("\nDone.");

    // Exit
    #[cfg(target_os = "windows")]
    utils::press_any_key();

    process::exit(failures);
}

// Tests
#[cfg(test)]
mod test;
