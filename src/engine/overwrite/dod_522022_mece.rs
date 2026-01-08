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


const FIXED_PATTERNS: &[Option<u8>] = &[
    Some(0x00),
    Some(0xFF),
    None,
    Some(0x00),
    Some(0x00),
    Some(0xFF),
    None
];

// -- Region : DOD 522 022 MECE overwriting method for basic error handling method

/// Function that implement [DOD 522022 MECE overwrite method](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&str) : path that you want to erase using DOD 522022 MECE overwrite method
///
/// ## Return
/// * `secure_deletion` (SecureDelete) : An SecureDelete object
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &PathBuf) -> Result<()> {
    let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;

    for pass in 0..FIXED_PATTERNS.len() {
        // rewind start of file
        file.seek(SeekFrom::Start(0)).map_err(|_| Error::OverwriteError(Method::Dod522022MECE, pass as u32))?;

        let mut remaining = file_size;
        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            match FIXED_PATTERNS[pass] {
                Some(b) => {
                    buffer[..write_size].fill(b);
                }
                None => {
                    rng.fill(&mut buffer[..write_size]);
                }
            }

            file.write_all(&buffer[..write_size]).map_err(|_| Error::OverwriteError(Method::Dod522022MECE, pass as u32))?;
            remaining -= write_size as u64;
        }

        // flush after each pass (best-effort)
        file.flush().map_err(|_| Error::OverwriteError(Method::Dod522022MECE, pass as u32))?;
    }
    file.sync_all().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;

    Ok(())
}