use std::{fs::{self, read_dir}, path::Path};

mod gutmann;

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

use crate::models::SecureDelete;
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(not(feature = "error-stack"))]
impl Method {
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);

        if !path_to_delete.exists() {
            return Err(Error::FileNotFound(path.to_string()));
        }
        if !path_to_delete.is_dir() {
            match self {
                Method::Dod522022MECE => todo!(),
                Method::Dod522022ME => todo!(),
                Method::RcmpTssitOpsII => todo!(),
                Method::HmgiS5 => todo!(),
                Method::Gutmann => gutmann::overwrite_file(path)?.delete()?,
            };
            return Ok(());
        }

        self.delete_folder(path_to_delete)?.delete()?;
        Ok(())
    }

    fn delete_folder(&self, path : &Path) -> Result<SecureDelete> {
        if !path.is_dir() {
            return Err(Error::FileNotFound(
                path.as_os_str()
                    .to_str()
                    .ok_or(Error::StringConversionError)?
                    .to_string(),
            ));
        }
        let files = read_dir(&path).map_err(|_| Error::CannotReadFolder)?;
        for file in files {
            if file.is_err() {}
            let dir_entry = file.map_err(|_| Error::CannotReadFolder)?.path();
            let path = dir_entry
                .as_path()
                .to_str()
                .ok_or(Error::StringConversionError)?;
            self.delete(path)?;
        }
        let folder_to_delete = path.to_str().ok_or(Error::StringConversionError)?;
        Ok(SecureDelete::new(folder_to_delete)?)
    }
}
