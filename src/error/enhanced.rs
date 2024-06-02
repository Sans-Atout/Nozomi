use crate::error::FSProblem;
use crate::{models::SecureDelete, Method};
use error_stack::Context;

/// Reexporting Result type
pub type Result<T> = error_stack::Result<T, Error>;

/// Enum used to represent errors in the library
#[derive(Debug, Clone)]
pub enum Error {
    /// Represent file problems with FSProblem and String
    SystemProblem(FSProblem, String),
    /// Represent an error during a specific overwrite method with Method and step
    OverwriteError(Method, u32),
    /// Represent the fact that we cannot found a file/folder name for a given path
    NoFileName(SecureDelete),
    /// Error during path to string conversion
    StringConversionError,
    /// error to help during debug phase
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
        }
    }
}

impl Context for Error {}
