use std::path::Path;
use std::io::{Seek, SeekFrom, Write};
use rand::Rng;
use crate::engine::overwrite::common::prepare_overwrite;
use crate::Method;

#[cfg(not(feature = "error-stack"))]
use crate::{Result,Error};
use crate::error::FSProblem;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "log")]
use log::info;

const  FIXED_PATTERNS: &[u8; 6] = &[0x00,0xFF,0x00,0xFF,0x00,0xFF];

/// Function that implement [RCMP TSSIT OPS II overwrite method](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html) using basic error handling method.
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&Path) : path that you want to erase using RCMP TSSIT OPS II overwrite method
///
/// ## Return
/// * `()
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &Path) -> Result<()> {

	let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;
	for pattern in 0..6 {
		// rewind start of file
		file.seek(SeekFrom::Start(0)).map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;

		let mut remaining = file_size;
		while remaining > 0 {
			let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
			buffer[..write_size].fill(FIXED_PATTERNS[pattern as usize]);
			file.write_all(&buffer[..write_size])
				.map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;
			remaining -= write_size as u64;
		}

		// flush after each pass (best-effort)
		file.flush().map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;
	}
	let mut remaining = file_size;
	file.seek(SeekFrom::Start(0)).map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, 7 ))?;

	while remaining > 0 {
		rng.fill(&mut buffer);
		let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
		file.write_all(&buffer[..write_size])
			.map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
		remaining -= write_size as u64;
	}

	file.flush()
		.map_err(|_| Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
	file.sync_all().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;

	Ok(())
}


/// Function that implement [RCMP TSSIT OPS II overwrite method](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html) using basic error handling method.
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&Path) : path that you want to erase using RCMP TSSIT OPS II overwrite method
///
/// ## Return
/// * `()
#[cfg(feature = "error-stack")]
pub(crate) fn overwrite_file(path: &Path) -> Result<()> {

	let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;
	for pattern in 0..6 {
		// rewind start of filels
		file.seek(SeekFrom::Start(0)).change_context(Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;

		let mut remaining = file_size;
		while remaining > 0 {
			let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
			buffer[..write_size].fill(FIXED_PATTERNS[pattern as usize]);
			file.write_all(&buffer[..write_size])
				.change_context(Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;
			remaining -= write_size as u64;
		}

		// flush after each pass (best-effort)
		file.flush().change_context(Error::OverwriteError(Method::RcmpTssitOpsII, &pattern+1 ))?;
	}
	let mut remaining = file_size;
	file.seek(SeekFrom::Start(0)).change_context(Error::OverwriteError(Method::RcmpTssitOpsII, 7 ))?;

	while remaining > 0 {
		rng.fill(&mut buffer);
		let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
		file.write_all(&buffer[..write_size])
			.change_context(Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
		remaining -= write_size as u64;
	}

	file.flush()
		.change_context(Error::OverwriteError(Method::RcmpTssitOpsII, 7))?;
	file.sync_all().change_context(Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;

	Ok(())
}


// -- Region : Tests
#[cfg(test)]
mod test {
	const METHOD_NAME: &str = "rcmp_tssit_ops_ii";
	use crate::Method::RcmpTssitOpsII as EraseMethod;

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
				overwrite_file(&path.to_path_buf())?;
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
				overwrite_file(&path.to_path_buf())?;
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
