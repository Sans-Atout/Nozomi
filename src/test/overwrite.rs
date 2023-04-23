#[test]
fn rename_file() {
    let mut result = crate::utils::rename_file("data/rename_file.txt".to_string(), 11);
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/000000000000").exists());
    result = crate::utils::rename_file("data/rename_file1.txt".to_string(), 11);
    assert!(result.is_err());
}

#[test]
fn afssi_5020() {
    let mut result =
        crate::overwrite::afssi_5020_overwrite_file("data/afssi_5020_overwrite_file.txt");
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/afssi_5020_overwrite_file.txt").exists());
    result = crate::overwrite::afssi_5020_overwrite_file("data/afssi_5020_overwrite_file1.txt");
    assert!(result.is_err());
}

#[test]
fn dod_522022_me() {
    let mut result =
        crate::overwrite::dod_522022_me_overwrite_file("data/dod_522022_me_overwrite_file.txt");
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/dod_522022_me_overwrite_file.txt").exists());
    result =
        crate::overwrite::dod_522022_me_overwrite_file("data/dod_522022_me_overwrite_file1.txt");
    assert!(result.is_err());
}

#[test]
fn dod_522022_mece() {
    let mut result =
        crate::overwrite::dod_522022_mece_overwrite_file("data/dod_522022_mece_overwrite_file.txt");
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/dod_522022_mece_overwrite_file.txt").exists());
    result = crate::overwrite::dod_522022_mece_overwrite_file(
        "data/dod_522022_mece_overwrite_file1.txt",
    );
    assert!(result.is_err());
}

#[test]
fn hexa() {
    let mut result = crate::utils::file_overwriting_hexa("data/file_overwriting_hexa.txt", 0x42_u8);
    assert!(result.is_ok());
    result = crate::utils::file_overwriting_hexa("data/file_overwriting_hexa1.txt", 0x42_u8);
    assert!(result.is_err());
}

#[test]
fn gutmann() {
    let mut result = crate::overwrite::gutmann_overwrite_file("data/gutmann_overwrite_file.txt");
    assert!(result.is_ok());
    result = crate::overwrite::gutmann_overwrite_file("data/gutmann_overwrite_file1.txt");
    assert!(result.is_err());
}

#[test]
fn hmgi_s5() {
    let mut result = crate::overwrite::hmgi_s5_overwrite_file("data/hmgi_s5_overwrite_file.txt");
    assert!(result.is_ok());
    result = crate::overwrite::hmgi_s5_overwrite_file("data/gutmann_overwrite_file1.txt");
    assert!(result.is_err());
}

#[test]
fn rcmp_tssit_ops_ii() {
    let mut result = crate::overwrite::rcmp_tssit_ops_ii_overwrite_file(
        "data/rcmp_tssit_ops_ii_overwrite_file.txt",
    );
    assert!(result.is_ok());
    result = crate::overwrite::rcmp_tssit_ops_ii_overwrite_file("data/gutmann_overwrite_file1.txt");
    assert!(result.is_err());
}

#[test]
fn zero_overwrite() {
    let mut result = crate::utils::file_overwriting_hexa("data/zero_overwrite.txt", 0x00_u8);
    assert!(result.is_ok());
    result =
        crate::utils::file_overwriting_hexa("data/rcmp_tssit_ops_ii_overwrite_file1.txt", 0x00_u8);
    assert!(result.is_err());
}
