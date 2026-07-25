use crate::error::FSProblem;
use crate::{Error, Method, Result};
use std::fmt::Debug;
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::{fs::File, io::Write, path::Path};

/// Function to test if an type (struct/enum) as send and sync trait
fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}

/// Function to test if an type (struct/enum) as send and sync trait
fn has_needed<T: Clone + Debug>() {}

#[test]
/// This test is used to check whether enums and structures have the various basic traits.
/// If this test fails, the library cannot be compiled
fn validate_structure() {
    // -- Test Type enum
    has_send_sync::<TestType>();
    has_needed::<TestType>();
    // -- File System possible Problem enum
    has_send_sync::<FSProblem>();
    has_needed::<FSProblem>();
    // -- overwrite Method enum
    has_send_sync::<Method>();
    has_needed::<Method>();

    // -- error type
    has_send_sync::<crate::Error>();
}

/// Basic lorem ipsum text used to create file
#[cfg(not(feature = "log"))]
const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent congue diam risus, quis hendrerit nunc commodo id. Duis volutpat vitae leo at malesuada. Phasellus libero nisl, auctor sit amet dapibus eget, egestas eget velit. Curabitur fermentum, libero vel dictum hendrerit, metus leo pellentesque tortor, et vehicula nisl felis elementum purus. Etiam eu ex in odio tincidunt maximus. Ut luctus blandit ligula et dignissim. Duis id imperdiet urna. Sed porttitor nulla vitae sollicitudin sagittis. Donec ut justo ut risus pretium dignissim. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec eu rhoncus erat, sed sagittis elit. Proin dui nisl, varius nec volutpat sed, lobortis nec tellus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n";

/// Enum that describe all tests' type used to test global method
#[derive(Debug, Clone, Copy)]
pub enum TestType {
    /// Test deleting a 1KB file
    #[cfg(not(feature = "log"))]
    SmallFile,
    /// Test deleting a 1MB file
    #[cfg(not(feature = "log"))]
    MediumFile,
    /// Test deleting a 10MB file
    #[cfg(not(feature = "log"))]
    LargeFile,
    /// Test deleting a read only file
    #[cfg(not(feature = "log"))]
    WritingError,
    /// Test deleting a folder containing multiple files
    #[cfg(not(feature = "log"))]
    Folder,
    /// Test the overwriting function
    #[cfg(not(feature = "log"))]
    OverwriteOnly,
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    /// Test deleting a 1KB file with log feature activated
    LogMini,
    #[cfg(feature = "secure_log")]
    /// Test deleting a 1KB file with secure log feature activated
    SecureLog,
}

/// Function for creating files for the various tests
///
/// ## Arguments
/// * `path` (&Path) : Path of the file you want to create
/// * `lorem` (&str) : What needs to be written to the file.
pub fn file(path: &Path, lorem: &str) -> Result<()> {
    let mut file = File::create(path).map_err(Error::FileCreationError)?;
    file.write_all(lorem.as_bytes())
        .map_err(Error::FileCreationError)?;
    Ok(())
}

/// Function that retrieves hexadecimal data from a given file
///
/// ## Arguments
/// * `path` (&Path) : Path of the file from which you wish to retrieve the data
///
/// ## Return
/// * `data` (Vec<u8>) : Hexadecimal data vector
pub fn get_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut created_file = File::open(path).map_err(Error::FileCreationError)?;
    let mut data = vec![];
    created_file
        .read_to_end(&mut data)
        .map_err(Error::FileCreationError)?;
    Ok(data)
}

/// Function can be used to create temporary files for the different types of test in the library
///
/// ## Arguments
/// * `test_type` (&TestType) : Type of test you wish to perform
/// * `method_name` (&str)  : Name of the method you wish to test
///
/// ## Return
/// * (&str) : Path of the test file created
/// * (&str) : Data written to the temporary file specially created for test function
pub fn create_test_file(test_type: &TestType, method_name: &str) -> Result<(String, String)> {
    let mut tmp_file = std::env::temp_dir();
    tmp_file.push(format!("nozomi/{method_name}"));
    if !tmp_file.as_path().exists() {
        create_dir_all(&tmp_file).map_err(Error::FileCreationError)?;
    }
    let test_folder = tmp_file
        .as_path()
        .to_str()
        .ok_or(crate::error::PlanningError::StringConversion)?;
    match test_type {
        #[cfg(not(feature = "log"))]
        TestType::SmallFile => {
            let file_name = format!("{test_folder}/{method_name}_small_file_test");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            file.write_all(lorem.as_bytes())
                .map_err(Error::FileCreationError)?;
            Ok((file_name, lorem))
        }
        #[cfg(not(feature = "log"))]
        TestType::MediumFile => {
            let file_name = format!("{test_folder}/{method_name}_medium_file_test");
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            for _ in 0..1263 {
                file.write(LOREM_IPSUM.as_bytes())
                    .map_err(Error::FileCreationError)?;
            }
            Ok((file_name, LOREM_IPSUM.to_string()))
        }
        #[cfg(not(feature = "log"))]
        TestType::WritingError => {
            let permission_error_file = format!("{test_folder}/permission_error.txt");
            let mut file =
                File::create(&permission_error_file).map_err(Error::FileCreationError)?;
            file.write(LOREM_IPSUM.as_bytes())
                .map_err(Error::FileCreationError)?;
            let metadata = file.metadata().map_err(|_| {
                crate::error::IoError::Filesystem(
                    FSProblem::Permissions,
                    permission_error_file.clone(),
                )
            })?;
            let mut permissions = metadata.permissions();
            permissions.set_readonly(true);
            std::fs::set_permissions(&permission_error_file, permissions).map_err(|_| {
                crate::error::IoError::Filesystem(
                    FSProblem::Permissions,
                    permission_error_file.clone(),
                )
            })?;
            Ok((permission_error_file, LOREM_IPSUM.to_string()))
        }
        #[cfg(not(feature = "log"))]
        TestType::Folder => {
            let folder_to_delete = format!("{test_folder}/folder_to_delete/");
            if !Path::new(&folder_to_delete).exists() {
                create_dir_all(&folder_to_delete).map_err(Error::FileCreationError)?;
            }
            for index in 0..10 {
                let small_folder = format!("{folder_to_delete}to_delete_{index}.txt");
                file(Path::new(&small_folder), LOREM_IPSUM)?;
            }
            Ok((folder_to_delete.to_string(), LOREM_IPSUM.to_string()))
        }
        #[cfg(not(feature = "log"))]
        TestType::OverwriteOnly => {
            let file_name = format!("{test_folder}/{method_name}_basic_over_write");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            file.write_all(lorem.as_bytes())
                .map_err(Error::FileCreationError)?;
            Ok((file_name, lorem))
        }
        #[cfg(not(feature = "log"))]
        TestType::LargeFile => {
            let file_name = format!("{test_folder}/{method_name}_large_file_test");
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            for _ in 0..(1263 * 25) {
                file.write(LOREM_IPSUM.as_bytes())
                    .map_err(Error::FileCreationError)?;
            }
            Ok((file_name, LOREM_IPSUM.to_string()))
        }
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        TestType::LogMini => {
            let file_name = format!("{test_folder}/{method_name}_log_mini.txt");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            file.write_all(lorem.as_bytes())
                .map_err(Error::FileCreationError)?;
            Ok((file_name, lorem))
        }
        #[cfg(feature = "secure_log")]
        TestType::SecureLog => {
            let file_name = format!("{test_folder}/{method_name}_secure_log.txt");
            let lorem = "Hello, world!".to_string();
            let mut file = File::create(&file_name).map_err(Error::FileCreationError)?;
            file.write_all(lorem.as_bytes())
                .map_err(Error::FileCreationError)?;
            Ok((file_name, lorem))
        }
    }
}

mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Test to check that the temporary file creation function is working correctly
    #[test]
    fn create() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_creation_test");
        let path = tmp_file.as_path();
        file(path, "Hello, world!")?;
        assert!(path.exists());
        let mut created_file = File::open(path).map_err(Error::FileCreationError)?;
        let mut created_file_content = String::new();
        created_file
            .read_to_string(&mut created_file_content)
            .map_err(Error::FileCreationError)?;
        assert_eq!(created_file_content, "Hello, world!");
        Ok(())
    }

    /// Test to check whether the get_bytes function works
    #[test]
    fn bytes_assertion() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_byte_assertion");
        let path = tmp_file.as_path();
        file(path, "Hello, world!")?;
        assert!(path.exists());
        assert_eq!(get_bytes(path)?, b"Hello, world!");
        Ok(())
    }
}
