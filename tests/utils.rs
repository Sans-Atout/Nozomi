use std::path::Path;

use nozomi::utils;
use nozomi::OverwriteMethod::PseudoRandom;
use nozomi::{
    erase_file, erase_folder,
    utils::{delete_file, generate_zero_string},
};
use pretty_assertions::assert_eq;

/// Function that test if the function `generate_zero_string` work well
/// 
/// Test passed if the function can generate a for zero string 
#[test]
fn zero_string() {
    let string_res = generate_zero_string(4);
    assert_eq!(string_res, "00000".to_string());
}

/// Function that test if the function `delete_file` work well
/// 
/// Test passed if :
/// * the function can delete a file
/// * if we want to delete a non existing file the function return an error
#[test]
fn delete() {
    let mut result = delete_file("./data/delete_file.txt".to_string());
    assert!(result.is_ok());
    assert!(!std::path::Path::new("./data/delete_file.txt").exists());
    result = delete_file("./data/delete_file.txt".to_string());
    assert!(result.is_err());
}

/// Function that test if the function `erase_file` work well
/// 
/// Test passed if :
/// * we can erase a file with this method
/// * if the path given is invalid the function return an error
#[test]
fn file() {
    let mut result = erase_file(&"./data/file_to_erase.txt", PseudoRandom);
    assert!(result.is_ok());
    assert!(!Path::new("./data/file_1.txt").exists());
    result = erase_file(&"./data/invalid.txt", nozomi::OverwriteMethod::PseudoRandom);
    assert!(result.is_err());
}

/// Function that test if the function `erase_folder` work well
/// 
/// Test passed if :
/// * we can erase a folder with this method
/// * if the path given is invalid the function return an error
#[test]
fn folder() {
    let mut result = erase_folder(&"./data/folder_to_erase", PseudoRandom, false);
    assert!(result.is_ok());
    assert!(!Path::new("./data/folder").exists());
    result = crate::erase_folder(&"./data/folder", PseudoRandom, false);
    assert!(result.is_err());
}

/// Function that test if the function `rename_file` work well
/// 
/// Test passed if :
/// * we can rename a file with this method
/// * if the path given is invalid the function return an error
#[test]
fn rename_file() {
    let mut result = utils::rename_file("data/rename_file.txt".to_string(), 11);
    assert!(result.is_ok());
    assert!(std::path::Path::new("data/000000000000").exists());
    result = utils::rename_file("data/invalid.txt".to_string(), 11);
    assert!(result.is_err());
}
