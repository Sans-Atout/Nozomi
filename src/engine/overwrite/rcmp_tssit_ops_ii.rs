use std::path::PathBuf;

use std::io::{Seek, SeekFrom, Write};

use rand::Rng;
#[cfg(not(feature = "error-stack"))]
use crate::{Result,Error};
use crate::error::FSProblem;

#[cfg(feature = "log")]
use log::info;
use crate::engine::overwrite::common::prepare_overwrite;
use crate::Method;

const  FIXED_PATTERNS: &[u8; 6] = &[0x00,0xFF,0x00,0xFF,0x00,0xFF];

/// Function that implement [RCMP TSSIT OPS II overwrite method](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html) using basic error handling method.
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&PathBuf) : path that you want to erase using RCMP TSSIT OPS II overwrite method
///
/// ## Return
/// * `()
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &PathBuf) -> Result<()> {

	let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;
	for pattern in 0..7 {
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