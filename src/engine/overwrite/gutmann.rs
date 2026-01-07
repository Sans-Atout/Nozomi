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

/// Function that implement [Gutmann overwrite method](https://en.wikipedia.org/wiki/Gutmann_method)
/// ! Please note that this method does not delete the given file.
///
/// ## Argument :
/// * `path` (&PathBuf) : path that you want to erase using basic pseudo random method overwrite method
///
/// ## Return
/// * `()`
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(path: &PathBuf) -> crate::Result<()> {
    let (mut file, file_size, mut rng,mut buffer) = prepare_overwrite(path)?;

    // Total passes = 35
    for pass in 0..35 {
        file.seek(SeekFrom::Start(0)).map_err(|_| Error::OverwriteError(Method::Gutmann, pass as u32))?;
        let mut remaining = file_size;

        let is_random = pass < 4 || pass >= 31;

        while remaining > 0 {
            let write_size = std::cmp::min(remaining, buffer.len() as u64) as usize;

            if is_random {
                rng.fill(&mut buffer[..write_size]);
            } else {
                let pattern = FIXED_PATTERNS[pass - 4];
                for i in 0..write_size {
                    buffer[i] = pattern[i % 3];
                }
            }

            file.write_all(&buffer[..write_size]).map_err(|_| Error::OverwriteError(Method::Gutmann, pass as u32))?;
            remaining -= write_size as u64;
        }

        file.flush().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;
    }

    file.sync_all().map_err(|_| Error::SystemProblem(FSProblem::Write, format!("{}", path.to_string_lossy())))?;
    Ok(())
}
