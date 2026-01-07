use std::path::PathBuf;

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use rand::Rng;
#[cfg(not(feature = "error-stack"))]
use crate::{Result,Error};
use crate::error::FSProblem;

#[cfg(feature = "log")]
use log::info;
use crate::engine::overwrite::common::prepare_overwrite;
use crate::Method;

/// Function that implement a basic pseudo random method using basic error handling method.
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&PathBuf) : path that you want to erase using basic pseudo random method overwrite method
///
/// ## Return
/// * `()`
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &PathBuf) -> Result<()> {
	#[cfg(feature = "secure_log")]
	let computed_md5 = md5::compute(format!("{}", path.to_string_lossy()));

	#[cfg(all(feature = "log", not(feature = "secure_log")))]
	info!("[{}][{path}]\t1/1", Method::PseudoRandom);
	#[cfg(all(feature = "log", feature = "secure_log"))]
	info!(
        "[{}][{:x}]\t1/1",
        Method::PseudoRandom,
        computed_md5
    );

	let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;

	let mut remaining = file_size;

	while remaining > 0 {
		rng.fill(&mut buffer);
		let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;
		file.write_all(&buffer[..write_size])
			.map_err(|_| Error::OverwriteError(Method::PseudoRandom, 1))?;
		remaining -= write_size as u64;
	}

	file.flush().map_err(|_| Error::OverwriteError(Method::PseudoRandom, 1))?;
	file.sync_all().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;

	Ok(())

}