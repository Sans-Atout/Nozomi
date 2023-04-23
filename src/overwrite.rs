use std::fs::File;
use std::io::{BufReader, Read};

use crate::utils::{file_overwriting,file_overwriting_random, file_overwriting_hexa};
use crate::enums::{success::Success, error::Error};

pub fn gutmann_overwrite_file(_path: &str) -> Result<Success, Error> {
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting(_path, [0x55_u8, 0x55_u8, 0x55_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xAA_u8, 0xAA_u8, 0xAA_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x92_u8, 0x49_u8, 0x24_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x00_u8, 0x00_u8, 0x00_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x11_u8, 0x11_u8, 0x11_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x22_u8, 0x22_u8, 0x22_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x33_u8, 0x33_u8, 0x33_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x44_u8, 0x44_u8, 0x44_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x55_u8, 0x55_u8, 0x55_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x66_u8, 0x66_u8, 0x66_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x77_u8, 0x77_u8, 0x77_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x88_u8, 0x88_u8, 0x88_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x99_u8, 0x99_u8, 0x99_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xAA_u8, 0xAA_u8, 0xAA_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xBB_u8, 0xBB_u8, 0xBB_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xCC_u8, 0xCC_u8, 0xCC_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xDD_u8, 0xDD_u8, 0xDD_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xEE_u8, 0xEE_u8, 0xEE_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xFF_u8, 0xFF_u8, 0xFF_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x92_u8, 0x49_u8, 0x24_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x49_u8, 0x24_u8, 0x92_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0x6D_u8, 0xB6_u8, 0xDB_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xB6_u8, 0xDB_u8, 0x6D_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting(_path, [0xDB_u8, 0x6D_u8, 0xB6_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError),
    };
    Ok(Success::GutmannSuccess)
}

pub fn hmgi_s5_overwrite_file(_path: &str) -> Result<Success, Error> {
    match file_overwriting(_path, [0x00_u8, 0x00_u8, 0x00_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::HmgiS5ErrorFirst),
    };
    match file_overwriting(_path, [0xFF_u8, 0xFF_u8, 0xFF_u8]) {
        Ok(_) => true,
        Err(_) => return Err(Error::HmgiS5ErrorSecond),
    };
    let mut _file = match File::options().read(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };

    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let _d = match reader.read_to_end(&mut buffer) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError),
    };
    for _char in buffer {
        if !_char == 0xFF_u8 {
            return Err(Error::VerificationFailed);
        }
    }
    Ok(Success::HmgiS5Sucess)
}

pub fn rcmp_tssit_ops_ii_overwrite_file(_path: &str) -> Result<Success, Error> {
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFirst),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => (),
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSecond),
    };
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorThird),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFourth),
    };
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFifth),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSixth),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSeventh),
    };
    Ok(Success::RcmpTssitOpsIISucess)
}

pub fn afssi_5020_overwrite_file(_path: &str) -> Result<Success, Error> {
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorFirst),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorSecond),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorThird),
    };
    Ok(Success::Afssi5020Success)
}

pub fn dod_522022_mece_overwrite_file(_path: &str) -> Result<Success, Error> {
    match dod_522022_me_overwrite_file(_path) {
        Ok(_) => true,
        Err(error) => return Err(error),
    };
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFourth),
    };
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFifth),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorSixth),
    };
    match file_overwriting_random(_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFourth),
    };
    Ok(Success::Dod522022MECESuccess)
}

pub fn dod_522022_me_overwrite_file(_path: &str) -> Result<Success, Error> {
    match file_overwriting_hexa(_path, 0x00_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorFirst),
    };
    match file_overwriting_hexa(_path, 0xFF_u8) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorSecond),
    };
    match file_overwriting(_path, crate::utils::get_random_patern()) {
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorThird),
    };
    Ok(Success::Dod522022MESucess)
}
