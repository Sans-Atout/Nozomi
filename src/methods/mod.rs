pub mod afssi_5020;
pub mod dod_522022_me;
pub mod dod_522022_mece;
pub mod gutmann;
pub mod hmgi_s5;
pub mod pseudo_random;
pub mod rcmp_tssit_ops_ii;

use crate::error::FSProblem;
use crate::SecureDelete;
use std::{fs::read_dir, path::Path};

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "log")]
use log::{error, info, warn};

/// Nozomi Eraser method enumeration based on Eraser for Windows main method
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Method {
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

#[cfg(not(feature = "error-stack"))]
impl Method {
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);

        if !path_to_delete.exists() {
            #[cfg(all(feature = "log", not(feature = "secure_log")))]
            error!("[{path}]\t did not exist");
            #[cfg(all(feature = "log", feature = "secure_log"))]
            error!("[{:x}]\tdid not exist", md5::compute(&path));
            return Err(Error::SystemProblem(FSProblem::NotFound, path.to_string()));
        }
        if !path_to_delete.is_dir() {
            match self {
                Method::Dod522022MECE => dod_522022_me::overwrite_file(path)?.delete()?,
                Method::Dod522022ME => dod_522022_mece::overwrite_file(path)?.delete()?,
                Method::Afssi5020 => afssi_5020::overwrite_file(path)?.delete()?,
                Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::overwrite_file(path)?.delete()?,
                Method::HmgiS5 => hmgi_s5::overwrite_file(path)?.delete()?,
                Method::Gutmann => gutmann::overwrite_file(path)?.delete()?,
                Method::PseudoRandom => pseudo_random::overwrite_file(path)?.delete()?,
            };
            return Ok(());
        }

        self.delete_folder(path_to_delete)?.delete()?;
        Ok(())
    }

    fn delete_folder(&self, path: &Path) -> Result<SecureDelete> {
        #[cfg(all(feature = "log", feature = "secure_log"))]
        let md5_value = md5::compute(
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)?
                .to_string(),
        );
        if !path.is_dir() {
            #[cfg(all(feature = "log", not(feature = "secure_log")))]
            error!("[{:#?}]\t is neither a folder or a file", path);
            #[cfg(all(feature = "log", feature = "secure_log"))]
            error!("[{:x}]\t is neither a folder or a file", md5_value);
            return Err(Error::SystemProblem(
                FSProblem::NotFound,
                path.as_os_str()
                    .to_str()
                    .ok_or(Error::StringConversionError)?
                    .to_string(),
            ));
        }
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        warn!("[{:#?}]\t is a folder", path);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        warn!("[{:x}]\t is a folder", md5_value);
        let files = read_dir(path).map_err(|_| {
            Error::SystemProblem(
                FSProblem::ReadFolder,
                path.as_os_str()
                    .to_str()
                    .ok_or(Error::StringConversionError)
                    .unwrap()
                    .to_string(),
            )
        })?;
        for file in files {
            if file.is_err() {
                #[cfg(all(feature = "log", not(feature = "secure_log")))]
                error!("[{:#?}]\t error during file reading", path);
                #[cfg(all(feature = "log", feature = "secure_log"))]
                error!("[{:x}]\t error during file reading", md5_value);
                continue;
            }
            let dir_entry = file
                .map_err(|_| {
                    Error::SystemProblem(
                        FSProblem::ReadFolder,
                        path.as_os_str()
                            .to_str()
                            .ok_or(Error::StringConversionError)
                            .unwrap()
                            .to_string(),
                    )
                })?
                .path();
            let path = dir_entry
                .as_path()
                .to_str()
                .ok_or(Error::StringConversionError)?;
            self.delete(path)?;
        }
        let folder_to_delete = path.to_str().ok_or(Error::StringConversionError)?;
        SecureDelete::new(folder_to_delete)
    }
}

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Context, Report, ResultExt};

#[cfg(feature = "error-stack")]
impl Method {
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);

        if !path_to_delete.exists() {
            #[cfg(all(feature = "log", not(feature = "secure_log")))]
            error!("[{path}]\t did not exist");
            #[cfg(all(feature = "log", feature = "secure_log"))]
            error!("[{:x}]\tdid not exist", md5::compute(&path));
            return Err(Report::new(Error::SystemProblem(
                FSProblem::NotFound,
                path.to_string(),
            )));
        }
        if !path_to_delete.is_dir() {
            match self {
                Method::Dod522022MECE => dod_522022_me::overwrite_file(path)?.delete()?,
                Method::Dod522022ME => dod_522022_mece::overwrite_file(path)?.delete()?,
                Method::Afssi5020 => afssi_5020::overwrite_file(path)?.delete()?,
                Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::overwrite_file(path)?.delete()?,
                Method::HmgiS5 => hmgi_s5::overwrite_file(path)?.delete()?,
                Method::Gutmann => gutmann::overwrite_file(path)?.delete()?,
                Method::PseudoRandom => pseudo_random::overwrite_file(path)?.delete()?,
            };
            return Ok(());
        }

        self.delete_folder(path_to_delete)?.delete()?;
        Ok(())
    }

    fn delete_folder(&self, path: &Path) -> Result<SecureDelete> {
        #[cfg(all(feature = "log", feature = "secure_log"))]
        let md5_value = md5::compute(
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)?
                .to_string(),
        );

        if !path.is_dir() {
            #[cfg(all(feature = "log", not(feature = "secure_log")))]
            error!("[{:#?}]\t is neither a folder or a file", path);
            #[cfg(all(feature = "log", feature = "secure_log"))]
            error!("[{:x}]\t is neither a folder or a file", md5_value);

            return Err(Report::new(Error::SystemProblem(
                FSProblem::NotFound,
                path.as_os_str()
                    .to_str()
                    .ok_or(Error::StringConversionError)?
                    .to_string(),
            )));
        }
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        warn!("[{:#?}]\t is a folder", path);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        warn!("[{:x}]\t is a folder", md5_value);

        let files = read_dir(path).change_context(Error::SystemProblem(
            FSProblem::ReadFolder,
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)
                .unwrap()
                .to_string(),
        ))?;
        for file in files {
            if file.is_err() {
                #[cfg(all(feature = "log", not(feature = "secure_log")))]
                error!("[{:#?}]\t error during file reading", path);
                #[cfg(all(feature = "log", feature = "secure_log"))]
                error!("[{:x}]\t error during file reading", md5_value);
                continue;
            }
            let dir_entry = file
                .change_context(Error::SystemProblem(
                    FSProblem::ReadFolder,
                    path.as_os_str()
                        .to_str()
                        .ok_or(Error::StringConversionError)
                        .unwrap()
                        .to_string(),
                ))?
                .path();
            let path = dir_entry
                .as_path()
                .to_str()
                .ok_or(Error::StringConversionError)?;
            self.delete(path)?;
        }
        let folder_to_delete = path.to_str().ok_or(Error::StringConversionError)?;
        SecureDelete::new(folder_to_delete)
    }
}

impl core::fmt::Display for Method {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Method::Dod522022MECE => write!(fmt, "DOD 522022 MECE"),
            Method::Dod522022ME => write!(fmt, "DOD 522022 ME"),
            Method::Afssi5020 => write!(fmt, "AFSSI 5020"),
            Method::RcmpTssitOpsII => write!(fmt, "RCMP TSSIT OPS II"),
            Method::HmgiS5 => write!(fmt, "HMGI S5"),
            Method::Gutmann => write!(fmt, "Gutmann"),
            Method::PseudoRandom => write!(fmt, "Pseudo Random"),
        }
    }
}
