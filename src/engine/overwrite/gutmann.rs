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

use crate::engine::utils::emit_safe;

#[cfg(feature = "verify")]
use crate::engine::utils::generate_seed;
#[cfg(all(feature = "verify", feature = "dry-run"))]
use crate::engine::verify::dry_verify_last_pass;
#[cfg(feature = "verify")]
use crate::engine::verify::{LastPassInfo, verify_last_pass};
#[cfg(feature = "verify")]
use rand::SeedableRng;
#[cfg(feature = "verify")]
use rand::rngs::StdRng;
// 3-byte fixed patterns (27 passes)
const FIXED_PATTERNS: &[[u8; 3]] = &[
    [0x55, 0x55, 0x55],
    [0xAA, 0xAA, 0xAA],
    [0x92, 0x49, 0x24],
    [0x49, 0x24, 0x92],
    [0x24, 0x92, 0x49],
    [0x00, 0x00, 0x00],
    [0x11, 0x11, 0x11],
    [0x22, 0x22, 0x22],
    [0x33, 0x33, 0x33],
    [0x44, 0x44, 0x44],
    [0x55, 0x55, 0x55],
    [0x66, 0x66, 0x66],
    [0x77, 0x77, 0x77],
    [0x88, 0x88, 0x88],
    [0x99, 0x99, 0x99],
    [0xAA, 0xAA, 0xAA],
    [0xBB, 0xBB, 0xBB],
    [0xCC, 0xCC, 0xCC],
    [0xDD, 0xDD, 0xDD],
    [0xEE, 0xEE, 0xEE],
    [0xFF, 0xFF, 0xFF],
    [0x92, 0x49, 0x24],
    [0x49, 0x24, 0x92],
    [0x24, 0x92, 0x49],
    [0x6D, 0xB6, 0xDB],
    [0xB6, 0xDB, 0x6D],
    [0xDB, 0x6D, 0xB6],
];

/// Overwrites the file at `path` using the
/// [Gutmann](https://en.wikipedia.org/wiki/Gutmann_method) sanitisation
/// standard (35 passes: 4 random, 27 fixed 3-byte patterns, 4 random).
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

    // Total passes = 35
    for pass in 0..35 {
        #[cfg(feature = "verify")]
        if pass == 34 {
            seed = generate_seed();
            rng = StdRng::from_seed(seed);
        }

        file.seek(SeekFrom::Start(0))
            .map_err(|_| Error::OverwriteError(Method::Gutmann, pass as u32))?;
        let mut remaining = file_size;

        let is_random = !(4..31).contains(&pass);

        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            if is_random {
                rng.fill_bytes(&mut buffer[..write_size]);
            } else {
                let pattern = FIXED_PATTERNS[pass - 4];
                for i in 0..write_size {
                    buffer[i] = pattern[i % 3];
                }
            }

            file.write_all(&buffer[..write_size])
                .map_err(|_| Error::OverwriteError(Method::Gutmann, pass as u32))?;
            remaining -= write_size as u64;
        }

        file.flush().map_err(|_| {
            Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy()))
        })?;
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: 35,
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

/// Simulates the Gutmann overwrite of `path` without writing any data.
///
/// Emits the same [`DeleteEvent::EntryOverwritePass`] events as [`overwrite_file`].
/// Only available when the `dry-run` feature is enabled.
#[cfg(all(not(feature = "error-stack"), feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    #[cfg(feature = "verify")]
    let seed = [0u8; 32];
    for pass in 0..35 {
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: 35,
            },
        );
    }
    #[cfg(feature = "verify")]
    dry_verify_last_pass(&path.to_path_buf(), LastPassInfo::Random { seed }, sink)?;

    Ok(())
}

/// Overwrites the file at `path` using the
/// [Gutmann](https://en.wikipedia.org/wiki/Gutmann_method) sanitisation
/// standard (35 passes: 4 random, 27 fixed 3-byte patterns, 4 random).
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

    // Total passes = 35
    for pass in 0..35 {
        #[cfg(feature = "verify")]
        if pass == 34 {
            seed = generate_seed();
            rng = StdRng::from_seed(seed);
        }

        file.seek(SeekFrom::Start(0))
            .change_context(Error::OverwriteError(Method::Gutmann, pass as u32))?;
        let mut remaining = file_size;

        let is_random = !(4..31).contains(&pass);

        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            if is_random {
                rng.fill_bytes(&mut buffer[..write_size]);
            } else {
                let pattern = FIXED_PATTERNS[pass - 4];
                for i in 0..write_size {
                    buffer[i] = pattern[i % 3];
                }
            }

            file.write_all(&buffer[..write_size])
                .change_context(Error::OverwriteError(Method::Gutmann, pass as u32))?;
            remaining -= write_size as u64;
        }

        file.flush().change_context(Error::SystemProblem(
            FSProblem::Write,
            format!("{}", path.to_string_lossy()),
        ))?;
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: 35,
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

/// Simulates the Gutmann overwrite of `path` without writing any data.
///
/// Emits the same [`DeleteEvent::EntryOverwritePass`] events as [`overwrite_file`].
/// Only available when the `dry-run` feature is enabled.
#[cfg(all(feature = "error-stack", feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(path: &Path, sink: &mut S) -> Result<()> {
    #[cfg(feature = "verify")]
    let seed = [0u8; 32];
    for pass in 0..35 {
        emit_safe(
            sink,
            DeleteEvent::EntryOverwritePass {
                path: path.to_path_buf(),
                pass: pass as u32 + 1,
                total_passes: 35,
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
    const METHOD_NAME: &str = "gutmann";
    use crate::Method::Gutmann as EraseMethod;

    use crate::tests::TestType;

    /// Module containing all the tests for the standard error handling method
    #[cfg(not(feature = "error-stack"))]
    mod standard {
        use super::*;

        use crate::Result;
        use crate::tests::standard::create_test_file;

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use super::*;
            use crate::Error;
            use crate::api::delete::request::NoopSink;
            use crate::error::FSProblem;
            use crate::tests::standard::get_bytes;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

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
                let (string_path, _) = create_test_file(&TestType::SecureLog, METHOD_NAME)?;
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

        use crate::Result;
        use crate::tests::enhanced::create_test_file;

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use super::*;
            use crate::Error;
            use crate::api::delete::request::NoopSink;
            use crate::engine::overwrite::dod_522022_me::overwrite_file;
            use crate::error::FSProblem;
            use crate::tests::enhanced::get_bytes;
            use error_stack::ResultExt;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

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
                let (string_path, _) = create_test_file(&TestType::SecureLog, METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }
    }
}
