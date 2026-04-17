//! `error-stack`-based error types (deprecated variant).
//!
//! This module is only compiled when the `error-stack` feature is enabled.
//! It mirrors [`super::standard`] but wraps errors in
//! [`error_stack::Report`] for richer context chains.
//!
//! **Deprecated since `3.1.0`.** Use the default error system instead.
//! This module will be removed in `4.0.0`.

use crate::error::FSProblem;
use crate::{Method, models::SecureDelete};
/// A `Result` alias backed by [`error_stack::Report`] for richer error context.
///
/// **Deprecated since `3.1.0`.**
#[deprecated(
    since = "3.1.0",
    note = "This legacy error system will be removed in `4.0.0`. Please use the default Error system instead."
)]
pub type Result<T> = core::result::Result<T, error_stack::Report<Error>>;

/// Errors that can be produced by the Nozomi library (`error-stack` variant).
///
/// **Deprecated since `3.1.0`.** Use the default [`Error`](crate::error::standard::Error) instead.
#[derive(Debug, Clone)]
#[deprecated(
    since = "3.1.0",
    note = "This legacy error system will be removed in `4.0.0`. Please use the default Error system instead."
)]
pub enum Error {
    /// A filesystem operation failed. Contains the operation category and the
    /// path that was being processed.
    SystemProblem(FSProblem, String),
    /// An overwrite pass failed for the given [`Method`](crate::Method) at the
    /// indicated pass number.
    OverwriteError(Method, u32),
    /// The provided path has no filename component.
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
    /// Produced when creating temporary test files fails.
    #[cfg(test)]
    FileCreationError,
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
            Error::FileCreationError => write!(
                fmt,
                "File Creation : Error during test file generation process"
            ),
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

impl Context for Error {}
