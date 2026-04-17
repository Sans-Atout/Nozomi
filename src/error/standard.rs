//! Standard error types (default, non-`error-stack` variant).

use crate::error::FSProblem;
use crate::{Method, models::SecureDelete};

/// A `Result` alias that fixes the error type to [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can be produced by the Nozomi library.
#[derive(Debug)]
pub enum Error {
    /// A filesystem operation failed. Contains the operation category and the
    /// path that was being processed.
    SystemProblem(FSProblem, String),
    /// An overwrite pass failed for the given [`Method`] at the indicated pass number.
    OverwriteError(Method, u32),
    /// The provided path has no filename component (e.g. it is a bare root or
    /// ends in `..`).
    NoFileName(SecureDelete),
    /// A [`Path`](std::path::Path) could not be converted to a valid UTF-8 string.
    StringConversionError,
    /// A required builder parameter was not set. The contained string names the
    /// missing field.
    MissingParameter(&'static str),
    /// A post-overwrite verification read back unexpected bytes at the given
    /// byte offset. Only available with the `verify` feature.
    #[cfg(feature = "verify")]
    VerificationFailed {
        /// Byte offset of the first mismatched byte within the file.
        offset: u64,
    },
    /// Wraps a [`std::io::Error`] produced when creating temporary test files.
    #[cfg(test)]
    FileCreationError(std::io::Error),
}

/// Implementing display trait for Error enum
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::SystemProblem(prb, path) => {
                write!(fmt, "File System Problem [{prb}]\tPath : [{path}]")
            }
            Error::OverwriteError(m, s) => {
                write!(fmt, "Overwrite Error : method [{m}]\tstep : [{s}]")
            }
            Error::NoFileName(_) => {
                write!(fmt, "Given path did not have a proper filename")
            }
            Error::StringConversionError => {
                write!(fmt, "Problem during String conversion process")
            }
            #[cfg(test)]
            Error::FileCreationError(e) => write!(fmt, "{e:?}"),
            Error::MissingParameter(param) => {
                write!(fmt, "RequestDeleter : {param} params missing")
            }
            #[cfg(feature = "verify")]
            Error::VerificationFailed { offset } => {
                write!(fmt, "Verification failed - offset : {offset}")
            }
        }
    }
}

impl core::error::Error for Error {}

#[cfg(test)]
mod test {

    use crate::error::rfc1236;

    #[test]
    fn test_rfc1236() {
        rfc1236::<super::Error>();
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;

        match (self, other) {
            (SystemProblem(a1, b1), SystemProblem(a2, b2)) => a1 == a2 && b1 == b2,
            (OverwriteError(m1, s1), OverwriteError(m2, s2)) => m1 == m2 && s1 == s2,
            (NoFileName(_), NoFileName(_)) => true, // or false, see note below
            (StringConversionError, StringConversionError) => true,
            (MissingParameter(p1), MissingParameter(p2)) => p1 == p2,

            // 👇 IMPORTANT: FileCreationError is never equal
            (FileCreationError(_), FileCreationError(_)) => false,

            _ => false,
        }
    }
}
