// Libraries use in this library
pub mod error;
pub mod method;
pub mod utils;

use std::{fs, path::Path};

use error::{InputError, ProcessError};
use error_stack::{IntoReport, Report, Result, ResultExt};

use method::{
    afssi_5020_overwrite_file, dod_522022_me_overwrite_file, dod_522022_mece_overwrite_file,
    gutmann_overwrite_file, hmgi_s5_overwrite_file, rcmp_tssit_ops_ii_overwrite_file,
};
use utils::{delete_file, Overwrite};

/// Nozomi Eraser method enumeration based on Eraser for Windows main method
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum OverwriteMethod {
    /// DOD 522022 MECE erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022MECE,
    /// DOD 522022 ME erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022ME,
    /// AFSSI 5020 erasing method <https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020>
    Afssi5020,
    /// RCMP TSSIT OPS II erasing method <https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html>
    RcmpTssitOpsII,
    /// HMGI S5 erasing method <https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php>
    HmgiS5,
    /// Gutmann erasing method <https://en.wikipedia.org/wiki/Gutmann_method>
    Gutmann,
    /// Pseudo Random erasing method <https://www.lifewire.com/data-sanitization-methods-2626133#toc-random-data>
    #[default]
    PseudoRandom,
}

/// Erase folder method
///
/// Function that allows you to delete, recursively or not, a folder according to a given secure deletion method
/// * `_path`        - The path of the folder you wish to delete
/// * `erase_method` - The secure file deletion method (OverwriteMethod enumeration)
/// * `is_recursive` - Does the deletion have to be recursive or not ?
pub fn erase_folder(
    _path: &str,
    erase_method: OverwriteMethod,
    is_recursive: bool,
) -> Result<(), ProcessError> {
    let _p = Path::new(_path);
    if !_p.exists() {
        return Err(Report::new(InputError)
            .attach_printable(format!("Folder does not exist  {_path}"))
            .change_context(ProcessError)
            .attach_printable("Process Error : erase folder fail"));
    }
    if !_p.is_dir() {
        return Err(Report::new(InputError)
            .attach_printable(format!("Given path is not a folder : {_path}"))
            .change_context(ProcessError)
            .attach_printable("Process Error : erase folder fail"));
    }
    let paths = fs::read_dir(_path).unwrap();
    for path in paths {
        let temp_path = path.unwrap().path();
        let _file_name = match temp_path.to_str() {
            Some(file) => file,
            None => {
                return Err(Report::new(InputError)
                    .attach_printable(format!("Cannot retrieve file name : {_path}"))
                    .change_context(ProcessError)
                    .attach_printable("Process Error : erase folder fail"))
            }
        };
        let is_folder = Path::new(_file_name).is_dir();
        if is_folder {
            if is_recursive {
                erase_folder(_file_name, erase_method, is_recursive)?;
            }
        } else {
            match erase_file(_file_name, erase_method) {
                Ok(_) => true,
                Err(error) => return Err(error),
            };
        }
    }
    fs::remove_dir_all(_path)
        .into_report()
        .change_context(ProcessError)
        .attach_printable("Process Error : can not delete folder")?;
    Ok(())
}

/// Erase one file method
///
/// Function that allows you to delete, recursively or not, a file according to a given secure deletion method
/// * `_path`        - The path of the file you wish to delete
/// * `erase_method` - The secure file deletion method (OverwriteMethod enumeration)
pub fn erase_file(_path: &str, erase_method: OverwriteMethod) -> Result<(), ProcessError> {
    let _p = Path::new(&_path);

    if !_p.exists() {
        return Err(Report::new(InputError)
            .attach_printable(format!("File does not exist : {_path}"))
            .change_context(ProcessError)
            .attach_printable("attachment"));
    }
    if !_p.is_file() {
        return Err(Report::new(InputError)
            .attach_printable(format!("Given path is not a file : {_path}"))
            .change_context(ProcessError)
            .attach_printable("Process Error : erase folder fail"));
    }
    match erase_method {
        OverwriteMethod::Gutmann => gutmann_overwrite_file(_path)?,
        OverwriteMethod::HmgiS5 => hmgi_s5_overwrite_file(_path)?,
        OverwriteMethod::RcmpTssitOpsII => rcmp_tssit_ops_ii_overwrite_file(_path)?,
        OverwriteMethod::PseudoRandom => Overwrite::new(_path).write()?,
        OverwriteMethod::Afssi5020 => afssi_5020_overwrite_file(_path)?,
        OverwriteMethod::Dod522022MECE => dod_522022_mece_overwrite_file(_path)?,
        OverwriteMethod::Dod522022ME => dod_522022_me_overwrite_file(_path)?,
    }

    delete_file(String::from(_path))
        .change_context(ProcessError)
        .attach_printable(format!("Process Error : Cannot delete file {_path}"))?;
    Ok(())
}
