use crate::DeleteEvent;
use crate::engine::events::EventSink;
use crate::engine::utils::{emit_safe, get_three_bytes_pattern_buffer};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::Result;
use crate::error::{Error, ExecutionError, FSProblem, IoError};

/// Describes the expected byte content of the last overwrite pass so that
/// [`verify_last_pass`] can reconstruct and compare the expected data.
#[derive(Debug, Clone)]
pub(crate) enum LastPassInfo {
    /// The last pass filled the file with `0x00` bytes.
    Zero,
    /// The last pass repeated the given 3-byte pattern cyclically.
    ///
    /// No built-in `Method` currently ends on a pattern pass, so this is
    /// unused until user-defined overwrite methods land (see `OverwriteMethod` trait).
    #[allow(dead_code)]
    ThreeBytesPattern([u8; 3]),
    /// The last pass filled the file with a single repeated byte value.
    ///
    /// No built-in `Method` currently ends on a single-byte pass, so this is
    /// unused until user-defined overwrite methods land (see `OverwriteMethod` trait).
    #[allow(dead_code)]
    Pattern(u8),
    /// The last pass filled the file with pseudo-random data seeded by `seed`.
    Random {
        /// The seed used to initialise the PRNG for this pass.
        seed: [u8; 32],
    },
}

/// Reads back the file at `path` and verifies that every byte matches the
/// pattern described by `info`.
///
/// Emits [`DeleteEvent::VerificationStarted`] before reading and either
/// [`DeleteEvent::VerificationCompleted`] on success or
/// [`DeleteEvent::VerificationFailed`] on mismatch.
///
/// # Errors
///
/// Returns [`ExecutionError::VerificationFailed`] if a byte mismatch is
/// detected, or an I/O error if the file cannot be opened or read.
pub(crate) fn verify_last_pass<S: EventSink>(
    path: &Path,
    info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted {
            path: path.to_path_buf(),
        },
    );

    let mut file = File::open(path)
        .map_err(|_| IoError::Filesystem(FSProblem::Opening, path.to_string_lossy().to_string()))?;
    file.sync_all()
        .map_err(|_| IoError::Filesystem(FSProblem::Read, path.to_string_lossy().to_string()))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|_| IoError::Filesystem(FSProblem::Read, path.to_string_lossy().to_string()))?;

    let mut buffer = vec![0u8; 8192];

    match info {
        LastPassInfo::Zero => match verify_fixed(&mut file, 0x00, &mut buffer, path) {
            Ok(_) => {}
            Err(Error::Execution(ExecutionError::VerificationFailed { offset })) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.to_path_buf(),
                        offset,
                    },
                );
                return Err(ExecutionError::VerificationFailed { offset }.into());
            }
            Err(e) => return Err(e),
        },
        LastPassInfo::ThreeBytesPattern(pattern) => {
            match verify_three_bytes_pattern(&mut file, &pattern, &mut buffer, path) {
                Ok(_) => {}
                Err(Error::Execution(ExecutionError::VerificationFailed { offset })) => {
                    emit_safe(
                        sink,
                        DeleteEvent::VerificationFailed {
                            path: path.to_path_buf(),
                            offset,
                        },
                    );
                    return Err(ExecutionError::VerificationFailed { offset }.into());
                }
                Err(e) => return Err(e),
            }
        }
        LastPassInfo::Pattern(p) => match verify_fixed(&mut file, p, &mut buffer, path) {
            Ok(_) => {}
            Err(Error::Execution(ExecutionError::VerificationFailed { offset })) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.to_path_buf(),
                        offset,
                    },
                );
                return Err(ExecutionError::VerificationFailed { offset }.into());
            }
            Err(e) => return Err(e),
        },
        LastPassInfo::Random { seed } => match verify_random(&mut file, seed, &mut buffer, path) {
            Ok(_) => {}
            Err(Error::Execution(ExecutionError::VerificationFailed { offset })) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.to_path_buf(),
                        offset,
                    },
                );
                return Err(ExecutionError::VerificationFailed { offset }.into());
            }
            Err(e) => return Err(e),
        },
    }

    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted {
            path: path.to_path_buf(),
        },
    );

    Ok(())
}

/// Simulates a last-pass verification by emitting the expected events without
/// reading the file.
///
/// Used during dry-run executions to preserve the event sequence seen by
/// [`EventSink`] implementations without performing actual I/O.
#[cfg(feature = "dry-run")]
pub(crate) fn dry_verify_last_pass<S: EventSink>(
    path: &Path,
    _info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted {
            path: path.to_path_buf(),
        },
    );
    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted {
            path: path.to_path_buf(),
        },
    );
    Ok(())
}

fn verify_fixed(file: &mut File, expected: u8, buffer: &mut [u8], path: &Path) -> Result<()> {
    let mut offset: u64 = 0;

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            IoError::Filesystem(FSProblem::Read, path.to_string_lossy().to_string())
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        for (i, &byte) in buffer[..bytes_read].iter().enumerate() {
            if byte != expected {
                return Err(ExecutionError::VerificationFailed {
                    offset: offset + i as u64,
                }
                .into());
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

fn verify_random(file: &mut File, seed: [u8; 32], buffer: &mut [u8], path: &Path) -> Result<()> {
    let mut rng = StdRng::from_seed(seed);
    let mut offset: u64 = 0;
    let mut expected = vec![0u8; buffer.len()];

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            IoError::Filesystem(FSProblem::Read, path.to_string_lossy().to_string())
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        rng.fill_bytes(&mut expected[..bytes_read]);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(ExecutionError::VerificationFailed {
                    offset: offset + i as u64,
                }
                .into());
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

fn verify_three_bytes_pattern(
    file: &mut File,
    pattern: &[u8; 3],
    buffer: &mut [u8],
    path: &Path,
) -> Result<()> {
    use std::io::Read;

    let mut offset: u64 = 0;

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            IoError::Filesystem(FSProblem::Read, path.to_string_lossy().to_string())
        })?;
        if bytes_read == 0 {
            break;
        }

        let expected = get_three_bytes_pattern_buffer(pattern, bytes_read);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(ExecutionError::VerificationFailed {
                    offset: offset + i as u64,
                }
                .into());
            }
        }
        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{Error, ExecutionError};
    use pretty_assertions::assert_eq;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::fs::{File, OpenOptions};
    use std::io::{Seek, SeekFrom, Write};
    use std::path::PathBuf;

    #[test]
    fn verify_fixed_ok() {
        let path = PathBuf::from("test_fixed_ok.tmp");

        {
            let mut file = File::create(&path).unwrap();
            file.write_all(&vec![0x00; 1024]).unwrap();
            file.sync_all().unwrap();
        }

        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 256];

        verify_fixed(&mut file, 0x00, &mut buffer, &path).expect("Verification should succeed");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn verify_random_detects_corruption() {
        let path = PathBuf::from("test_random_fail.tmp");

        let seed = [42u8; 32];

        {
            let mut file = File::create(&path).unwrap();
            let mut rng = StdRng::from_seed(seed);

            let mut buffer = vec![0u8; 1024];
            rng.fill_bytes(&mut buffer);

            file.write_all(&buffer).unwrap();
            file.sync_all().unwrap();
        }

        {
            let mut file = OpenOptions::new().write(true).open(&path).unwrap();

            file.seek(SeekFrom::Start(512)).unwrap();
            file.write_all(&[0xFF]).unwrap();
            file.sync_all().unwrap();
        }

        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 256];

        let result = verify_random(&mut file, seed, &mut buffer, &path);

        match result {
            Err(Error::Execution(ExecutionError::VerificationFailed { offset })) => {
                assert_eq!(offset, 512);
            }
            _ => panic!("Verification should have failed"),
        }

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn verify_random_ok() {
        let path = PathBuf::from("test_random_ok.tmp");
        let seed = [7u8; 32];

        {
            let mut file = File::create(&path).unwrap();
            let mut rng = StdRng::from_seed(seed);

            let mut buffer = vec![0u8; 2048];
            rng.fill_bytes(&mut buffer);

            file.write_all(&buffer).unwrap();
            file.sync_all().unwrap();
        }

        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 512];

        verify_random(&mut file, seed, &mut buffer, &path).expect("Verification should succeed");

        std::fs::remove_file(path).unwrap();
    }
}
