// Librairie use in this librairie
pub mod enums;
pub mod utils;
pub mod overwrite;
mod test;

use std::{path::Path, fs};

use enums::erase_method::EraserEntity;
use enums::error::Error;
use enums::success::Success;
use overwrite::{gutmann_overwrite_file, hmgi_s5_overwrite_file, rcmp_tssit_ops_ii_overwrite_file, afssi_5020_overwrite_file, dod_522022_mece_overwrite_file, dod_522022_me_overwrite_file};
use utils::{file_overwriting_random, delete_file};

/// Erase folder method
///
/// Function that allows you to delete, recursively or not, a folder according to a given secure deletion method
///
/// * `_path`        - The path of the folder you wish to delete
/// * `erase_method` - The secure file deletion method (EraserEntity enumeration)
/// * `is_recursive` - Does the deletion have to be recursive or not ?
pub fn erase_folder(
    _path: &str,
    erase_method: EraserEntity,
    is_recursive: bool,
) -> Result<Success, Error> {
    let _p = Path::new(_path);
    if !(_p.exists() && _p.is_dir()) {
        return Err(Error::NotAFolderOrDidntExist);
    }

    let paths = fs::read_dir(_path).unwrap();
    for path in paths {
        let temp_path = path.unwrap().path();
        let _file_name = match temp_path.to_str() {
            Some(file) => file,
            None => return Err(Error::ErrorGetFileName),
        };
        let is_folder = Path::new(_file_name).is_dir();
        if is_folder {
            if is_recursive {
                match erase_folder(_file_name, erase_method, is_recursive) {
                    Ok(_) => true,
                    Err(error) => return Err(error),
                };
            }
        } else {
            match erase_file(_file_name, erase_method) {
                Ok(_) => true,
                Err(error) => return Err(error),
            };
        }
    }
    match fs::remove_dir_all(_path) {
        Ok(_) => Ok(Success::EraseFolderSuccess),
        Err(_) => Err(Error::RemoveDirError),
    }
}

/// Erase one file method
///
/// Function that allows you to delete, recursively or not, a file according to a given secure deletion method
///
/// * `_path`        - The path of the file you wish to delete
/// * `erase_method` - The secure file deletion method (EraserEntity enumeration)
pub fn erase_file(_path: &str, erase_method: EraserEntity) -> Result<Success, Error> {
    let _p = Path::new(_path);
    if !(_p.exists() && _p.is_file()) {
        return Err(Error::NotAFileOrDidntExist);
    }
    match erase_method {
        EraserEntity::Gutmann => {
            match gutmann_overwrite_file(_path) {
                Ok(_) => (),
                Err(error) => return Err(error),
            };
        }
        EraserEntity::HmgiS5 => {
            match hmgi_s5_overwrite_file(_path) {
                Ok(_) => (),
                Err(error) => return Err(error),
            };
        }
        EraserEntity::RcmpTssitOpsII => {
            match rcmp_tssit_ops_ii_overwrite_file(_path) {
                Ok(_) => (),
                Err(error) => return Err(error),
            };
        }
        EraserEntity::PseudoRandom => match file_overwriting_random(_path) {
            Ok(_) => (),
            Err(error) => return Err(error),
        },
        EraserEntity::Afssi5020 => match afssi_5020_overwrite_file(_path) {
            Ok(_) => (),
            Err(error) => return Err(error),
        },
        EraserEntity::Dod522022MECE => match dod_522022_mece_overwrite_file(_path) {
            Ok(_) => (),
            Err(error) => return Err(error),
        },
        EraserEntity::Dod522022ME => match dod_522022_me_overwrite_file(_path) {
            Ok(_) => (),
            Err(error) => return Err(error),
        },
    }

    match delete_file(String::from(_path)) {
        Ok(_) => true,
        Err(error) => return Err(error),
    };
    Ok(Success::EraseFileSuccess)
}

