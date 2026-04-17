//! Error types used throughout the library.
//!
//! Two implementations are provided depending on the active feature flag:
//!
//! - **default** (`standard`) — uses the standard [`core::error::Error`] trait.
//! - **`error-stack`** (`enhanced`) — wraps errors in an [`error_stack::Report`]
//!   for richer context. This variant is deprecated and will be removed in `4.0.0`.
//!
//! [`FSProblem`] is shared between both variants and describes the category of
//! filesystem operation that failed.

#[cfg(not(feature = "error-stack"))]
pub mod standard;

#[cfg(feature = "error-stack")]
#[allow(deprecated)]
pub mod enhanced;

/// Identifies the category of filesystem operation that produced an error.
///
/// Carried inside [`Error::SystemProblem`](crate::Error::SystemProblem) to give
/// callers and logging tooling precise context about what the engine was
/// attempting when the failure occurred.
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
#[cfg(not(feature = "error-stack"))]
pub(crate) fn rfc1236<T: core::error::Error + Send + Sync + 'static>() {}
