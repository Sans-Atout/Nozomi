use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::error::FSProblem;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

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
#[cfg(not(feature = "error-stack"))]
pub(crate) fn prepare_overwrite(path: &Path) -> Result<(std::fs::File, u64, StdRng, [u8; 8192])> {
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
            Error::SystemProblem(FSProblem::Opening, format!("{}", path.to_string_lossy()))
        })?
        .len();

    file.seek(SeekFrom::Start(0)).map_err(|_| {
        Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy()))
    })?;

    let seed = generate_seed();
    let rng = StdRng::from_seed(seed);
    let buffer = [0u8; 8192];

    Ok((file, file_size, rng, buffer))
}

/// Opens the file at `path` for reading and writing, queries its size, rewinds
/// the cursor to the start, and returns a fresh CSPRNG seeded with random
/// entropy alongside a zeroed 8 KiB write buffer.
///
/// This setup is shared by all overwrite method implementations.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if the
/// file cannot be opened, its metadata cannot be read, or the cursor cannot be
/// rewound.
#[cfg(feature = "error-stack")]
pub(crate) fn prepare_overwrite(path: &Path) -> Result<(std::fs::File, u64, StdRng, [u8; 8192])> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .change_context(Error::SystemProblem(
            FSProblem::Permissions,
            format!("{}", path.to_string_lossy()),
        ))?;

    let file_size = file
        .metadata()
        .change_context(Error::SystemProblem(
            FSProblem::Opening,
            format!("{}", path.to_string_lossy()),
        ))?
        .len();

    file.seek(SeekFrom::Start(0))
        .change_context(Error::SystemProblem(
            FSProblem::Write,
            format!("{}", path.to_string_lossy()),
        ))?;

    let seed = generate_seed();
    let rng = StdRng::from_seed(seed);
    let buffer = [0u8; 8192];

    Ok((file, file_size, rng, buffer))
}
