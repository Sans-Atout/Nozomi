use std::{
    fs,
    path::{self, Path},
};

use crate::error::Error;
#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct SecureDelete {
    path: String,
    pattern: Option<[u8; 3]>,
    byte: Option<u8>,
    buffer_size : usize
}

#[cfg(not(feature = "error-stack"))]
impl SecureDelete {
    pub fn new(path: &str) -> Result<Self> {
        if !Path::new(&path).exists() {
            return Err(crate::Error::FileNotFound(path.to_string()));
        }
        Ok(SecureDelete {
            path: path.to_string(),
            pattern: None,
            byte: None,
            buffer_size : 4096
        })
    }

    pub fn delete(&mut self) -> Result<()> {
        let zero_name = self.zero_name()?;

        let mut new_path = Path::new(&self.path).to_path_buf();
        new_path.set_file_name(&zero_name);
        self.rename(&new_path)?;

        let mut anon_file_size = zero_name.len();
        while anon_file_size > 1 {
            anon_file_size -= 1;
            let new_file_name = (0..anon_file_size).map(|_| "0").collect::<String>();
            new_path.set_file_name(&new_file_name);
            self.rename(&new_path)?;
        }
        fs::remove_file(&self.path).map_err(|_| Error::FileDeletionError(self.path.clone()))?;
        Ok(())
    }

    pub fn rename(&mut self, new_name: &Path) -> Result<()> {
        fs::rename(&self.path, new_name).map_err(|_| Error::RenameError(self.path.clone()))?;
        self.path = new_name
            .to_str()
            .ok_or(Error::StringConversionError)?
            .to_string();
        Ok(())
    }

    pub fn overwrite(&mut self) -> Result<()> {
        Ok(())
    }

    fn zero_name(&self) -> Result<String> {
        let name = Path::new(&self.path)
            .file_name()
            .ok_or(Error::NoFileName(self.clone().to_owned()))?;
        let new_name = (0..name.len()).map(|_| "0").collect::<String>();
        Ok(new_name)
    }

}

#[cfg(feature = "error-stack")]
impl SecureDelete {
    pub fn new(path: &str) -> Result<Self> {
        if !Path::new(&path).exists() {
            return Err(crate::Error::FileNotFound(path.to_string()));
        }
        Ok(SecureDelete {
            path: path.to_string(),
            pattern: None,
            byte: None,
        })
    }
}

impl SecureDelete {
    pub fn byte(&mut self, byte: &u8) -> &mut Self {
        self.byte = Some(*byte);
        self.pattern = None;
        self
    }

    pub fn pattern(&mut self, pattern: &[u8; 3]) -> &mut Self {
        self.pattern = Some(*pattern);
        self.byte = None;
        self
    }

    pub fn buffer(&mut self, new_buffer_size : usize) -> &mut Self{
        self.buffer_size = new_buffer_size;
        self
    }

}

#[cfg(test)]
#[cfg(not(feature = "error-stack"))]
mod std_test {
    use std::fs::File;

    use crate::Error;
    use crate::Result;
    use pretty_assertions::{assert_eq, assert_str_eq};

    use super::SecureDelete;

    #[test]
    fn creation() -> Result<()> {
        let mut basic_creation = SecureDelete::new("README.md")?;
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: None,
                buffer_size : 4096
            }
        );
        basic_creation.pattern(&[0x00_u8, 0x00_u8, 0x00_u8]);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: Some([0x00_u8, 0x00_u8, 0x00_u8]),
                buffer_size : 4096
            }
        );
        basic_creation.byte(&0x00_u8);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: Some(0x00_u8),
                pattern: None,
                buffer_size : 4096
            }
        );
        Ok(())
    }

    #[test]
    fn zero_string() -> Result<()> {
        let tested = SecureDelete::new("README.md")?.zero_name()?;
        assert_eq!("000000000", &tested);
        assert_ne!("0000000", &tested);

        let folder_test = SecureDelete::new("images/AFSSI_5020.png")?.zero_name()?;
        assert_eq!("00000000000000", &folder_test);
        Ok(())
    }

    #[test]
    fn rename_test() -> Result<()> {
        let mut file_to_rename = std::env::temp_dir();
        file_to_rename.push("nozomi_to_rename_file.txt");
        let file_to_rename_path = file_to_rename.as_path();
        File::create(file_to_rename_path).map_err(|e| Error::FileCreationError(e))?;

        let mut secure_delete = SecureDelete::new(
            file_to_rename_path
                .to_str()
                .ok_or(Error::StringConversionError)?,
        )?;
        file_to_rename.pop();
        file_to_rename.push("0000000000000000000000000");
        let wanted_path = file_to_rename
            .to_str()
            .ok_or(Error::StringConversionError)?;

        secure_delete.rename(file_to_rename.as_path())?;

        assert!(file_to_rename.exists());
        assert_eq!(
            secure_delete.clone(),
            SecureDelete {
                path: wanted_path.to_string(),
                pattern: None,
                byte: None,
                buffer_size : 4096
            }
        );
        Ok(())
    }

    #[test]
    fn deletion_test() -> Result<()> {
        let mut file_to_rename = std::env::temp_dir();
        file_to_rename.push("nozomi_deletion_test.txt");
        let file_to_rename_path = file_to_rename.as_path();
        File::create(file_to_rename_path).map_err(|e| Error::FileCreationError(e))?;
        let mut secure_delete = SecureDelete::new(
            &file_to_rename_path
                .to_str()
                .ok_or(Error::StringConversionError)?,
        )?;
        secure_delete.delete()?;
        let mut file_to_rename = std::env::temp_dir();
        file_to_rename.push("0");
        assert_eq!(secure_delete,             SecureDelete {
            path:             file_to_rename
            .to_str()
            .ok_or(Error::StringConversionError)?.to_string(),
            pattern: None,
            byte: None,
            buffer_size : 4096
        });
        assert!(!file_to_rename_path.exists());
        Ok(())
    }

    #[test]
    fn resize_buffer() -> Result<()> {
        let mut basic_creation = SecureDelete::new("README.md")?;
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: None,
                buffer_size : 4096
            }
        );
        basic_creation.buffer(1024);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: None,
                buffer_size : 1024
            }
        );
        Ok(())
    }
}
