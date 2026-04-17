use crate::engine::overwrite::common::prepare_overwrite;
use crate::{DeleteEvent, EventSink, Method};
use rand::Rng;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use crate::error::FSProblem;
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "verify")]
use crate::engine::utils::generate_seed;
#[cfg(feature = "verify")]
use crate::engine::verify::{LastPassInfo, verify_last_pass};
#[cfg(feature = "verify")]
use rand::SeedableRng;
#[cfg(feature = "verify")]
use rand::rngs::StdRng;

use crate::engine::utils::emit_safe;
#[cfg(all(feature = "verify", feature = "dry-run"))]
use crate::engine::verify::dry_verify_last_pass;

const FIXED_PATTERNS: &[Option<u8>] = &[Some(0x00), Some(0xFF), None];

// -- Region : AFSSI 5020 overwriting method for basic error handling method

/// Overwrites the file at `path` using the
/// [AFSSI 5020](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020)
/// sanitisation standard (3 passes: `0x00`, `0xFF`, random).
///
/// This function overwrites the file contents only; it does **not** delete
/// the file. Deletion is handled by the executor after all passes complete.
///
/// # Errors
///
/// Returns an error if any write pass fails or the file cannot be synced.
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    let (mut file, file_size, mut rng, mut buffer) = prepare_overwrite(path)?;
    #[cfg(feature = "verify")]
    let mut seed = [0u8; 32];

    for (pass, patterns) in FIXED_PATTERNS.iter().enumerate() {
        #[cfg(feature = "verify")]
        if pass == FIXED_PATTERNS.len() - 1 {
            seed = generate_seed();
            rng = StdRng::from_seed(seed);
        }
        // rewind start of file
        file.seek(SeekFrom::Start(0))
            .map_err(|_| Error::OverwriteError(Method::Afssi5020, pass as u32))?;

        let mut remaining = file_size;
        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            match patterns {
                Some(b) => {
                    buffer[..write_size].fill(*b);
                }
                None => {
                    rng.fill_bytes(&mut buffer[..write_size]);
                }
            }

            file.write_all(&buffer[..write_size])
                .map_err(|_| Error::OverwriteError(Method::Afssi5020, pass as u32))?;
            remaining -= write_size as u64;
        }

        file.flush()
            .map_err(|_| Error::OverwriteError(Method::Afssi5020, pass as u32))?;
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: FIXED_PATTERNS.len() as u32,
            },
        );
    }
    file.sync_all().map_err(|_| {
        Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy()))
    })?;

    #[cfg(feature = "verify")]
    verify_last_pass(&path.to_path_buf(), LastPassInfo::Random { seed }, sink)?;

    Ok(())
}

/// Simulates the AFSSI 5020 overwrite of `path` without writing any data.
///
/// Emits the same [`DeleteEvent::EntryOverwritePass`] events as
/// [`overwrite_file`] so that event-driven code behaves identically in dry-run
/// mode. Only available when the `dry-run` feature is enabled.
#[cfg(all(not(feature = "error-stack"), feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    #[cfg(feature = "verify")]
    let seed = [0u8; 32];
    for (pass, _) in FIXED_PATTERNS.iter().enumerate() {
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: FIXED_PATTERNS.len() as u32,
            },
        );
    }
    #[cfg(feature = "verify")]
    dry_verify_last_pass(&path.to_path_buf(), LastPassInfo::Random { seed }, sink)?;

    Ok(())
}

/// Overwrites the file at `path` using the
/// [AFSSI 5020](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020)
/// sanitisation standard (3 passes: `0x00`, `0xFF`, random).
///
/// This function overwrites the file contents only; it does **not** delete
/// the file. Deletion is handled by the executor after all passes complete.
///
/// # Errors
///
/// Returns an error if any write pass fails or the file cannot be synced.
#[cfg(feature = "error-stack")]
pub(crate) fn overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    let (mut file, file_size, mut rng, mut buffer) = prepare_overwrite(path)?;
    #[cfg(feature = "verify")]
    let mut seed = [0u8; 32];

    for (pass, patterns) in FIXED_PATTERNS.iter().enumerate() {
        #[cfg(feature = "verify")]
        if pass == FIXED_PATTERNS.len() - 1 {
            seed = generate_seed();
            rng = StdRng::from_seed(seed);
        }
        // rewind start of file
        file.seek(SeekFrom::Start(0))
            .change_context(Error::OverwriteError(Method::Afssi5020, pass as u32))?;

        let mut remaining = file_size;
        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            match patterns {
                Some(b) => {
                    buffer[..write_size].fill(*b);
                }
                None => {
                    rng.fill_bytes(&mut buffer[..write_size]);
                }
            }

            file.write_all(&buffer[..write_size])
                .change_context(Error::OverwriteError(Method::Afssi5020, pass as u32))?;
            remaining -= write_size as u64;
        }

        file.flush()
            .change_context(Error::OverwriteError(Method::Afssi5020, pass as u32))?;
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: FIXED_PATTERNS.len() as u32,
            },
        );
    }
    file.sync_all().change_context(Error::SystemProblem(
        FSProblem::Write,
        format!("{}", path.to_string_lossy()),
    ))?;
    #[cfg(feature = "verify")]
    verify_last_pass(&path.to_path_buf(), LastPassInfo::Random { seed }, sink)?;
    Ok(())
}

/// Simulates the AFSSI 5020 overwrite of `path` without writing any data.
///
/// Emits the same [`DeleteEvent::EntryOverwritePass`] events as
/// [`overwrite_file`] so that event-driven code behaves identically in dry-run
/// mode. Only available when the `dry-run` feature is enabled.
#[cfg(all(feature = "error-stack", feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    #[cfg(feature = "verify")]
    let seed = [0u8; 32];
    for (pass, _) in FIXED_PATTERNS.iter().enumerate() {
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: FIXED_PATTERNS.len() as u32,
            },
        );
    }
    #[cfg(feature = "verify")]
    dry_verify_last_pass(path, LastPassInfo::Random { seed }, sink)?;
    Ok(())
}

// -- Region : Tests
#[cfg(test)]
mod test {
    use crate::Method::Afssi5020 as EraseMethod;
    const METHOD_NAME: &str = "afssi_5020";

    use crate::tests::TestType;

    /// Module containing all the tests for the standard error handling method
    #[cfg(not(feature = "error-stack"))]
    mod standard {
        use super::*;

        use crate::tests::standard::{create_test_file};
        use crate::Result;

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use super::*;
            use crate::api::delete::request::NoopSink;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;
            use crate::Error;
            use crate::tests::standard::get_bytes;
            use crate::error::FSProblem;

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
                let mut sink = NoopSink;
                crate::engine::overwrite::dod_522022_me::overwrite_file(
                    &path.to_path_buf(),
                    &mut sink,
                )?;
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

        use crate::tests::enhanced::{create_test_file};
        use crate::Result;

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use super::*;
            use crate::api::delete::request::NoopSink;
            use crate::engine::overwrite::dod_522022_me::overwrite_file;
            use error_stack::ResultExt;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;
            use crate::Error;
            use crate::tests::enhanced::get_bytes;
            use crate::error::FSProblem;

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
                let mut sink = NoopSink;
                overwrite_file(&path.to_path_buf(), &mut sink)?;
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
