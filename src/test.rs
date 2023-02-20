use super::*;

#[test]
fn test_get_texture_material_kind() {
    assert_eq!(
        TextureMaterialKind::from_filename(
            "symbols_arrow_03a_local.tga$bc7$streamed$mtlkind=decalnormal.png".into(),
            "symbols_arrow_03a_local".into(),
            TextureFormat::FmtBc7
        ),
        TextureMaterialKind::TmkDecalnormal
    );
    assert_eq!(
        TextureMaterialKind::from_filename(
            "glass_textured_orange_n.tga$bc5$streamed.png".into(),
            "glass_textured_orange_n".into(),
            TextureFormat::FmtBc5
        ),
        TextureMaterialKind::TmkNormal
    );
    assert_eq!(
        TextureMaterialKind::from_filename(
            "asphalt_g.tga$bc4$streamed$mtlkind=heightmap.png".into(),
            "asphalt_g".into(),
            TextureFormat::FmtBc4
        ),
        TextureMaterialKind::TmkHeightmap
    );
    assert_eq!(
        TextureMaterialKind::from_filename(
            "hud_demon_icon_ability_quantumorb.tga$bc3$streamed$mtlkind=particle.png".into(),
            "hud_demon_icon_ability_quantumorb".into(),
            TextureFormat::FmtBc3
        ),
        TextureMaterialKind::TmkParticle
    );
    assert_eq!(
        TextureMaterialKind::from_filename("test.png".into(), "test".into(), TextureFormat::FmtBc1Srgb),
        TextureMaterialKind::TmkAlbedo
    );
}

#[test]
fn test_kraken_compress() {
    let mut test_bytes = vec![
        0x74, 0x65, 0x73, 0x74, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E,
    ];
    let comp_test_bytes = vec![
        68, 73, 86, 73, 78, 73, 84, 89, 15, 0, 0, 0, 0, 0, 0, 0, 204, 6, 116, 101, 115, 116, 99, 111, 109,
        112, 114, 101, 115, 115, 105, 111, 110,
    ];
    assert_eq!(kraken_compress(&mut test_bytes).unwrap(), comp_test_bytes);
}

fn helper_convert_to_bimage(file_path: &str, format: TextureFormat, expected_bim_bytes: [u8; 63]) {
    // Get file name
    let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    let stripped_file_name = file_name
        .split('$')
        .next()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_owned();

    // Load image
    let mut src_reader = match ImageReader::open(file_path) {
        Ok(reader) => reader,
        Err(_) => panic!("Could not load image")
    };

    src_reader.set_format(ImageFormat::Png);

    let src_img = match src_reader.decode() {
        Ok(img) => img.into_rgba8(),
        Err(_) => panic!("Could not load image")
    };

    // Init bc7 encoder
    unsafe {
        bc7e::compress_block_init();
    }

    // Convert image to bimage format
    let bim_bytes = match convert_to_bimage(src_img, file_name.into(), stripped_file_name, format, false) {
        Ok(vec) => vec,
        Err(_) => panic!("Failed to convert to bimage")
    };

    // Compare to expected result
    assert_eq!(bim_bytes[0..63], expected_bim_bytes);
}

#[test]
fn test_convert_to_bimage_1() {
    let file_path = "./test/symbols_arrow_03a_local.tga$bc7$streamed$mtlkind=decalnormal.png";
    let format = TextureFormat::FmtBc7;
    let bim_bytes: [u8; 63] = [
        66, 73, 77, 21, 0, 0, 0, 0, 11, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 23, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_2() {
    let file_path = "./test/glass_textured_orange_n.tga$bc5$streamed.png";
    let format = TextureFormat::FmtBc5;
    let bim_bytes: [u8; 63] = [
        66, 73, 77, 21, 0, 0, 0, 0, 3, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 25, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_3() {
    let file_path = "./test/asphalt_g.tga$bc4$streamed$mtlkind=heightmap.png";
    let format = TextureFormat::FmtBc4;
    let bim_bytes: [u8; 63] = [
        66, 73, 77, 21, 0, 0, 0, 0, 9, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 128, 63, 0, 24, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_4() {
    let file_path = "./test/hud_demon_icon_ability_quantumorb.tga$bc3$streamed.png";
    let format = TextureFormat::FmtBc3;
    let bim_bytes: [u8; 63] = [
        66, 73, 77, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 128, 63, 0, 11, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_5() {
    let file_path = "./test/test.png";
    let format = TextureFormat::FmtBc1Srgb;
    let bim_bytes: [u8; 63] = [
        66, 73, 77, 21, 0, 0, 0, 0, 1, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 128, 63, 0, 33, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}
