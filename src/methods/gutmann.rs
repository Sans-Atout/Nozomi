/// The overwrite pattern for gutmann overwriting method
const OVERWRITE_PATTERN: [[u8; 3]; 27] = [
    [0x55_u8, 0x55_u8, 0x55_u8],
    [0xAA_u8, 0xAA_u8, 0xAA_u8],
    [0x92_u8, 0x49_u8, 0x24_u8],
    [0x49_u8, 0x24_u8, 0x92_u8],
    [0x24_u8, 0x92_u8, 0x49_u8],
    [0x00_u8, 0x00_u8, 0x00_u8],
    [0x11_u8, 0x11_u8, 0x11_u8],
    [0x22_u8, 0x22_u8, 0x22_u8],
    [0x33_u8, 0x33_u8, 0x33_u8],
    [0x44_u8, 0x44_u8, 0x44_u8],
    [0x55_u8, 0x55_u8, 0x55_u8],
    [0x66_u8, 0x66_u8, 0x66_u8],
    [0x77_u8, 0x77_u8, 0x77_u8],
    [0x88_u8, 0x88_u8, 0x88_u8],
    [0x99_u8, 0x99_u8, 0x99_u8],
    [0xAA_u8, 0xAA_u8, 0xAA_u8],
    [0xBB_u8, 0xBB_u8, 0xBB_u8],
    [0xCC_u8, 0xCC_u8, 0xCC_u8],
    [0xDD_u8, 0xDD_u8, 0xDD_u8],
    [0xEE_u8, 0xEE_u8, 0xEE_u8],
    [0xFF_u8, 0xFF_u8, 0xFF_u8],
    [0x92_u8, 0x49_u8, 0x24_u8],
    [0x49_u8, 0x24_u8, 0x92_u8],
    [0x24_u8, 0x92_u8, 0x49_u8],
    [0x6D_u8, 0xB6_u8, 0xDB_u8],
    [0xB6_u8, 0xDB_u8, 0x6D_u8],
    [0xDB_u8, 0x6D_u8, 0xB6_u8],
];

use crate::models::SecureDelete;
use crate::Method;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "log")]
use log::info;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

// -- Region : Gutmann overwriting method for basic error handling method

/// Function that implement [Gutmann overwrite method](https://en.wikipedia.org/wiki/Gutmann_method)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&str) : path that you want to erase using Gutmann overwrite method
///
/// ## Return
/// * `secure_deletion` (SecureDelete) : An SecureDelete object
#[cfg(not(feature = "error-stack"))]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;

    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 1/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 1/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 2/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 2/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 3))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 3/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 3/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 4))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 3/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 4/35", Method::Gutmann, &secure_deletion.md5);
    let mut step = 4;
    for pattern in &OVERWRITE_PATTERN {
        step += 1;
        secure_deletion
            .pattern(pattern)
            .overwrite()
            .map_err(|_| Error::OverwriteError(Method::Gutmann, step))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}] [{path}]\t{step:2}/35", Method::Gutmann);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{step:2}/35",
            Method::Gutmann,
            &secure_deletion.md5
        );
    }
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 1);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 1
    );
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 2);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 2
    );
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 3))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 3);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 3
    );
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 4))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 4);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 4
    );
    Ok(secure_deletion)
}

// -- Region : Gutmann overwriting method for error-stack error handling method

/// Function that implement [Gutmann overwrite method](https://en.wikipedia.org/wiki/Gutmann_method)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&str) : path that you want to erase using Gutmann overwrite method
///
/// ## Return
/// * `secure_deletion` (SecureDelete) : An SecureDelete object
#[cfg(feature = "error-stack")]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 1/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 1/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 2/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 2/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, 3))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 3/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 3/35", Method::Gutmann, &secure_deletion.md5);
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, 4))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t 3/35", Method::Gutmann);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t 4/35", Method::Gutmann, &secure_deletion.md5);

    let mut step = 4;
    for pattern in &OVERWRITE_PATTERN {
        step += 1;
        secure_deletion
            .pattern(pattern)
            .overwrite()
            .change_context(Error::OverwriteError(Method::Gutmann, step))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}] [{path}]\t{step:2}/35", Method::Gutmann);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{step:2}/35",
            Method::Gutmann,
            &secure_deletion.md5
        );
    }
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, step + 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 1);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 1
    );
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, step + 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 2);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 2
    );
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, step + 3))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 3);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 3
    );
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Gutmann, step + 4))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}] [{path}]\t{:2}/35", Method::Gutmann, step + 4);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t{:2}/35",
        Method::Gutmann,
        &secure_deletion.md5,
        step + 4
    );
    Ok(secure_deletion)
}

// -- Region : Tests 
#[cfg(test)]
mod test {
    const METHOD_NAME: &str = "gutmann";
    use crate::Method::Gutmann as EraseMethod;

    use super::overwrite_file;
    use crate::error::FSProblem;
    use crate::tests::TestType;

    /// Module containing all the tests for the standard error handling method
    #[cfg(not(feature = "error-stack"))]
    mod standard {
        use super::*;

        use crate::tests::standard::{create_test_file, get_bytes};
        use crate::{Error, Result};

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

            use super::*;

            /// Test if the overwrite method for this particular erase protocol work well or not.
            ///
            /// Test success is all conditions are met :
            /// * function overwrite_file is success
            /// * file is overwritten
            /// * file is overwritten with good method
            /// * file is well deleted
            #[test]
            fn basic_overwrite() -> Result<()> {
                let (string_path, lorem) =
                    create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                overwrite_file(&string_path)?;
                let bytes = get_bytes(&path)?;
                assert_eq!(bytes.len(), lorem.as_bytes().len());
                assert_ne!(bytes, lorem.as_bytes());
                std::fs::remove_file(&string_path).map_err(|_| {
                    Error::SystemProblem(FSProblem::Delete, string_path.to_string())
                })?;
                Ok(())
            }

            /// This test checks whether a 1KB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether a 1MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

            /// This test checks whether a 10MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            #[ignore = "test too long"]
            fn large_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LargeFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether an error is returned when a file is read-only and a user tries to delete it using a particular method..
            ///
            /// Test success is all conditions are met :
            /// * A readonly file is created
            /// * An error is returned
            /// * The file is deleted at the end of the test
            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// The test can be used to check whether a folder can be deleted using a particular method.
            ///
            /// Test success is all conditions are met :
            /// * a specific folder with multiple files in it is created
            /// * folder is delete thanks to the specific erasing method
            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WritingError, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                let result = EraseMethod.delete(&string_path);
                println!("{:?}", result);
                assert!(result.is_err());
                let mut perms = path.metadata().unwrap().permissions();
                perms.set_readonly(false);
                std::fs::set_permissions(&string_path, perms).map_err(|_| {
                    Error::SystemProblem(FSProblem::Permissions, string_path.clone())
                })?;
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        mod log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature log functions correctly for basic error handling.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LogMini, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(feature = "secure_log")]
        mod secure_log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature secure_log functions correctly for basic error handling.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SecureLog, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }
    }

    /// Module containing all the tests for the error-stack handling method
    #[cfg(feature = "error-stack")]
    mod enhanced {
        use super::*;

        use crate::tests::enhanced::{create_test_file, get_bytes};
        use crate::{Error, Result};

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use error_stack::ResultExt;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

            use super::*;

            /// Test if the overwrite method for this particular erase protocol work well or not.
            ///
            /// Test success is all conditions are met :
            /// * function overwrite_file is success
            /// * file is overwritten
            /// * file is overwritten with good method
            /// * file is well deleted
            #[test]
            fn basic_overwrite() -> Result<()> {
                let (string_path, lorem) =
                    create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                overwrite_file(&string_path)?;
                let bytes = get_bytes(&path)?;
                assert_eq!(bytes.len(), lorem.as_bytes().len());
                assert_ne!(bytes, lorem.as_bytes());
                std::fs::remove_file(&string_path).change_context(Error::SystemProblem(
                    FSProblem::Delete,
                    string_path.to_string(),
                ))?;
                Ok(())
            }

            /// This test checks whether a 1KB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether a 1MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

            /// This test checks whether a 10MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            #[ignore = "test too long"]
            fn large_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LargeFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// The test can be used to check whether a folder can be deleted using a particular method.
            ///
            /// Test success is all conditions are met :
            /// * a specific folder with multiple files in it is created
            /// * folder is delete thanks to the specific erasing method
            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether an error is returned when a file is read-only and a user tries to delete it using a particular method..
            ///
            /// Test success is all conditions are met :
            /// * A readonly file is created
            /// * An error is returned
            /// * The file is deleted at the end of the test
            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WritingError, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                let result = EraseMethod.delete(&string_path);
                println!("{:?}", result);
                assert!(result.is_err());
                let mut perms = path.metadata().unwrap().permissions();
                perms.set_readonly(false);
                std::fs::set_permissions(&string_path, perms).change_context(
                    Error::SystemProblem(FSProblem::Permissions, string_path.clone()),
                )?;
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        mod log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature log functions correctly
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LogMini, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(feature = "secure_log")]
        mod secure_log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature secure_log functions correctly.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SecureLog, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }
    }
}
