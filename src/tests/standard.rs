use crate::error::standard::FSProblem;
use crate::{Error, Result};
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::{fs::File, io::Write, path::Path};
use super::TestType;
use super::LOREM_IPSUM;

#[cfg(not(feature="error-stack"))]
pub fn file(path: &Path, lorem: &str) -> Result<()> {
    let mut file = File::create(path).map_err(|e| Error::FileCreationError(e))?;
    file.write_all(lorem.as_bytes())
        .map_err(|e| Error::FileCreationError(e))?;
    Ok(())
}

#[cfg(feature="error-stack")]
pub fn file(path: &Path, lorem: &str) -> Result<()> {
    let mut file = File::create(path).change_context(Error::FileCreationError(std::io::Error))?;
    file.write_all(lorem.as_bytes())
        .change_context(Error::FileCreationError(e))?;
    Ok(())
}

#[cfg(not(feature="error-stack"))]
pub fn create_test_file(test_type: &TestType, method_name: &str) -> Result<(String, String)> {

    let mut tmp_file = std::env::temp_dir();
    tmp_file.push(method_name);
    if !tmp_file.as_path().exists() {
        create_dir_all(&tmp_file).map_err(|e| Error::FileCreationError(e))?;
    }
    let test_folder = tmp_file
        .as_path()
        .to_str()
        .ok_or(Error::StringConversionError)?;
    match test_type {
        TestType::SmallFile => {
            let file_name = format!("{test_folder}/{method_name}_small_file_test");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(|e| Error::FileCreationError(e))?;
            file.write_all(lorem.as_bytes())
                .map_err(|e| Error::FileCreationError(e))?;
            return Ok((file_name, lorem));
        }
        TestType::MediumFile => {
            let file_name = format!("{test_folder}/{method_name}_medium_file_test");
            let mut file = File::create(&file_name).map_err(|e| Error::FileCreationError(e))?;
            for _ in 0..1263 {
                file.write(LOREM_IPSUM.as_bytes())
                    .map_err(|e| Error::FileCreationError(e))?;
            }
            return Ok((file_name, LOREM_IPSUM.to_string()));
        }
        TestType::WrittingError => {
            let permission_error_file = format!("{test_folder}/permission_error.txt");
            let file =File::create(&permission_error_file).map_err(|e| Error::FileCreationError(e))?;
            let metadata = file.metadata().map_err(|_| Error::SystemProblem(FSProblem::Permissions, permission_error_file.clone()))?;
            let mut permissions = metadata.permissions();
            permissions.set_readonly(true);
            return Ok((permission_error_file, String::default()));
        }
        TestType::Folder => {
            let folder_to_delete = format!("{test_folder}/folder_to_delete/");
            if !Path::new(&folder_to_delete).exists() {
                create_dir_all(&folder_to_delete).map_err(|e| Error::FileCreationError(e))?;
            }
            for index in 0..10 {
                let small_folder = format!("{folder_to_delete}to_delete_{index}.txt");
                file(Path::new(&small_folder), LOREM_IPSUM)?;
            }
            return Ok((folder_to_delete.to_string(), LOREM_IPSUM.to_string()));
        }
        TestType::OverwriteOnly => {
            let file_name = format!("{test_folder}/{method_name}_basic_over_write");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(|e| Error::FileCreationError(e))?;
            file.write_all(lorem.as_bytes())
                .map_err(|e| Error::FileCreationError(e))?;
            return Ok((file_name, lorem));
        }
        TestType::LargeFile => {
            let file_name = format!("{test_folder}/{method_name}_large_file_test");
            let mut file = File::create(&file_name).map_err(|e| Error::FileCreationError(e))?;
            for _ in 0..(1263 * 25) {
                file.write(LOREM_IPSUM.as_bytes())
                    .map_err(|e| Error::FileCreationError(e))?;
            }
            return Ok((file_name, LOREM_IPSUM.to_string()));
        }
    }
}

#[cfg(not(feature="error-stack"))]
pub fn get_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut created_file = File::open(path).map_err(|e| Error::FileCreationError(e))?;
    let mut data = vec![];
    created_file
        .read_to_end(&mut data)
        .map_err(|e| Error::FileCreationError(e))?;
    Ok(data)
}

#[cfg(not(feature="error-stack"))]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_creation_test");
        let path = tmp_file.as_path();
        file(path, "Hello, world!")?;
        assert!(path.exists());
        let mut created_file = File::open(path).map_err(|e| Error::FileCreationError(e))?;
        let mut created_file_content = String::new();
        created_file
            .read_to_string(&mut created_file_content)
            .map_err(|e| Error::FileCreationError(e))?;
        assert_eq!(created_file_content, "Hello, world!");
        Ok(())
    }

    #[test]
    fn bytes_assertion() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_byte_assertion");
        let path = tmp_file.as_path();
        file(&path, "Hello, world!")?;
        assert!(path.exists());
        assert_eq!(get_bytes(&path)?, b"Hello, world!");
        Ok(())
    }
}
