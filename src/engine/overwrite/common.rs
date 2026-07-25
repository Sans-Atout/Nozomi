use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::Result;
use crate::error::{FSProblem, IoError};
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::engine::utils::generate_seed;

/// Opens the file at `path` for reading and writing, queries its size, rewinds
/// the cursor to the start, and returns a fresh CSPRNG seeded with random
/// entropy alongside a zeroed 8 KiB write buffer.
///
/// This setup is shared by all overwrite method implementations.
///
/// # Errors
///
/// Returns an error if the file cannot be opened, its metadata cannot be read,
/// or the cursor cannot be rewound.
pub(crate) fn prepare_overwrite(path: &Path) -> Result<(std::fs::File, u64, StdRng, [u8; 8192])> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|_| {
            IoError::Filesystem(FSProblem::Permissions, path.to_string_lossy().to_string())
        })?;

    let file_size = file
        .metadata()
        .map_err(|_| IoError::Filesystem(FSProblem::Opening, path.to_string_lossy().to_string()))?
        .len();

    file.seek(SeekFrom::Start(0))
        .map_err(|_| IoError::Filesystem(FSProblem::Write, path.to_string_lossy().to_string()))?;

    let seed = generate_seed();
    let rng = StdRng::from_seed(seed);
    let buffer = [0u8; 8192];

    Ok((file, file_size, rng, buffer))
}
