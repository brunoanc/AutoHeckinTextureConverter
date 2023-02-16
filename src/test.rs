use super::*;

// Tests
#[test]
fn test_get_mipmap_size() {
    assert_eq!(TextureFormat::FmtBc1Srgb.calculate_mipmap_size(1024, 2048), Some(1048576));
    assert_eq!(TextureFormat::FmtBc3.calculate_mipmap_size(720, 560), Some(403200));
    assert_eq!(TextureFormat::FmtBc4.calculate_mipmap_size(271, 783), Some(106624));
    assert_eq!(TextureFormat::FmtBc5.calculate_mipmap_size(576, 254), Some(147456));
    assert_eq!(TextureFormat::FmtBc7.calculate_mipmap_size(2946, 822), Some(2429152));
}

#[test]
fn test_get_texture_material_kind() {
    assert_eq!(
        TextureMaterialKind::from_filename("symbols_arrow_03a_local.tga$bc7$streamed$mtlkind=decalnormal.png".into(),
            "symbols_arrow_03a_local".into(), TextureFormat::FmtBc7),
        TextureMaterialKind::TmkDecalnormal
    );
    assert_eq!(
        TextureMaterialKind::from_filename("glass_textured_orange_n.tga$bc5$streamed.png".into(),
            "glass_textured_orange_n".into(), TextureFormat::FmtBc5),
        TextureMaterialKind::TmkNormal
    );
    assert_eq!(
        TextureMaterialKind::from_filename("asphalt_g.tga$bc4$streamed$mtlkind=heightmap.png".into(),
            "asphalt_g".into(), TextureFormat::FmtBc4),
        TextureMaterialKind::TmkHeightmap
    );
    assert_eq!(
        TextureMaterialKind::from_filename("hud_demon_icon_ability_quantumorb.tga$bc3$streamed$mtlkind=particle.png".into(),
            "hud_demon_icon_ability_quantumorb".into(), TextureFormat::FmtBc3),
        TextureMaterialKind::TmkParticle
    );
    assert_eq!(
        TextureMaterialKind::from_filename("test.png".into(), "test".into(), TextureFormat::FmtBc1Srgb),
        TextureMaterialKind::TmkAlbedo
    );
}

#[test]
fn test_kraken_compress() {
    let test_bytes = vec![0x74, 0x65, 0x73, 0x74, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E];
    let comp_test_bytes = vec![68, 73, 86, 73, 78, 73, 84, 89, 15, 0, 0, 0, 0, 0, 0, 0, 204, 6, 116, 101, 115, 116,
        99, 111, 109, 112, 114, 101, 115, 115, 105, 111, 110];
    assert_eq!(kraken_compress(test_bytes).unwrap(), comp_test_bytes);
}

fn helper_convert_to_bimage(file_path: &str, format: TextureFormat, expected_bim_bytes: [u8; 63]) {
    // Get file name
    let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    let stripped_file_name = file_name.split('$').next().unwrap().split('.').next().unwrap().to_owned();

    // Load image
    let src_img = match image::open(file_path) {
        Ok(img) => DynamicImage::ImageRgba8(img.into_rgba8()),
        Err(_) => panic!("Could not load image")
    };

    // Init bc7 encoder
    unsafe {
        bc7e::bc7e_compress_block_init();
    }

    // Init bc7 compress options
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

    // Convert image to bimage format
    let bim_bytes = match convert_to_bimage(src_img, file_name.into(), stripped_file_name, format, false, p) {
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
    let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 11, 0, 0, 0, 128, 0, 0, 0, 128,
    0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 23, 0, 0, 0,
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_2() {
    let file_path = "./test/glass_textured_orange_n.tga$bc5$streamed.png";
    let format = TextureFormat::FmtBc5;
    let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 3, 0, 0, 0, 128, 0, 0, 0, 128,
    0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 25, 0, 0, 0,
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_3() {
    let file_path = "./test/asphalt_g.tga$bc4$streamed$mtlkind=heightmap.png";
    let format = TextureFormat::FmtBc4;
    let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 9, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0,
    0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 24, 0, 0, 0, 7, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_4() {
    let file_path = "./test/hud_demon_icon_ability_quantumorb.tga$bc3$streamed.png";
    let format = TextureFormat::FmtBc3;
    let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 11, 0, 0, 0, 7, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}

#[test]
fn test_convert_to_bimage_5() {
    let file_path = "./test/test.png";
    let format = TextureFormat::FmtBc1Srgb;
    let bim_bytes: [u8; 63] = [66, 73, 77, 21, 0, 0, 0, 0, 1, 0, 0, 0, 0, 8, 0, 0, 0, 8,
    0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 33, 0, 0, 0,
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    helper_convert_to_bimage(file_path, format, bim_bytes);
}
