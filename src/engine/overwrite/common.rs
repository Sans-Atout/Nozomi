use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use rand::rngs::ThreadRng;

#[cfg(not(feature = "error-stack"))]
use crate::{Result,Error};
use crate::error::FSProblem;

pub(crate) fn prepare_overwrite(
	path: &Path,
) -> Result<(std::fs::File, u64, ThreadRng, [u8; 8192])> {
	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.open(path)
		.map_err(|_| {
			Error::SystemProblem(
				FSProblem::Permissions,
				format!("{}", path.to_string_lossy()),
			)
		})?;

	let file_size = file
		.metadata()
		.map_err(|_| {
			Error::SystemProblem(
				FSProblem::Opening,
				format!("{}", path.to_string_lossy()),
			)
		})?
		.len();

	file.seek(SeekFrom::Start(0)).map_err(|_| {
		Error::SystemProblem(
			FSProblem::Write,
			format!("{}", path.to_string_lossy()),
		)
	})?;

	let rng = rand::rng();
	let buffer = [0u8; 8192];

	Ok((file, file_size, rng, buffer))
}
