//! Error types used throughout the library.
//!
//! Errors are organised into three categories mirroring the phases of a
//! deletion:
//!
//! - [`PlanningError`] — invalid input, detected before any destructive
//!   operation starts (e.g. a missing builder parameter).
//! - [`ExecutionError`] — a failure during the overwrite or verification
//!   passes themselves.
//! - [`IoError`] — a raw filesystem operation failed.
//!
//! [`FSProblem`] describes the category of filesystem operation that failed
//! and is carried inside [`IoError::Filesystem`].

use crate::Method;
use std::fmt;

/// A `Result` alias that fixes the error type to [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can be produced by the Nozomi library.
#[derive(Debug)]
pub enum Error {
    /// Invalid input was detected before any destructive operation started.
    Planning(PlanningError),
    /// A failure occurred during the overwrite or verification passes.
    Execution(ExecutionError),
    /// A raw filesystem operation failed.
    Io(IoError),
    /// Wraps a [`std::io::Error`] produced when creating temporary test files.
    #[cfg(test)]
    FileCreationError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Planning(e) => write!(fmt, "{e}"),
            Error::Execution(e) => write!(fmt, "{e}"),
            Error::Io(e) => write!(fmt, "{e}"),
            #[cfg(test)]
            Error::FileCreationError(e) => write!(fmt, "{e:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PlanningError> for Error {
    fn from(e: PlanningError) -> Self {
        Error::Planning(e)
    }
}

impl From<ExecutionError> for Error {
    fn from(e: ExecutionError) -> Self {
        Error::Execution(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;

        match (self, other) {
            (Planning(a), Planning(b)) => a == b,
            (Execution(a), Execution(b)) => a == b,
            (Io(a), Io(b)) => a == b,

            // 👇 IMPORTANT: FileCreationError is never equal
            (FileCreationError(_), FileCreationError(_)) => false,

            _ => false,
        }
    }
}

/// Errors raised while validating input, before any destructive operation
/// has started.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PlanningError {
    /// A required builder parameter was not set. The contained string names
    /// the missing field.
    MissingParameter(&'static str),
    /// A [`Path`](std::path::Path) could not be converted to a valid UTF-8
    /// string.
    StringConversion,
}

impl fmt::Display for PlanningError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanningError::MissingParameter(param) => {
                write!(fmt, "RequestDeleter : {param} params missing")
            }
            PlanningError::StringConversion => {
                write!(fmt, "Problem during String conversion process")
            }
        }
    }
}

impl std::error::Error for PlanningError {}

/// Errors raised while executing the overwrite or verification passes of a
/// deletion.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ExecutionError {
    /// An overwrite pass failed for the given [`Method`] at the indicated
    /// pass number.
    OverwritePass {
        /// The overwrite method being executed.
        method: Method,
        /// The 1-indexed pass number that failed.
        pass: u32,
    },
    /// A post-overwrite verification read back unexpected bytes at the given
    /// byte offset. Only available with the `verify` feature.
    #[cfg(feature = "verify")]
    VerificationFailed {
        /// Byte offset of the first mismatched byte within the file.
        offset: u64,
    },
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::OverwritePass { method, pass } => {
                write!(fmt, "Overwrite Error : method [{method}]\tstep : [{pass}]")
            }
            #[cfg(feature = "verify")]
            ExecutionError::VerificationFailed { offset } => {
                write!(fmt, "Verification failed - offset : {offset}")
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

/// Raw filesystem operation failures.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum IoError {
    /// A filesystem operation failed. Contains the operation category and
    /// the path that was being processed.
    Filesystem(FSProblem, String),
    /// The provided path has no filename component (e.g. it is a bare root
    /// or ends in `..`).
    NoFileName(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::Filesystem(prb, path) => {
                write!(fmt, "File System Problem [{prb}]\tPath : [{path}]")
            }
            IoError::NoFileName(_) => {
                write!(fmt, "Given path did not have a proper filename")
            }
        }
    }
}

impl std::error::Error for IoError {}

/// Identifies the category of filesystem operation that produced an error.
///
/// Carried inside [`IoError::Filesystem`] to give callers and logging
/// tooling precise context about what the engine was attempting when the
/// failure occurred.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FSProblem {
    /// Failure while renaming a file or directory.
    Rename,
    /// Failure while opening a file for reading or writing.
    Opening,
    /// Failure while writing data to a file.
    Write,
    /// Failure while reading data from a file (requires the `verify` feature).
    #[cfg(feature = "verify")]
    Read,
    /// Failure while removing a file or directory from the filesystem.
    Delete,
    /// Failure while enumerating the entries of a directory.
    ReadFolder,
    /// The requested file or directory does not exist.
    NotFound,
    /// Failure while querying or modifying filesystem permissions.
    Permissions,
}

/// Implementing display trait for FSProblem enum
impl core::fmt::Display for FSProblem {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            FSProblem::Rename => write!(fmt, "Rename"),
            FSProblem::Opening => write!(fmt, "Opening"),
            FSProblem::Write => write!(fmt, "writing"),
            FSProblem::Delete => write!(fmt, "Delete"),
            FSProblem::ReadFolder => write!(fmt, "Read Folder"),
            FSProblem::NotFound => write!(fmt, "File/Folder not found"),
            FSProblem::Permissions => write!(fmt, "Change permission error"),
            #[cfg(feature = "verify")]
            FSProblem::Read => write!(fmt, "Cannot read buffer in file during verify stage"),
        }
    }
}

#[cfg(test)]
pub(crate) fn rfc1236<T: core::error::Error + Send + Sync + 'static>() {}

#[cfg(test)]
mod test {
    use super::{Error, ExecutionError, IoError, PlanningError, rfc1236};

    #[test]
    fn test_rfc1236() {
        rfc1236::<Error>();
        rfc1236::<PlanningError>();
        rfc1236::<ExecutionError>();
        rfc1236::<IoError>();
    }
}
