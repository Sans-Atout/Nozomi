use error_stack::Context;
use crate::error::FSProblem;
use crate::{Method, models::SecureDelete};

pub type Result <T> = error_stack::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    SystemProblem(FSProblem, String),
    OverwriteError(Method, u32),
    NoFileName(SecureDelete),
    StringConversionError,
    #[cfg(test)]
    FileCreationError,
}

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
                write!(fmt, "Problem during String conversion processus")
            }
            #[cfg(test)]
            Error::FileCreationError => write!(fmt, "File Creation : Error during test file generation process"),
        }
    }
}

impl Context for Error {}