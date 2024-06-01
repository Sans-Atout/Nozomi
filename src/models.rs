use crate::error::FSProblem;
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Report, ResultExt};
#[cfg(feature = "log")]
use log::trace;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct SecureDelete {
    path: String,
    pattern: Option<[u8; 3]>,
    byte: Option<u8>,
    buffer_size: usize,
    #[cfg(feature = "secure_log")]
    pub md5: md5::Digest,
}

impl SecureDelete {
    pub fn byte(&mut self, byte: &u8) -> &mut Self {
        #[cfg(feature = "log")]
        trace!("[{}]\tbyte [{:x}]\tpattern [None]", &self.path, byte);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tbyte [{:x}]\tpattern [None]", &self.md5, byte);

        self.byte = Some(*byte);
        self.pattern = None;
        self
    }

    pub fn pattern(&mut self, pattern: &[u8; 3]) -> &mut Self {
        #[cfg(feature = "log")]
        trace!(
            "[{}]\tbyte [None]\tpatern [{:x}{:x}{:x}]",
            &self.path,
            &pattern[0],
            &pattern[1],
            &pattern[2]
        );
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tbyte [None]\tpatern [{:x}{:x}{:x}]",
            &self.md5,
            &pattern[0],
            &pattern[1],
            &pattern[2]
        );
        self.pattern = Some(*pattern);
        self.byte = None;
        self
    }

    pub fn buffer(&mut self, new_buffer_size: usize) -> &mut Self {
        #[cfg(feature = "log")]
        trace!("[{}]\tbuffer size [{}]", &self.path, new_buffer_size);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tbuffer size [{}]", &self.md5, new_buffer_size);

        self.buffer_size = new_buffer_size;
        self
    }

    fn get_buffer(&self, size: usize) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        if self.byte.is_some() {
            let byte = &self.byte.unwrap();
            for _ in 0..size {
                buffer.push(*byte);
            }
            return buffer;
        }
        if self.pattern.is_some() {
            let bytes_pattern = &self.pattern.unwrap();
            for i in 0..size {
                let pattern_index = i % 3;
                buffer.push(bytes_pattern[pattern_index]);
            }
            return buffer;
        }
        for _ in 0..size {
            let random_byte: u8 = rand::random();
            buffer.push(random_byte);
        }
        buffer
    }
}

#[cfg(not(feature = "error-stack"))]
impl SecureDelete {
    pub fn new(path: &str) -> Result<Self> {
        if !Path::new(&path).exists() {
            return Err(Error::SystemProblem(FSProblem::NotFound, path.to_string()));
        }
        #[cfg(feature = "log")]
        trace!("[{}]\tSecure deletion object creation", &path);
        #[cfg(feature = "secure_log")]
        let computed_md5 = md5::compute(&path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tSecure deletion object creation", &computed_md5);

        Ok(SecureDelete {
            path: path.to_string(),
            pattern: None,
            byte: None,
            buffer_size: 4096,
            #[cfg(feature = "secure_log")]
            md5: computed_md5.clone(),
        })
    }

    pub fn delete(&mut self) -> Result<()> {
        let zero_name = self.zero_name()?;
        #[cfg(feature = "log")]
        trace!("[{}]\tBeginning of deletion", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tBeginning of deletion", &self.md5);

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
        if Path::new(&self.path).is_dir() {
            fs::remove_dir(&self.path)
                .map_err(|_| Error::SystemProblem(FSProblem::Delete, self.path.clone()))?;
            return Ok(());
        }
        fs::remove_file(&self.path)
            .map_err(|_| Error::SystemProblem(FSProblem::Delete, self.path.clone()))?;

        #[cfg(feature = "log")]
        trace!("[{}]\tEnding of deletion", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tEnding of deletion", &self.md5);

        Ok(())
    }

    pub fn rename(&mut self, new_name: &Path) -> Result<()> {
        fs::rename(&self.path, new_name)
            .map_err(|_| Error::SystemProblem(FSProblem::Rename, self.path.clone()))?;
        self.path = new_name
            .to_str()
            .ok_or(Error::StringConversionError)?
            .to_string();
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &self.path);
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &self.md5,
            md5::compute(&self.path)
        );

        Ok(())
    }

    pub fn overwrite(&mut self) -> Result<&mut Self> {
        #[cfg(feature = "log")]
        trace!("[{}]\tBegging of overwritting phase", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tBegging of overwritting phase", &self.md5);

        let file_to_overwrite = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(|_| Error::SystemProblem(FSProblem::Opening, self.path.clone()))?;
        let file_size = file_to_overwrite
            .metadata()
            .map_err(|_| Error::SystemProblem(FSProblem::Opening, self.path.clone()))?
            .size();
        let mut overwrited_length: u64 = 0;
        let mut overwritting_buffer = BufWriter::new(file_to_overwrite);

        while overwrited_length < file_size {
            overwrited_length += self.buffer_size as u64;
            if file_size <= overwrited_length {
                let special_buffer_size =
                    file_size as usize + self.buffer_size - overwrited_length as usize;
                overwritting_buffer
                    .write(&self.get_buffer(special_buffer_size))
                    .map_err(|_| Error::SystemProblem(FSProblem::Write, self.path.clone()))?;
                break;
            }
            overwritting_buffer
                .write(&self.get_buffer(self.buffer_size))
                .map_err(|_| Error::SystemProblem(FSProblem::Write, self.path.clone()))?;
        }
        #[cfg(feature = "log")]
        trace!("[{}]\tEnding of overwritting phase", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tEnding of overwritting phase", &self.md5);
        Ok(self)
    }

    fn zero_name(&self) -> Result<String> {
        let name = Path::new(&self.path)
            .file_name()
            .ok_or(Error::NoFileName(self.clone().to_owned()))?;
        let new_name = (0..name.len()).map(|_| "0").collect::<String>();
        Ok(new_name)
    }
}

#[cfg(test)]
#[cfg(not(any(feature = "error-stack", feature = "log", feature = "secure_log")))]
mod std_test {
    use std::fs::File;

    use crate::Error;
    use crate::Result;
    use pretty_assertions::assert_eq;

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
                buffer_size: 4096,
            }
        );
        basic_creation.pattern(&[0x00_u8, 0x00_u8, 0x00_u8]);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: Some([0x00_u8, 0x00_u8, 0x00_u8]),
                buffer_size: 4096,
            }
        );
        basic_creation.byte(&0x00_u8);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: Some(0x00_u8),
                pattern: None,
                buffer_size: 4096,
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
                buffer_size: 4096,
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
        assert_eq!(
            secure_delete,
            SecureDelete {
                path: file_to_rename
                    .to_str()
                    .ok_or(Error::StringConversionError)?
                    .to_string(),
                pattern: None,
                byte: None,
                buffer_size: 4096,
            }
        );
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
                buffer_size: 4096,
            }
        );
        basic_creation.buffer(1024);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: None,
                buffer_size: 1024,
            }
        );
        Ok(())
    }
}

#[cfg(feature = "error-stack")]
impl SecureDelete {
    pub fn new(path: &str) -> Result<Self> {
        if !Path::new(&path).exists() {
            return Err(Report::new(Error::SystemProblem(
                FSProblem::NotFound,
                path.to_string(),
            )));
        }

        #[cfg(feature = "log")]
        trace!("[{}]\tSecure deletion object creation", &path);
        #[cfg(feature = "secure_log")]
        let computed_md5 = md5::compute(&path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tSecure deletion object creation", &computed_md5);

        Ok(SecureDelete {
            path: path.to_string(),
            pattern: None,
            byte: None,
            buffer_size: 4096,
            #[cfg(feature = "secure_log")]
            md5: computed_md5,
        })
    }

    pub fn delete(&mut self) -> Result<()> {
        let zero_name = self.zero_name()?;

        #[cfg(feature = "log")]
        trace!("[{}]\tBeginning of deletion", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tBeginning of deletion", &self.md5);

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
        if Path::new(&self.path).is_dir() {
            fs::remove_dir(&self.path)
                .change_context(Error::SystemProblem(FSProblem::Delete, self.path.clone()))?;
            return Ok(());
        }
        fs::remove_file(&self.path)
            .change_context(Error::SystemProblem(FSProblem::Delete, self.path.clone()))?;
        #[cfg(feature = "log")]
        trace!("[{}]\tEnding of deletion", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tEnding of deletion", &self.md5);

        Ok(())
    }

    pub fn rename(&mut self, new_name: &Path) -> Result<()> {
        fs::rename(&self.path, new_name)
            .change_context(Error::SystemProblem(FSProblem::Rename, self.path.clone()))?;
        self.path = new_name
            .to_str()
            .ok_or(Error::StringConversionError)?
            .to_string();
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &self.path);
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &self.md5,
            md5::compute(&self.path)
        );
        Ok(())
    }

    pub fn overwrite(&mut self) -> Result<&mut Self> {
        #[cfg(feature = "log")]
        trace!("[{}]\tBegging of overwritting phase", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tBegging of overwritting phase", &self.md5);
        let file_to_overwrite = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .change_context(Error::SystemProblem(FSProblem::Opening, self.path.clone()))?;
        let file_size = file_to_overwrite
            .metadata()
            .change_context(Error::SystemProblem(FSProblem::Opening, self.path.clone()))?
            .size();
        let mut overwrited_length: u64 = 0;
        let mut overwritting_buffer = BufWriter::new(file_to_overwrite);

        while overwrited_length < file_size {
            overwrited_length += self.buffer_size as u64;
            if file_size <= overwrited_length {
                let special_buffer_size =
                    file_size as usize + self.buffer_size - overwrited_length as usize;
                overwritting_buffer
                    .write(&self.get_buffer(special_buffer_size))
                    .change_context(Error::SystemProblem(FSProblem::Write, self.path.clone()))?;
                break;
            }
            overwritting_buffer
                .write(&self.get_buffer(self.buffer_size))
                .change_context(Error::SystemProblem(FSProblem::Write, self.path.clone()))?;
        }
        #[cfg(feature = "log")]
        trace!("[{}]\tEnding of overwritting phase", &self.path);
        #[cfg(feature = "secure_log")]
        trace!("[{:x}]\tEnding of overwritting phase", &self.md5);
        Ok(self)
    }

    fn zero_name(&self) -> Result<String> {
        let name = Path::new(&self.path)
            .file_name()
            .ok_or(Error::NoFileName(self.clone().to_owned()))?;
        let new_name = (0..name.len()).map(|_| "0").collect::<String>();
        Ok(new_name)
    }
}

#[cfg(all(
    test,
    feature = "error-stack",
    not(feature = "log"),
    not(feature = "secure_log")
))]
mod ehanced_test {
    use std::fs::File;

    use crate::{Error, Result};
    use error_stack::ResultExt;
    use pretty_assertions::assert_eq;

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
                buffer_size: 4096,
            }
        );
        basic_creation.pattern(&[0x00_u8, 0x00_u8, 0x00_u8]);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: Some([0x00_u8, 0x00_u8, 0x00_u8]),
                buffer_size: 4096,
            }
        );
        basic_creation.byte(&0x00_u8);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: Some(0x00_u8),
                pattern: None,
                buffer_size: 4096,
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
        File::create(file_to_rename_path).change_context(Error::FileCreationError)?;

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
                buffer_size: 4096,
            }
        );
        Ok(())
    }

    #[test]
    fn deletion_test() -> Result<()> {
        let mut file_to_rename = std::env::temp_dir();
        file_to_rename.push("nozomi_deletion_test.txt");
        let file_to_rename_path = file_to_rename.as_path();
        File::create(file_to_rename_path).change_context(Error::FileCreationError)?;
        let mut secure_delete = SecureDelete::new(
            &file_to_rename_path
                .to_str()
                .ok_or(Error::StringConversionError)?,
        )?;
        secure_delete.delete()?;
        let mut file_to_rename = std::env::temp_dir();
        file_to_rename.push("0");
        assert_eq!(
            secure_delete,
            SecureDelete {
                path: file_to_rename
                    .to_str()
                    .ok_or(Error::StringConversionError)?
                    .to_string(),
                pattern: None,
                byte: None,
                buffer_size: 4096,
            }
        );
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
                buffer_size: 4096,
            }
        );
        basic_creation.buffer(1024);
        assert_eq!(
            basic_creation,
            SecureDelete {
                path: "README.md".to_string(),
                byte: None,
                pattern: None,
                buffer_size: 1024,
            }
        );
        Ok(())
    }
}
