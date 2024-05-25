use crate::{Method, models::SecureDelete};
#[cfg(not(feature = "error-stack"))]
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SystemProblem(FSProblem, String),
    OverwriteError(Method, u32),
    NoFileName(SecureDelete),
    StringConversionError,
    #[cfg(test)]
    FileCreationError(std::io::Error),
}

#[derive(Debug)]
pub enum FSProblem {
    Rename,
    Opening,
    Write,
    Delete,
    ReadFolder,
    NotFound,
    Permissions
}

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
        }
    }
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
            Error::FileCreationError(e) => write!(fmt, "{e:?}"),
        }
    }
}

#[cfg(not(feature = "error-stack"))]
impl std::error::Error for Error {}
