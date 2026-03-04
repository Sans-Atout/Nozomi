use crate::DeleteEvent;
use crate::engine::events::EventSink;
use crate::engine::utils::{emit_safe, get_three_bytes_pattern_buffer};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
#[cfg(feature = "error-stack")]
use std::path::Path;

use crate::error::FSProblem;
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Report, ResultExt};

#[derive(Debug, Clone)]
pub(crate) enum LastPassInfo {
    Zero,
    ThreeBytesPattern([u8; 3]),
    Pattern(u8),
    Random { seed: [u8; 32] },
}

#[cfg(not(feature = "error-stack"))]
pub(crate) fn verify_last_pass<S: EventSink>(
    path: &PathBuf,
    info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted { path: path.clone() },
    );

    let mut file = File::open(path).map_err(|_| {
        Error::SystemProblem(FSProblem::Opening, format!("{}", path.to_string_lossy()))
    })?;
    file.sync_all().map_err(|_| {
        Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
    })?;
    file.seek(SeekFrom::Start(0)).map_err(|_| {
        Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
    })?;

    let mut buffer = vec![0u8; 8192];

    match info {
        LastPassInfo::Zero => match verify_fixed(&mut file, 0x00, &mut buffer, &path) {
            Ok(_) => {}
            Err(Error::VerificationFailed { offset }) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.clone(),
                        offset,
                    },
                );
                return Err(Error::VerificationFailed { offset });
            }
            Err(e) => return Err(e),
        },
        LastPassInfo::ThreeBytesPattern(pattern) => {
            match verify_three_bytes_pattern(&mut file, &pattern, &mut buffer, &path) {
                Ok(_) => {}
                Err(Error::VerificationFailed { offset }) => {
                    emit_safe(
                        sink,
                        DeleteEvent::VerificationFailed {
                            path: path.clone(),
                            offset,
                        },
                    );
                    return Err(Error::VerificationFailed { offset });
                }
                Err(e) => return Err(e),
            }
        }
        LastPassInfo::Pattern(p) => match verify_fixed(&mut file, p, &mut buffer, &path) {
            Ok(_) => {}
            Err(Error::VerificationFailed { offset }) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.clone(),
                        offset,
                    },
                );
                return Err(Error::VerificationFailed { offset });
            }
            Err(e) => return Err(e),
        },
        LastPassInfo::Random { seed } => match verify_random(&mut file, seed, &mut buffer, &path) {
            Ok(_) => {}
            Err(Error::VerificationFailed { offset }) => {
                emit_safe(
                    sink,
                    DeleteEvent::VerificationFailed {
                        path: path.clone(),
                        offset,
                    },
                );
                return Err(Error::VerificationFailed { offset });
            }
            Err(e) => return Err(e),
        },
    }

    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted { path: path.clone() },
    );

    Ok(())
}

#[cfg(all(not(feature = "error-stack"), feature = "dry-run"))]
pub(crate) fn dry_verify_last_pass<S: EventSink>(
    path: &PathBuf,
    _info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted { path: path.clone() },
    );
    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted { path: path.clone() },
    );
    Ok(())
}

#[cfg(not(feature = "error-stack"))]
fn verify_fixed(file: &mut File, expected: u8, buffer: &mut [u8], path: &PathBuf) -> Result<()> {
    let mut offset: u64 = 0;

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        for (i, &byte) in buffer[..bytes_read].iter().enumerate() {
            if byte != expected {
                return Err(Error::VerificationFailed {
                    offset: offset + i as u64,
                });
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(not(feature = "error-stack"))]
fn verify_random(file: &mut File, seed: [u8; 32], buffer: &mut [u8], path: &PathBuf) -> Result<()> {
    let mut rng = StdRng::from_seed(seed);
    let mut offset: u64 = 0;
    let mut expected = vec![0u8; buffer.len()];

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        rng.fill_bytes(&mut expected[..bytes_read]);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(Error::VerificationFailed {
                    offset: offset + i as u64,
                });
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(not(feature = "error-stack"))]
fn verify_three_bytes_pattern(
    file: &mut File,
    pattern: &[u8; 3],
    buffer: &mut [u8],
    path: &PathBuf,
) -> Result<()> {
    use std::io::Read;

    let mut offset: u64 = 0;

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;
        if bytes_read == 0 {
            break;
        }

        let expected = get_three_bytes_pattern_buffer(&pattern, bytes_read);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(Error::VerificationFailed {
                    offset: offset + i as u64,
                });
            }
        }
        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(test)]
#[cfg(not(feature = "error-stack"))]
mod tests {
    use super::*;
    use crate::Error;
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
            Err(Error::VerificationFailed { offset }) => {
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

#[cfg(feature = "error-stack")]
pub(crate) fn verify_last_pass<S: EventSink>(
    path: &PathBuf,
    info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted { path: path.clone() },
    );

    let mut file = File::open(path).change_context(Error::SystemProblem(
        FSProblem::Opening,
        format!("{}", path.to_string_lossy()),
    ))?;
    file.sync_all().change_context(Error::SystemProblem(
        FSProblem::Read,
        format!("{}", path.to_string_lossy()),
    ))?;
    file.seek(SeekFrom::Start(0))
        .change_context(Error::SystemProblem(
            FSProblem::Read,
            format!("{}", path.to_string_lossy()),
        ))?;

    let mut buffer = vec![0u8; 8192];

    match info {
        LastPassInfo::Zero => match verify_fixed(&mut file, 0x00, &mut buffer, path) {
            Ok(_) => {}
            Err(report) => {
                if let Error::VerificationFailed { offset } = report.current_context() {
                    emit_safe(
                        sink,
                        DeleteEvent::VerificationFailed {
                            path: path.clone(),
                            offset: *offset,
                        },
                    );
                }
                return Err(report);
            }
        },
        LastPassInfo::ThreeBytesPattern(pattern) => {
            match verify_three_bytes_pattern(&mut file, &pattern, &mut buffer, path) {
                Ok(_) => {}
                Err(report) => {
                    if let Error::VerificationFailed { offset } = report.current_context() {
                        emit_safe(
                            sink,
                            DeleteEvent::VerificationFailed {
                                path: path.clone(),
                                offset: *offset,
                            },
                        );
                    }
                    return Err(report);
                }
            }
        }
        LastPassInfo::Pattern(p) => match verify_fixed(&mut file, p, &mut buffer, path) {
            Ok(_) => {}
            Err(report) => {
                if let Error::VerificationFailed { offset } = report.current_context() {
                    emit_safe(
                        sink,
                        DeleteEvent::VerificationFailed {
                            path: path.clone(),
                            offset: *offset,
                        },
                    );
                }
                return Err(report);
            }
        },
        LastPassInfo::Random { seed } => match verify_random(&mut file, seed, &mut buffer, path) {
            Ok(_) => {}
            Err(report) => {
                if let Error::VerificationFailed { offset } = report.current_context() {
                    emit_safe(
                        sink,
                        DeleteEvent::VerificationFailed {
                            path: path.clone(),
                            offset: *offset,
                        },
                    );
                }
                return Err(report);
            }
        },
    }

    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted { path: path.clone() },
    );

    Ok(())
}

#[cfg(all(feature = "error-stack", feature = "dry-run"))]
pub(crate) fn dry_verify_last_pass<S: EventSink>(
    path: &Path,
    _info: LastPassInfo,
    sink: &mut S,
) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::VerificationStarted { path: path.to_path_buf() },
    );
    emit_safe(
        sink,
        DeleteEvent::VerificationCompleted { path: path.to_path_buf() },
    );
    Ok(())
}
#[cfg(feature = "error-stack")]
fn verify_fixed(
    file: &mut std::fs::File,
    expected: u8,
    buffer: &mut [u8],
    path: &Path,
) -> Result<()> {
    let mut offset: u64 = 0;

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        for (i, &byte) in buffer[..bytes_read].iter().enumerate() {
            if byte != expected {
                return Err(Report::new(Error::VerificationFailed {
                    offset: offset + i as u64,
                }));
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(feature = "error-stack")]
fn verify_random(
    file: &mut std::fs::File,
    seed: [u8; 32],
    buffer: &mut [u8],
    path: &Path,
) -> Result<()> {
    let mut rng = StdRng::from_seed(seed);
    let mut offset: u64 = 0;
    let mut expected = vec![0u8; buffer.len()];

    loop {
        let bytes_read = file.read(buffer).map_err(|_| {
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        rng.fill_bytes(&mut expected[..bytes_read]);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(Report::new(Error::VerificationFailed {
                    offset: offset + i as u64,
                }));
            }
        }

        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(feature = "error-stack")]
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
            Error::SystemProblem(FSProblem::Read, format!("{}", path.to_string_lossy()))
        })?;
        if bytes_read == 0 {
            break;
        }

        let expected = get_three_bytes_pattern_buffer(pattern, bytes_read);

        for i in 0..bytes_read {
            if buffer[i] != expected[i] {
                return Err(Report::new(Error::VerificationFailed {
                    offset: offset + i as u64,
                }));
            }
        }
        offset += bytes_read as u64;
    }

    Ok(())
}

#[cfg(test)]
#[cfg(feature = "error-stack")]
mod tests {
    use super::*;
    use crate::Error;
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
            Err(report) => match report.current_context() {
                Error::VerificationFailed { offset } => {
                    assert_eq!(*offset, 512);
                }
                other => panic!("Unexpected error: {:?}", other),
            },
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
