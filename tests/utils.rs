
use std::path::Path;

use nozomi::{enums::erase_method::EraserEntity, erase_folder, utils::{delete_file, generate_zero_string, self}, erase_file};

#[test]
fn zero_string() {
    let string_res = generate_zero_string(4);
    assert_eq!(string_res, "00000".to_string());
}

#[test]
fn delete() {
    let mut result = delete_file("./data/delete_file.txt".to_string());
    assert!(result.is_ok());
    assert!(!std::path::Path::new("./data/delete_file.txt").exists());
    result = delete_file("./data/delete_file.txt".to_string());
    assert!(result.is_err());
}

#[test]
fn file() {
    let mut result = erase_file(&"./data/file_to_erase.txt", EraserEntity::PseudoRandom);
    assert!(result.is_ok());
    assert!(!Path::new("./data/file_1.txt").exists());
    result = erase_file(&"./data/file_1.txt", EraserEntity::PseudoRandom);
    assert!(result.is_err());
}

#[test]
fn folder() {
    let mut result = erase_folder(&"./data/folder_to_erase", EraserEntity::PseudoRandom, false);
    assert!(result.is_ok());
    assert!(!Path::new("./data/folder").exists());
    result = crate::erase_folder(&"./data/folder", EraserEntity::PseudoRandom, false);
    assert!(result.is_err());
}

#[test]
fn rename_file() {
    let mut result =  utils::rename_file("data/rename_file.txt".to_string(), 11);
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/000000000000").exists());
    result = utils::rename_file("data/rename_file1.txt".to_string(), 11);
    assert!(result.is_err());
}
