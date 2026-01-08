use std::path::PathBuf;

use std::io::{Seek, SeekFrom, Write};

#[cfg(not(feature = "error-stack"))]
use crate::{Result,Error};
use crate::error::FSProblem;

#[cfg(feature = "log")]
use log::info;
use crate::engine::overwrite::common::prepare_overwrite;
use crate::Method;

/// Function that implement [HMGI S5 overwrite method](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&PathBuf) : path that you want to erase using HMGI S5 overwrite method
///
/// ## Return
/// * `()`
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &PathBuf) -> Result<()> {
	let (mut file, file_size, _,mut buffer) = prepare_overwrite(path)?;
	for pattern in 0..2 {
		file.seek(SeekFrom::Start(0)).map_err(|_| Error::OverwriteError(Method::HmgiS5, &pattern+1 ))?;

		let mut remaining = file_size;
		while remaining > 0 {
			let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
			buffer[..write_size].fill(0x00);
			file.write_all(&buffer[..write_size])
				.map_err(|_| Error::OverwriteError(Method::HmgiS5, &pattern+1 ))?;
			remaining -= write_size as u64;
		}

		file.flush().map_err(|_| Error::OverwriteError(Method::HmgiS5, &pattern+1 ))?;
	}
	file.sync_all().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;
	Ok(())
}