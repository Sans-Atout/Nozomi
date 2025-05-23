use crate::Method;
use crate::models::SecureDelete;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "log")]
use log::info;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

// -- Region : HMGI S5 overwriting method for basic error handling method

/// Function that implement [HMGI S5 overwrite method](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&str) : path that you want to erase using HMGI S5 overwrite method
///
/// ## Return
/// * `secure_deletion` (SecureDelete) : An SecureDelete object
#[cfg(not(feature = "error-stack"))]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::HmgiS5, 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t1/2", Method::HmgiS5);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t1/2", Method::HmgiS5, &secure_deletion.md5);
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::HmgiS5, 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t2/2", Method::HmgiS5);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t2/2", Method::HmgiS5, &secure_deletion.md5);

    Ok(secure_deletion)
}

// -- Region : HMGI S5 overwriting method for error-stack error handling method

/// Function that implement [HMGI S5 overwrite method](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&str) : path that you want to erase using HMGI S5 overwrite method
///
/// ## Return
/// * `secure_deletion` (SecureDelete) : An SecureDelete object
#[cfg(feature = "error-stack")]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .change_context(Error::OverwriteError(Method::HmgiS5, 1))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t1/2", Method::HmgiS5);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t1/2", Method::HmgiS5, &secure_deletion.md5);
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .change_context(Error::OverwriteError(Method::HmgiS5, 2))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t2/2", Method::HmgiS5);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!("[{}][{:x}]\t2/2", Method::HmgiS5, &secure_deletion.md5);

    Ok(secure_deletion)
}

// -- Region : Tests
#[cfg(test)]
mod test {
    const METHOD_NAME: &str = "hmgi_S5";
    use crate::Method::HmgiS5 as EraseMethod;

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
