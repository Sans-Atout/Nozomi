use crate::error::FSProblem;
use crate::{Method, models::SecureDelete};

#[cfg(not(feature = "error-stack"))]
/// Reexporting Result type
pub type Result<T> = core::result::Result<T, Error>;

/// Enum used to represent errors in the library
#[derive(Debug)]
pub enum Error {
    /// Represent file problems with FSProblem and String
    SystemProblem(FSProblem, String),
    /// Represent an error during a specific overwrite method with Method and step
    OverwriteError(Method, u32),
    /// Represent the fact that we cannot found a file/folder name for a given path
    NoFileName(SecureDelete),
    /// Error during path to string conversion
    StringConversionError,
    /// Wrapper for std::io:Error to help during debug phase
    #[cfg(test)]
    FileCreationError(std::io::Error),
    MissingParameter(&'static str),
    #[cfg(feature = "verify")]
    VerificationFailed {
        offset: u64,
    },
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
            },
            #[cfg(feature="verify")]
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
