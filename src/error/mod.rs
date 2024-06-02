#[cfg(not(feature = "error-stack"))]
pub mod standard;

#[cfg(feature = "error-stack")]
pub mod enhanced;

#[derive(Debug)]
pub enum FSProblem {
    Rename,
    Opening,
    Write,
    Delete,
    ReadFolder,
    NotFound,
    Permissions,
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
