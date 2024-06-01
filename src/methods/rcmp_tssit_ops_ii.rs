use crate::models::SecureDelete;
use crate::Method;
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
#[cfg(feature = "log")]
use log::info;
#[cfg(not(feature = "error-stack"))]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    for i in 0..3 {
        secure_deletion
            .byte(&0x00_u8)
            .overwrite()
            .map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, i * 2 + 1))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}][{path}]\t{:2}/7", Method::RcmpTssitOpsII, i * 2 + 1);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{:2}/7",
            Method::RcmpTssitOpsII,
            &secure_deletion.md5,
            i * 2 + 1
        );
        secure_deletion
            .byte(&0xFF_u8)
            .overwrite()
            .map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, i * 2 + 2))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}][{path}]\t{:2}/7", Method::RcmpTssitOpsII, i * 2 + 2);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{:2}/7",
            Method::RcmpTssitOpsII,
            &secure_deletion.md5,
            i * 2 + 2
        );
    }
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t7/7", Method::RcmpTssitOpsII);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t7/7",
        Method::RcmpTssitOpsII,
        &secure_deletion.md5
    );
    Ok(secure_deletion)
}

// * Feature error-stack code base

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "error-stack")]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    for i in 0..3 {
        secure_deletion
            .byte(&0x00_u8)
            .overwrite()
            .change_context(Error::OverwriteError(Method::RcmpTssitOpsII, i * 2 + 1))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}][{path}]\t{:2}/7", Method::RcmpTssitOpsII, i * 2 + 1);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{:2}/7",
            Method::RcmpTssitOpsII,
            &secure_deletion.md5,
            i * 2 + 1
        );
        secure_deletion
            .byte(&0xFF_u8)
            .overwrite()
            .change_context(Error::OverwriteError(Method::RcmpTssitOpsII, i * 2 + 2))?;
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        info!("[{}][{path}]\t{:2}/7", Method::RcmpTssitOpsII, i * 2 + 2);
        #[cfg(all(feature = "log", feature = "secure_log"))]
        info!(
            "[{}][{:x}]\t{:2}/7",
            Method::RcmpTssitOpsII,
            &secure_deletion.md5,
            i * 2 + 2
        );
    }
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
    #[cfg(all(feature = "log", not(feature = "secure_log")))]
    info!("[{}][{path}]\t7/7", Method::RcmpTssitOpsII);
    #[cfg(all(feature = "log", feature = "secure_log"))]
    info!(
        "[{}][{:x}]\t7/7",
        Method::RcmpTssitOpsII,
        &secure_deletion.md5
    );
    Ok(secure_deletion)
}

#[cfg(test)]
mod test {
    const METHOD_NAME: &str = "rcmp_tssit_ops_ii";
    use crate::Method::RcmpTssitOpsII as EraseMethod;

    // ! NO CHANGE BEYOND THIS LINE PLEASE
    use super::overwrite_file;
    use crate::error::FSProblem;
    use crate::tests::TestType;

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

            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

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

            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WrittingError, &METHOD_NAME)?;
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

    #[cfg(feature = "error-stack")]
    mod ehanced {
        use super::*;

        use crate::tests::ehanced::{create_test_file, get_bytes};
        use crate::{Error, Result};

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use error_stack::ResultExt;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

            use super::*;

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

            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

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

            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WrittingError, &METHOD_NAME)?;
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
